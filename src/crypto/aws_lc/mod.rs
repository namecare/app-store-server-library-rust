//! AWS-LC backend implementation using aws-lc-sys directly for X509 parsing.

use aws_lc_rs::signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING};

use crate::crypto::{
    CryptoError, CryptoProvider, P256PrivateKey, P256PublicKey, P256Signature, P256SigningSuite,
    Sha1Hasher, VerifiedChain, X509Suite,
};

/// Marker type implementing every capability this backend provides.
#[derive(Debug)]
struct AwsLc;

impl Sha1Hasher for AwsLc {
    fn hash(&self, data: &[u8]) -> [u8; 20] {
        let digest = aws_lc_rs::digest::digest(
            &aws_lc_rs::digest::SHA1_FOR_LEGACY_USE_ONLY,
            data,
        );
        let mut out = [0u8; 20];
        out.copy_from_slice(digest.as_ref());
        out
    }
}

#[derive(Debug)]
struct AwsLcP256PrivateKey {
    key_pair: EcdsaKeyPair,
}

impl P256PrivateKey for AwsLcP256PrivateKey {
    fn signature(&self, message: &[u8]) -> Result<Box<dyn P256Signature>, CryptoError> {
        let rng = aws_lc_rs::rand::SystemRandom::new();
        let sig = self
            .key_pair
            .sign(&rng, message)
            .map_err(|e| CryptoError::SigningError(e.to_string()))?;

        // The key pair was loaded with FIXED_SIGNING, so this is already r‖s.
        let raw: [u8; 64] = sig
            .as_ref()
            .try_into()
            .map_err(|_| CryptoError::SigningError("unexpected signature length".into()))?;

        Ok(Box::new(AwsLcP256Signature { raw }))
    }
}

#[derive(Debug)]
struct AwsLcP256PublicKey {
    spki_der: Vec<u8>,
}

impl P256PublicKey for AwsLcP256PublicKey {
    fn is_valid_signature(
        &self,
        signature: &dyn P256Signature,
        message: &[u8],
    ) -> Result<(), CryptoError> {
        use aws_lc_rs::signature::{UnparsedPublicKey, ECDSA_P256_SHA256_FIXED};

        // SPKI DER wraps the SEC1 point; the last 65 bytes are the uncompressed
        // point aws-lc expects.
        if self.spki_der.len() < 65 {
            return Err(CryptoError::KeyError("SPKI too short".into()));
        }
        let point = &self.spki_der[self.spki_der.len() - 65..];

        UnparsedPublicKey::new(&ECDSA_P256_SHA256_FIXED, point)
            .verify(message, &signature.raw_representation())
            .map_err(|e| CryptoError::VerificationError(e.to_string()))
    }
}

#[derive(Debug)]
struct AwsLcP256Signature {
    raw: [u8; 64],
}

impl P256Signature for AwsLcP256Signature {
    fn raw_representation(&self) -> [u8; 64] {
        self.raw
    }

    fn der_representation(&self) -> Result<Vec<u8>, CryptoError> {
        x509::ecdsa_raw_to_der(&self.raw)
            .map_err(|e| CryptoError::SigningError(e.to_string()))
    }
}

impl P256SigningSuite for AwsLc {
    fn private_key(&self, pem: &str) -> Result<Box<dyn P256PrivateKey>, CryptoError> {
        let der = decode_pem(pem).map_err(|e| CryptoError::KeyError(e.to_string()))?;

        // FIXED_SIGNING so `sign` yields r‖s directly — no conversion needed.
        let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &der)
            .map_err(|e| CryptoError::KeyError(e.to_string()))?;

