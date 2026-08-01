//! RustCrypto backend implementation

use p256::ecdsa::signature::Signer;
use p256::ecdsa::SigningKey;
use p256::pkcs8::DecodePrivateKey;

use crate::crypto::{
    CryptoError, CryptoProvider, P256PrivateKey, P256PublicKey, P256Signature,
    P256SigningSuite, Sha1Hasher, VerifiedChain, X509Suite,
};

/// Marker type implementing every capability this backend provides.
#[derive(Debug)]
struct RustCrypto;

impl Sha1Hasher for RustCrypto {
    fn hash(&self, data: &[u8]) -> [u8; 20] {
        use sha1::{Digest, Sha1};
        Sha1::digest(data).into()
    }
}

#[derive(Debug)]
struct RustCryptoP256PrivateKey {
    key: SigningKey,
}

impl P256PrivateKey for RustCryptoP256PrivateKey {
    fn signature(&self, message: &[u8]) -> Result<Box<dyn P256Signature>, CryptoError> {
        let sig: p256::ecdsa::Signature = self.key.sign(message);
        Ok(Box::new(RustCryptoP256Signature { sig }))
    }
}

#[derive(Debug)]
struct RustCryptoP256PublicKey {
    key: p256::ecdsa::VerifyingKey,
}

impl P256PublicKey for RustCryptoP256PublicKey {
    fn is_valid_signature(
        &self,
        signature: &dyn P256Signature,
        message: &[u8],
    ) -> Result<(), CryptoError> {
        use signature::Verifier;

        // Rebuild from the raw form so signatures from any backend are accepted.
        let sig = p256::ecdsa::Signature::from_slice(&signature.raw_representation())
            .map_err(|e| CryptoError::VerificationError(e.to_string()))?;

        self.key
            .verify(message, &sig)
            .map_err(|e| CryptoError::VerificationError(e.to_string()))
    }
}

#[derive(Debug)]
struct RustCryptoP256Signature {
    sig: p256::ecdsa::Signature,
}

impl P256Signature for RustCryptoP256Signature {
    fn raw_representation(&self) -> [u8; 64] {
        self.sig.to_bytes().into()
    }

    fn der_representation(&self) -> Result<Vec<u8>, CryptoError> {
        Ok(self.sig.to_der().to_bytes().to_vec())
    }
}

impl P256SigningSuite for RustCrypto {
    fn private_key(&self, pem: &str) -> Result<Box<dyn P256PrivateKey>, CryptoError> {
        let mut buf = [0u8; 2048];
        let (_, der) = pem_rfc7468::decode(pem.as_bytes(), &mut buf)
            .map_err(|e| CryptoError::KeyError(e.to_string()))?;

        let key = SigningKey::from_pkcs8_der(der)
            .map_err(|e| CryptoError::KeyError(e.to_string()))?;

        Ok(Box::new(RustCryptoP256PrivateKey { key }))
    }

    fn public_key(&self, spki_der: &[u8]) -> Result<Box<dyn P256PublicKey>, CryptoError> {
        use p256::pkcs8::DecodePublicKey;

        let key = p256::ecdsa::VerifyingKey::from_public_key_der(spki_der)
            .map_err(|e| CryptoError::KeyError(e.to_string()))?;

        Ok(Box::new(RustCryptoP256PublicKey { key }))
    }

    fn signature_from_raw(&self, rs: &[u8; 64]) -> Result<Box<dyn P256Signature>, CryptoError> {
        let sig = p256::ecdsa::Signature::from_slice(rs)
            .map_err(|e| CryptoError::VerificationError(e.to_string()))?;
        Ok(Box::new(RustCryptoP256Signature { sig }))
    }
}

use const_oid::ObjectIdentifier;
use der::referenced::OwnedToRef;
use der::{Decode, Encode};
use x509_cert::time::Time;
use x509_cert::Certificate;

