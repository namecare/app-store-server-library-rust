//! AWS-LC backend implementation using aws-lc-sys directly for X509 parsing.

use aws_lc_rs::signature::{EcdsaKeyPair, ECDSA_P256_SHA256_ASN1_SIGNING};

use crate::chain_verifier::{ChainVerificationFailureReason, ChainVerifier, ChainVerifierError};
use crate::crypto::{ChainVerifierFactory, CryptoProvider, PromotionalOfferSignerFactory};
use crate::promotional_offer_signature_creator::{PromotionalOfferSignatureCreatorError, PromotionalOfferSigner};

struct AwsLcPromotionalOfferSigner {
    key_pair: EcdsaKeyPair,
}

impl PromotionalOfferSigner for AwsLcPromotionalOfferSigner {
    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, PromotionalOfferSignatureCreatorError> {
        let rng = aws_lc_rs::rand::SystemRandom::new();
        let sig = self
            .key_pair
            .sign(&rng, message)
            .map_err(|e| PromotionalOfferSignatureCreatorError::SigningError(e.to_string()))?;
        Ok(sig.as_ref().to_vec())
    }
}

fn new_promotional_offer_signer(
    private_key_pem: &str,
) -> Result<Box<dyn PromotionalOfferSigner>, PromotionalOfferSignatureCreatorError> {
    let der =
        decode_pem(private_key_pem).map_err(|e| PromotionalOfferSignatureCreatorError::KeyError(e.to_string()))?;

    let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_ASN1_SIGNING, &der)
        .map_err(|e| PromotionalOfferSignatureCreatorError::KeyError(e.to_string()))?;

    Ok(Box::new(AwsLcPromotionalOfferSigner { key_pair }))
}

/// Simple PEM decoder - extracts base64 content between BEGIN/END markers
fn decode_pem(pem: &str) -> Result<Vec<u8>, &'static str> {
    use base64::prelude::*;

    let lines: Vec<&str> = pem.lines().collect();

    // Find BEGIN and END markers
    let start = lines
        .iter()
        .position(|l| l.starts_with("-----BEGIN"))
        .ok_or("missing BEGIN marker")?;
    let end = lines
        .iter()
        .position(|l| l.starts_with("-----END"))
        .ok_or("missing END marker")?;

    if end <= start + 1 {
        return Err("no content between markers");
    }

    // Concatenate base64 lines
    let b64: String = lines[start + 1..end].concat();

    BASE64_STANDARD
        .decode(&b64)
        .map_err(|_| "invalid base64")
}

mod x509 {
    use aws_lc_sys::{
        d2i_X509, i2d_X509_PUBKEY, ASN1_TIME_to_posix, OBJ_get0_data, OBJ_length, OPENSSL_free,
        X509_EXTENSION_get_object, X509_free, X509_get0_notAfter, X509_get0_notBefore, X509_get0_pubkey,
        X509_get_X509_PUBKEY, X509_get_ext, X509_get_ext_count, X509_verify, EVP_PKEY, X509,
    };
    use std::os::raw::c_long;
    use std::ptr::null_mut;
    use std::slice;