        Ok(Box::new(AwsLcP256PrivateKey { key_pair }))
    }

    fn public_key(&self, spki_der: &[u8]) -> Result<Box<dyn P256PublicKey>, CryptoError> {
        Ok(Box::new(AwsLcP256PublicKey { spki_der: spki_der.to_vec() }))
    }

    fn signature_from_raw(&self, rs: &[u8; 64]) -> Result<Box<dyn P256Signature>, CryptoError> {
        Ok(Box::new(AwsLcP256Signature { raw: *rs }))
    }
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
    use aws_lc_sys::{
        BN_bin2bn, BN_free, ECDSA_SIG_free, ECDSA_SIG_new, ECDSA_SIG_set0, ECDSA_SIG_to_bytes,
    };
    use std::os::raw::c_long;
    use std::ptr::null_mut;
    use std::slice;

    /// Converts a dotted OID string ("1.2.840.113635.100.6.11.1") to the raw
    /// DER content bytes used by `OBJ_get0_data`.
    ///
    /// `pub(super)` so the encoding test in the parent module can reach it.
    pub(super) fn oid_to_der_bytes(oid: &str) -> Option<Vec<u8>> {
        let parts: Vec<u64> = oid.split('.').map(|p| p.parse().ok()).collect::<Option<_>>()?;
        if parts.len() < 2 {
            return None;
        }
        let mut out = vec![(parts[0] * 40 + parts[1]) as u8];
        for &part in &parts[2..] {
            let mut stack = Vec::new();
            let mut v = part;
            stack.push((v & 0x7f) as u8);
            v >>= 7;
            while v > 0 {
                stack.push(((v & 0x7f) as u8) | 0x80);
                v >>= 7;
            }
            stack.reverse();
            out.extend_from_slice(&stack);
        }
        Some(out)
    }

    /// Converts a fixed-width `r‖s` ECDSA signature to DER.
    ///
    /// Uses aws-lc's own `ECDSA_SIG` encoder rather than hand-written ASN.1.
    pub fn ecdsa_raw_to_der(rs: &[u8; 64]) -> Result<Vec<u8>, &'static str> {
        unsafe {
            let r = BN_bin2bn(rs[..32].as_ptr(), 32, null_mut());
            if r.is_null() {
                return Err("failed to parse r");
            }
            let s = BN_bin2bn(rs[32..].as_ptr(), 32, null_mut());
            if s.is_null() {
                BN_free(r);
                return Err("failed to parse s");
            }

            let sig = ECDSA_SIG_new();
            if sig.is_null() {
                BN_free(r);
                BN_free(s);
                return Err("failed to allocate ECDSA_SIG");
            }

            // set0 takes ownership of r and s on success.
            if ECDSA_SIG_set0(sig, r, s) != 1 {
                BN_free(r);
                BN_free(s);
                ECDSA_SIG_free(sig);
                return Err("failed to set ECDSA_SIG components");
            }

            let mut out: *mut u8 = null_mut();
            let mut out_len: usize = 0;
            let ok = ECDSA_SIG_to_bytes(&mut out, &mut out_len, sig);
            ECDSA_SIG_free(sig);

            if ok != 1 || out.is_null() {
                return Err("failed to encode ECDSA_SIG");
            }

            let der = slice::from_raw_parts(out, out_len).to_vec();
            OPENSSL_free(out as *mut _);
            Ok(der)
        }
    }

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

        /// Whether this certificate carries an extension with the given dotted OID.
        pub fn has_oid(&self, oid: &str) -> bool {
            match oid_to_der_bytes(oid) {
                Some(bytes) => self.has_extension_oid(&bytes),
                None => false,
            }
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

#[cfg(test)]
mod oid_tests {
    #[test]
    fn dotted_oid_converts_to_der_bytes() {
        // 1.2.840.113635.100.6.11.1 — Apple's receipt-signing OID.
        let expected: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x63, 0x64, 0x06, 0x0B, 0x01];
        assert_eq!(
            super::x509::oid_to_der_bytes("1.2.840.113635.100.6.11.1").unwrap(),
            expected
        );

        // 1.2.840.113635.100.6.2.1 — Apple's WWDR intermediate OID.
        let expected: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x63, 0x64, 0x06, 0x02, 0x01];
        assert_eq!(
            super::x509::oid_to_der_bytes("1.2.840.113635.100.6.2.1").unwrap(),
            expected
        );
    }
}

#[cfg(test)]
mod signature_tests {
    use crate::crypto::CryptoProvider;

    #[test]
    fn raw_is_64_bytes_and_der_is_well_formed() {
        let pem = include_str!("../../../tests/resources/certs/testSigningKey.p8");
        let provider = CryptoProvider::default_provider();

        let key = provider.p256_signing.private_key(pem).expect("load key");
        let sig = key.signature(b"hello world").expect("sign");

        // FIXED_SIGNING must yield exactly r‖s, never DER.
        let raw = sig.raw_representation();
        assert_eq!(raw.len(), 64);

        let der = sig.der_representation().expect("convert to DER");
        assert_eq!(der[0], 0x30, "DER must start with SEQUENCE");
        assert_eq!(der[1] as usize, der.len() - 2, "length byte must match");
        assert!(der.len() >= 68 && der.len() <= 72, "got {} bytes", der.len());
    }