// Signature algorithm OIDs
const OID_RSA_SHA256: &str = "1.2.840.113549.1.1.11";
const OID_RSA_SHA384: &str = "1.2.840.113549.1.1.12";
const OID_ECDSA_SHA256: &str = "1.2.840.10045.4.3.2";
const OID_ECDSA_SHA384: &str = "1.2.840.10045.4.3.3";

struct RustCryptoVerifiedChain {
    leaf: Certificate,
    intermediate: Certificate,
    root: Certificate,
}

impl VerifiedChain for RustCryptoVerifiedChain {
    fn has_extension(&self, index: usize, oid: &str) -> bool {
        let cert = match index {
            0 => &self.leaf,
            1 => &self.intermediate,
            2 => &self.root,
            _ => return false,
        };
        let Ok(parsed) = ObjectIdentifier::new(oid) else {
            return false;
        };
        has_extension(cert, &parsed)
    }

    fn leaf_spki_der(&self) -> Vec<u8> {
        public_key_bytes(&self.leaf)
    }
}

impl X509Suite for RustCrypto {
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

        let leaf = parse_certificate(leaf).map_err(CryptoError::VerificationError)?;
        let intermediate = parse_certificate(intermediate).map_err(CryptoError::VerificationError)?;

        // Find the root that signed the intermediate, by trial verification.
        let mut root: Option<Certificate> = None;
        for cert_bytes in roots {
            let cert = parse_certificate(cert_bytes).map_err(CryptoError::VerificationError)?;
            if verify_signature(&intermediate, &cert).is_ok() {
                root = Some(cert);
                break;
            }
        }
        let root = root.ok_or_else(|| CryptoError::VerificationError("no matching root".into()))?;

        verify_signature(&leaf, &intermediate).map_err(CryptoError::VerificationError)?;

        if let Some(date) = effective_date {
            let timestamp = i64::try_from(date)
                .map_err(|_| CryptoError::VerificationError("effective date out of range".into()))?;

            if !is_valid_at(&leaf, timestamp)
                || !is_valid_at(&intermediate, timestamp)
                || !is_valid_at(&root, timestamp)
            {
                return Err(CryptoError::VerificationError("certificate expired".into()));
            }
        }

        Ok(Box::new(RustCryptoVerifiedChain { leaf, intermediate, root }))
    }
}

fn parse_certificate(der_bytes: &[u8]) -> Result<Certificate, String> {
    Certificate::from_der(der_bytes).map_err(|e| e.to_string())
}

fn has_extension(cert: &Certificate, oid: &ObjectIdentifier) -> bool {
    cert.tbs_certificate
        .extensions
        .as_ref()
        .map(|exts| {
            exts.iter()
                .any(|ext| ext.extn_id == *oid)
        })
        .unwrap_or(false)
}

fn public_key_bytes(cert: &Certificate) -> Vec<u8> {
    let spki = &cert
        .tbs_certificate
        .subject_public_key_info;
    spki.owned_to_ref()
        .to_der()
        .unwrap_or_default()
}

fn is_valid_at(cert: &Certificate, timestamp: i64) -> bool {
    let validity = &cert.tbs_certificate.validity;

    let not_before_ok = match &validity.not_before {
        Time::UtcTime(t) => timestamp >= t.to_unix_duration().as_secs() as i64,
        Time::GeneralTime(t) => timestamp >= t.to_unix_duration().as_secs() as i64,
    };

    let not_after_ok = match &validity.not_after {
        Time::UtcTime(t) => timestamp <= t.to_unix_duration().as_secs() as i64,
        Time::GeneralTime(t) => timestamp <= t.to_unix_duration().as_secs() as i64,
    };

    not_before_ok && not_after_ok
}