    // Apple extension OIDs (raw DER-encoded bytes)
    // 1.2.840.113635.100.6.11.1
    const APPLE_LEAF_OID: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x63, 0x64, 0x06, 0x0B, 0x01];
    // 1.2.840.113635.100.6.2.1
    const APPLE_INTERMEDIATE_OID: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x63, 0x64, 0x06, 0x02, 0x01];

    /// RAII wrapper for X509*
    pub struct X509Cert {
        ptr: *mut X509,
    }

    impl X509Cert {
        /// Parse DER-encoded certificate
        pub fn from_der(der: &[u8]) -> Result<Self, &'static str> {
            let mut ptr = der.as_ptr();
            let cert = unsafe { d2i_X509(null_mut(), &mut ptr, der.len() as c_long) };
            if cert.is_null() {
                return Err("failed to parse X509 certificate");
            }
            Ok(Self { ptr: cert })
        }

        /// Get public key (borrowed, don't free)
        pub fn pubkey(&self) -> Result<*mut EVP_PKEY, &'static str> {
            let pkey = unsafe { X509_get0_pubkey(self.ptr) };
            if pkey.is_null() {
                return Err("failed to get public key");
            }
            Ok(pkey)
        }

        /// Verify this certificate's signature using issuer's public key
        pub fn verify(&self, issuer_pubkey: *mut EVP_PKEY) -> bool {
            unsafe { X509_verify(self.ptr, issuer_pubkey) == 1 }
        }

        /// Get validity not_before as Unix timestamp
        pub fn not_before(&self) -> Result<i64, &'static str> {
            let t = unsafe { X509_get0_notBefore(self.ptr) };
            if t.is_null() {
                return Err("failed to get notBefore");
            }
            let mut posix: i64 = 0;
            if unsafe { ASN1_TIME_to_posix(t, &mut posix) } != 1 {
                return Err("failed to convert notBefore to posix");
            }
            Ok(posix)
        }

        /// Get validity not_after as Unix timestamp
        pub fn not_after(&self) -> Result<i64, &'static str> {
            let t = unsafe { X509_get0_notAfter(self.ptr) };
            if t.is_null() {
                return Err("failed to get notAfter");
            }
            let mut posix: i64 = 0;
            if unsafe { ASN1_TIME_to_posix(t, &mut posix) } != 1 {
                return Err("failed to convert notAfter to posix");
            }
            Ok(posix)
        }

        /// Check if certificate has Apple leaf extension OID
        pub fn has_apple_leaf_oid(&self) -> bool {
            self.has_extension_oid(APPLE_LEAF_OID)
        }

        /// Check if certificate has Apple intermediate extension OID
        pub fn has_apple_intermediate_oid(&self) -> bool {
            self.has_extension_oid(APPLE_INTERMEDIATE_OID)
        }

        fn has_extension_oid(&self, target_oid: &[u8]) -> bool {
            let ext_count = unsafe { X509_get_ext_count(self.ptr) };
            for i in 0..ext_count {
                let ext = unsafe { X509_get_ext(self.ptr, i) };
                if ext.is_null() {
                    continue;
                }
                let obj = unsafe { X509_EXTENSION_get_object(ext) };
                if obj.is_null() {
                    continue;
                }
                let oid_data = unsafe { OBJ_get0_data(obj) };
                let oid_len = unsafe { OBJ_length(obj) };
                if oid_data.is_null() || oid_len == 0 {
                    continue;
                }
                let oid_bytes = unsafe { slice::from_raw_parts(oid_data, oid_len) };
                if oid_bytes == target_oid {
                    return true;
                }
            }
            false
        }

        /// Get SPKI (SubjectPublicKeyInfo) as DER bytes
        pub fn spki_der(&self) -> Result<Vec<u8>, &'static str> {
            let pubkey = unsafe { X509_get_X509_PUBKEY(self.ptr) };
            if pubkey.is_null() {
                return Err("failed to get X509_PUBKEY");
            }

            let mut out: *mut u8 = null_mut();
            let len = unsafe { i2d_X509_PUBKEY(pubkey, &mut out) };
            if len <= 0 || out.is_null() {
                return Err("failed to encode SPKI");
            }

            let bytes = unsafe { slice::from_raw_parts(out, len as usize) }.to_vec();
            unsafe { OPENSSL_free(out as *mut _) };
            Ok(bytes)
        }
    }

    impl Drop for X509Cert {
        fn drop(&mut self) {
            if !self.ptr.is_null() {
                unsafe { X509_free(self.ptr) };
            }
        }
    }

    // Safety: X509 operations we use are thread-safe (read-only after parsing)
    unsafe impl Send for X509Cert {}
    unsafe impl Sync for X509Cert {}
}

// ============================================================================
// Chain Verifier
// ============================================================================

struct AwsLcChainVerifier;