    #[test]
    fn der_and_raw_describe_the_same_signature() {
        let pem = include_str!("../../../tests/resources/certs/testSigningKey.p8");
        let provider = CryptoProvider::default_provider();

        let key = provider.p256_signing.private_key(pem).expect("load key");
        let sig = key.signature(b"same signature").expect("sign");

        let raw = sig.raw_representation();
        let der = sig.der_representation().expect("convert to DER");

        // Every byte of r and s must appear in the DER, in order, ignoring
        // ASN.1 framing and any minimal-encoding adjustments.
        let r_trimmed: Vec<u8> = raw[..32].iter().copied().skip_while(|&b| b == 0).collect();
        let s_trimmed: Vec<u8> = raw[32..].iter().copied().skip_while(|&b| b == 0).collect();

        assert!(
            der.windows(r_trimmed.len()).any(|w| w == r_trimmed.as_slice()),
            "r not found in DER"
        );
        assert!(
            der.windows(s_trimmed.len()).any(|w| w == s_trimmed.as_slice()),
            "s not found in DER"
        );
    }
}

// ============================================================================
// Chain Verifier
// ============================================================================

struct AwsLcVerifiedChain {
    leaf: x509::X509Cert,
    intermediate: x509::X509Cert,
    root: x509::X509Cert,
}

impl VerifiedChain for AwsLcVerifiedChain {
    fn has_extension(&self, index: usize, oid: &str) -> bool {
        let cert = match index {
            0 => &self.leaf,
            1 => &self.intermediate,
            2 => &self.root,
            _ => return false,
        };
        cert.has_oid(oid)
    }

    fn leaf_spki_der(&self) -> Vec<u8> {
        self.leaf.spki_der().unwrap_or_default()
    }
}

impl X509Suite for AwsLc {
    fn verify_chain(
        &self,
        leaf: &[u8],
        intermediate: &[u8],
        roots: &[Vec<u8>],
        effective_date: Option<u64>,
    ) -> Result<Box<dyn VerifiedChain>, CryptoError> {
        if roots.is_empty() {
            return Err(CryptoError::VerificationError("no root certificates".into()));
        }

        let leaf = x509::X509Cert::from_der(leaf)
            .map_err(|e| CryptoError::VerificationError(e.to_string()))?;
        let intermediate = x509::X509Cert::from_der(intermediate)
            .map_err(|e| CryptoError::VerificationError(e.to_string()))?;

        let mut found: Option<x509::X509Cert> = None;
        for cert_bytes in roots {
            let root = match x509::X509Cert::from_der(cert_bytes) {
                Ok(c) => c,
                Err(_) => continue,
            };
            let root_pubkey = match root.pubkey() {
                Ok(pk) => pk,
                Err(_) => continue,
            };
            if intermediate.verify(root_pubkey) {
                found = Some(root);
                break;
            }
        }
        let root = found
            .ok_or_else(|| CryptoError::VerificationError("no matching root".into()))?;

        let intermediate_pubkey = intermediate
            .pubkey()
            .map_err(|e| CryptoError::VerificationError(e.to_string()))?;
        if !leaf.verify(intermediate_pubkey) {
            return Err(CryptoError::VerificationError("leaf signature invalid".into()));
        }

        if let Some(date) = effective_date {
            let timestamp = i64::try_from(date)
                .map_err(|_| CryptoError::VerificationError("effective date out of range".into()))?;

            for cert in [&leaf, &intermediate, &root] {
                let not_before = cert
                    .not_before()
                    .map_err(|e| CryptoError::VerificationError(e.to_string()))?;
                let not_after = cert
                    .not_after()
                    .map_err(|e| CryptoError::VerificationError(e.to_string()))?;
                if timestamp < not_before || timestamp > not_after {
                    return Err(CryptoError::VerificationError("certificate expired".into()));
                }
            }
        }

        Ok(Box::new(AwsLcVerifiedChain { leaf, intermediate, root }))
    }
}

pub const DEFAULT_PROVIDER: CryptoProvider = CryptoProvider {
    p256_signing: &AwsLc,
    x509: &AwsLc,
    sha1_hasher: &AwsLc,
};