fn verify_signature(cert: &Certificate, issuer: &Certificate) -> Result<(), String> {
    let issuer_spki = (&issuer
        .tbs_certificate
        .subject_public_key_info)
        .owned_to_ref();

    let tbs_bytes = cert
        .tbs_certificate
        .to_der()
        .map_err(|e| e.to_string())?;

    let signature_bytes = cert.signature.raw_bytes();
    let sig_alg_oid = cert.signature_algorithm.oid.to_string();

    match sig_alg_oid.as_str() {
        OID_RSA_SHA256 => {
            let spki_der = issuer_spki
                .to_der()
                .map_err(|e| e.to_string())?;
            verify_rsa_sha256(&tbs_bytes, signature_bytes, &spki_der)
        }
        OID_RSA_SHA384 => {
            let spki_der = issuer_spki
                .to_der()
                .map_err(|e| e.to_string())?;
            verify_rsa_sha384(&tbs_bytes, signature_bytes, &spki_der)
        }
        OID_ECDSA_SHA256 => {
            let key_bytes = issuer_spki
                .subject_public_key
                .raw_bytes();
            verify_ecdsa_p256(&tbs_bytes, signature_bytes, key_bytes)
        }
        OID_ECDSA_SHA384 => {
            let key_bytes = issuer_spki
                .subject_public_key
                .raw_bytes();
            match key_bytes.len() {
                65 => verify_ecdsa_p256_sha384(&tbs_bytes, signature_bytes, key_bytes),
                97 => verify_ecdsa_p384(&tbs_bytes, signature_bytes, key_bytes),
                len => Err(format!("unexpected ECDSA key length: {len} bytes")),
            }
        }
        _ => Err(format!("unsupported signature algorithm: {sig_alg_oid}")),
    }
}

fn verify_rsa_sha256(message: &[u8], signature: &[u8], spki_der: &[u8]) -> Result<(), String> {
    use rsa::pkcs1v15::{Signature, VerifyingKey};
    use rsa::pkcs8::DecodePublicKey;
    use rsa::RsaPublicKey;
    use sha2::Sha256;
    use signature::Verifier;

    let public_key = RsaPublicKey::from_public_key_der(spki_der).map_err(|e| e.to_string())?;
    let verifying_key = VerifyingKey::<Sha256>::new(public_key);
    let sig = Signature::try_from(signature).map_err(|e| e.to_string())?;

    verifying_key
        .verify(message, &sig)
        .map_err(|e| e.to_string())
}

fn verify_rsa_sha384(message: &[u8], signature: &[u8], spki_der: &[u8]) -> Result<(), String> {
    use rsa::pkcs1v15::{Signature, VerifyingKey};
    use rsa::pkcs8::DecodePublicKey;
    use rsa::RsaPublicKey;
    use sha2::Sha384;
    use signature::Verifier;

    let public_key = RsaPublicKey::from_public_key_der(spki_der).map_err(|e| e.to_string())?;
    let verifying_key = VerifyingKey::<Sha384>::new(public_key);
    let sig = Signature::try_from(signature).map_err(|e| e.to_string())?;

    verifying_key
        .verify(message, &sig)
        .map_err(|e| e.to_string())
}

fn verify_ecdsa_p256(message: &[u8], signature: &[u8], public_key: &[u8]) -> Result<(), String> {
    use p256::ecdsa::VerifyingKey;
    use signature::Verifier;

    let verifying_key = VerifyingKey::from_sec1_bytes(public_key).map_err(|e| e.to_string())?;
    let sig = p256::ecdsa::Signature::from_der(signature).map_err(|e| e.to_string())?;

    verifying_key
        .verify(message, &sig)
        .map_err(|e| e.to_string())
}

fn verify_ecdsa_p384(message: &[u8], signature: &[u8], public_key: &[u8]) -> Result<(), String> {
    use p384::ecdsa::VerifyingKey;
    use signature::Verifier;

    let verifying_key = VerifyingKey::from_sec1_bytes(public_key).map_err(|e| e.to_string())?;
    let sig = p384::ecdsa::Signature::from_der(signature).map_err(|e| e.to_string())?;
    verifying_key
        .verify(message, &sig)
        .map_err(|e| e.to_string())
}