impl ChainVerifier for AwsLcChainVerifier {
    fn verify(
        &self,
        leaf_certificate: &[u8],
        intermediate_certificate: &[u8],
        root_certificates: &[Vec<u8>],
        effective_date: Option<u64>,
    ) -> Result<Vec<u8>, ChainVerifierError> {
        use ChainVerificationFailureReason::*;

        if root_certificates.is_empty() {
            return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
        }

        // Parse leaf certificate
        let leaf = x509::X509Cert::from_der(leaf_certificate)
            .map_err(|_| ChainVerifierError::VerificationFailure(InvalidCertificate))?;

        // Check Apple-specific leaf OID
        if !leaf.has_apple_leaf_oid() {
            return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
        }

        // Parse intermediate certificate
        let intermediate = x509::X509Cert::from_der(intermediate_certificate)
            .map_err(|_| ChainVerifierError::VerificationFailure(InvalidCertificate))?;

        // Check Apple-specific intermediate OID
        if !intermediate.has_apple_intermediate_oid() {
            return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
        }

        // Find matching root certificate that signed the intermediate
        let mut found_root = false;
        for cert_bytes in root_certificates {
            let root = match x509::X509Cert::from_der(cert_bytes) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let root_pubkey = match root.pubkey() {
                Ok(pk) => pk,
                Err(_) => continue,
            };

            if intermediate.verify(root_pubkey) {
                found_root = true;

                // Check root validity if effective_date provided
                if let Some(date) = effective_date {
                    let timestamp = i64::try_from(date)
                        .map_err(|_| ChainVerifierError::VerificationFailure(InvalidEffectiveDate))?;
                    let not_before = root
                        .not_before()
                        .map_err(|_| ChainVerifierError::VerificationFailure(InvalidCertificate))?;
                    let not_after = root
                        .not_after()
                        .map_err(|_| ChainVerifierError::VerificationFailure(InvalidCertificate))?;
                    if timestamp < not_before || timestamp > not_after {
                        return Err(ChainVerifierError::VerificationFailure(CertificateExpired));
                    }
                }
                break;
            }
        }

        if !found_root {
            return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
        }

        // Verify leaf signature using intermediate's public key
        let intermediate_pubkey = intermediate
            .pubkey()
            .map_err(|_| ChainVerifierError::VerificationFailure(InvalidCertificate))?;

        if !leaf.verify(intermediate_pubkey) {
            return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
        }

        // Check validity dates if effective_date provided
        if let Some(date) = effective_date {
            let timestamp =
                i64::try_from(date).map_err(|_| ChainVerifierError::VerificationFailure(InvalidEffectiveDate))?;

            let leaf_not_before = leaf
                .not_before()
                .map_err(|_| ChainVerifierError::VerificationFailure(InvalidCertificate))?;
            let leaf_not_after = leaf
                .not_after()
                .map_err(|_| ChainVerifierError::VerificationFailure(InvalidCertificate))?;
            let int_not_before = intermediate
                .not_before()
                .map_err(|_| ChainVerifierError::VerificationFailure(InvalidCertificate))?;
            let int_not_after = intermediate
                .not_after()
                .map_err(|_| ChainVerifierError::VerificationFailure(InvalidCertificate))?;

            if timestamp < leaf_not_before
                || timestamp > leaf_not_after
                || timestamp < int_not_before
                || timestamp > int_not_after
            {
                return Err(ChainVerifierError::VerificationFailure(CertificateExpired));
            }
        }

        // Return leaf's SPKI bytes
        leaf.spki_der()
            .map_err(|_| ChainVerifierError::VerificationFailure(InvalidCertificate))
    }
}

fn new_chain_verifier() -> Box<dyn ChainVerifier> {
    Box::new(AwsLcChainVerifier)
}

pub static DEFAULT_PROVIDER: CryptoProvider = CryptoProvider {
    chain_verifier: new_chain_verifier as ChainVerifierFactory,
    promotional_offer_signer: new_promotional_offer_signer as PromotionalOfferSignerFactory,
};