fn verify_ecdsa_p256_sha384(message: &[u8], signature: &[u8], public_key: &[u8]) -> Result<(), String> {
    use p256::ecdsa::VerifyingKey;
    use sha2::{Digest, Sha384};
    use signature::hazmat::PrehashVerifier;

    let verifying_key = VerifyingKey::from_sec1_bytes(public_key).map_err(|e| e.to_string())?;
    let sig = p256::ecdsa::Signature::from_der(signature).map_err(|e| e.to_string())?;
    let prehash = Sha384::digest(message);

    verifying_key
        .verify_prehash(&prehash, &sig)
        .map_err(|e| e.to_string())
}

#[cfg(feature = "ocsp")]
mod ocsp_support {
    use super::*;
    use const_oid::db::rfc5280::ID_AD_OCSP;
    use der::Encode;
    use x509_ocsp::builder::OcspRequestBuilder;
    use x509_ocsp::Version;

    /// Internal error type for OCSP validation
    #[derive(Debug)]
    #[allow(dead_code)]
    pub enum OcspError {
        /// Network-related error (connection failure, timeout, etc.)
        NetworkError(String),
        /// HTTP error with non-200 status code
        HttpError(u16),
        /// Failed to read response body
        FetchFailed,
        /// Certificate has been revoked
        CertificateRevoked,
        /// Other validation errors
        ValidationError,
    }

    /// Checks the OCSP revocation status of a certificate
    pub fn check_ocsp_status(leaf: &Certificate, issuer: &Certificate) -> Result<(), ChainVerifierError> {
        match check_ocsp_status_internal(leaf, issuer) {
            Ok(()) => Ok(()),
            Err(OcspError::NetworkError(_)) | Err(OcspError::HttpError(_)) | Err(OcspError::FetchFailed) => Err(
                ChainVerifierError::VerificationFailure(ChainVerificationFailureReason::RetryableVerificationFailure),
            ),
            Err(OcspError::CertificateRevoked) => Err(ChainVerifierError::VerificationFailure(
                ChainVerificationFailureReason::CertificateRevoked,
            )),
            Err(OcspError::ValidationError) => Err(ChainVerifierError::VerificationFailure(
                ChainVerificationFailureReason::InvalidCertificate,
            )),
        }
    }

    fn check_ocsp_status_internal(leaf: &Certificate, issuer: &Certificate) -> Result<(), OcspError> {
        use sha1::Sha1;
        use x509_ocsp::{BasicOcspResponse, CertStatus, OcspResponse, Request};

        let ocsp_url = extract_ocsp_url(leaf).map_err(|_| OcspError::ValidationError)?;

        let request = Request::from_cert::<Sha1>(issuer, leaf).map_err(|_| OcspError::ValidationError)?;

        let ocsp_request = OcspRequestBuilder::new(Version::V1)
            .with_request(request)
            .build();

        let request_bytes = ocsp_request
            .to_der()
            .map_err(|_| OcspError::ValidationError)?;

        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .map_err(|e| OcspError::NetworkError(format!("Failed to build HTTP client: {}", e)))?;

        let response = client
            .post(&ocsp_url)
            .header("Content-Type", "application/ocsp-request")
            .body(request_bytes)
            .send()
            .map_err(|e| OcspError::NetworkError(format!("OCSP request failed: {}", e)))?;

        let status = response.status();
        if !status.is_success() {
            return Err(OcspError::HttpError(status.as_u16()));
        }

        let response_bytes = response
            .bytes()
            .map_err(|_| OcspError::FetchFailed)?;

        let ocsp_response =
            OcspResponse::from_der(response_bytes.to_vec().as_slice()).map_err(|_| OcspError::ValidationError)?;

        use x509_ocsp::OcspResponseStatus;
        match ocsp_response.response_status {
            OcspResponseStatus::Successful => {}
            _ => return Err(OcspError::ValidationError),
        }

        let response_bytes = ocsp_response
            .response_bytes
            .ok_or(OcspError::ValidationError)?;

        const ID_PKIX_OCSP_BASIC: &str = "1.3.6.1.5.5.7.48.1.1";
        if response_bytes.response_type.to_string() != ID_PKIX_OCSP_BASIC {
            return Err(OcspError::ValidationError);
        }

        let basic_response =
            BasicOcspResponse::from_der(response_bytes.response.as_bytes()).map_err(|_| OcspError::ValidationError)?;

        for single_response in &basic_response
            .tbs_response_data
            .responses
        {
            match &single_response.cert_status {
                CertStatus::Good(_) => return Ok(()),
                CertStatus::Revoked(_) => return Err(OcspError::CertificateRevoked),
                CertStatus::Unknown(_) => return Err(OcspError::ValidationError),
            }
        }

        Err(OcspError::ValidationError)
    }

    fn extract_ocsp_url(cert: &Certificate) -> Result<String, ChainVerifierError> {
        // AIA extension OID: 1.3.6.1.5.5.7.1.1
        let aia_oid = ObjectIdentifier::new_unwrap("1.3.6.1.5.5.7.1.1");

        let Some(extensions) = &cert.tbs_certificate.extensions else {
            return Err(ChainVerifierError::VerificationFailure(
                ChainVerificationFailureReason::InvalidCertificate,
            ));
        };

        for ext in extensions {
            if ext.extn_id == aia_oid {
                if let Ok(url) = parse_aia_for_ocsp(ext.extn_value.as_bytes()) {
                    return Ok(url);
                }
            }
        }

        Err(ChainVerifierError::VerificationFailure(
            ChainVerificationFailureReason::InvalidCertificate,
        ))
    }

    fn parse_aia_for_ocsp(aia_bytes: &[u8]) -> Result<String, ChainVerifierError> {
        use crate::crypto::asn1::asn1::{read_oid, read_sequence, read_tlv};

        let (mut offset, length) =
            read_sequence(aia_bytes, 0).map_err(|e| ChainVerifierError::InternalError(e.to_string()))?;

        let end_offset = offset + length;

        while offset < end_offset {
            let (desc_offset, desc_length) =
                read_sequence(aia_bytes, offset).map_err(|e| ChainVerifierError::InternalError(e.to_string()))?;

            let desc_end = desc_offset + desc_length;

            let (oid_offset, oid_length) =
                read_oid(aia_bytes, desc_offset).map_err(|e| ChainVerifierError::InternalError(e.to_string()))?;

            let oid_bytes = &aia_bytes[oid_offset..oid_offset + oid_length];
            let expected_ocsp_oid = ID_AD_OCSP.as_bytes();

            if oid_bytes == expected_ocsp_oid {
                let location_offset = oid_offset + oid_length;
                let (tag, uri_length, uri_offset) = read_tlv(aia_bytes, location_offset)
                    .map_err(|e| ChainVerifierError::InternalError(e.to_string()))?;

                // Tag [6] for uniformResourceIdentifier is 0x86
                if tag == 0x86 {
                    let uri_bytes = &aia_bytes[uri_offset..uri_offset + uri_length];
                    let uri = std::str::from_utf8(uri_bytes).map_err(|_| {
                        ChainVerifierError::VerificationFailure(ChainVerificationFailureReason::InvalidCertificate)
                    })?;
                    return Ok(uri.to_string());
                }
            }

            offset = desc_end;
        }

        Err(ChainVerifierError::VerificationFailure(
            ChainVerificationFailureReason::InvalidCertificate,
        ))
    }
}

#[cfg(feature = "ocsp")]
pub use ocsp_support::check_ocsp_status;

pub const DEFAULT_PROVIDER: CryptoProvider = CryptoProvider {
    p256_signing: &RustCrypto,
    x509: &RustCrypto,
    sha1_hasher: &RustCrypto,
};
