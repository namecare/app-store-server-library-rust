/// Custom X.509 certificate verification using x509-cert from RustCrypto
use x509_cert::Certificate;
use der::Decode;
use const_oid::ObjectIdentifier;

#[derive(Debug, PartialEq)]
pub enum X509Error {
    ParseError(String),
    VerificationError(String),
    InvalidCertificate(String),
}

impl std::fmt::Display for X509Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            X509Error::ParseError(msg) => write!(f, "ParseError: {}", msg),
            X509Error::VerificationError(msg) => write!(f, "VerificationError: {}", msg),
            X509Error::InvalidCertificate(msg) => write!(f, "InvalidCertificate: {}", msg),
        }
    }
}

impl std::error::Error for X509Error {}

impl From<der::Error> for X509Error {
    fn from(err: der::Error) -> Self {
        X509Error::ParseError(err.to_string())
    }
}

impl From<spki::Error> for X509Error {
    fn from(err: spki::Error) -> Self {
        X509Error::ParseError(err.to_string())
    }
}

/// Parse a DER-encoded X.509 certificate
pub fn parse_certificate(der_bytes: &[u8]) -> Result<Certificate, X509Error> {
    Certificate::from_der(der_bytes).map_err(|e| X509Error::ParseError(e.to_string()))
}

/// Check if a certificate has a specific extension by OID
pub fn has_extension(cert: &Certificate, oid: &ObjectIdentifier) -> bool {
    if let Some(extensions) = &cert.tbs_certificate.extensions {
        extensions.iter().any(|ext| ext.extn_id == *oid)
    } else {
        false
    }
}

/// Extract the public key bytes from a certificate
/// This returns the full SPKI (SubjectPublicKeyInfo) structure in DER format
pub fn public_key_bytes(cert: &Certificate) -> Vec<u8> {
    use der::referenced::OwnedToRef;
    use der::Encode;

    let spki_owned = &cert.tbs_certificate.subject_public_key_info;
    let spki_ref = spki_owned.owned_to_ref();

    // Return the full SPKI DER encoding (not just the raw key bytes)
    spki_ref.to_der().unwrap_or_default()
}

/// Check if a certificate is valid at a specific Unix timestamp
pub fn is_valid_at(cert: &Certificate, timestamp: i64) -> bool {
    use x509_cert::time::Time;

    let validity = &cert.tbs_certificate.validity;

    // Check not_before
    let not_before_valid = match &validity.not_before {
        Time::UtcTime(utc) => {
            let not_before_ts = utc.to_unix_duration().as_secs() as i64;
            timestamp >= not_before_ts
        }
        Time::GeneralTime(gen) => {
            let not_before_ts = gen.to_unix_duration().as_secs() as i64;
            timestamp >= not_before_ts
        }
    };

    // Check not_after
    let not_after_valid = match &validity.not_after {
        Time::UtcTime(utc) => {
            let not_after_ts = utc.to_unix_duration().as_secs() as i64;
            timestamp <= not_after_ts
        }
        Time::GeneralTime(gen) => {
            let not_after_ts = gen.to_unix_duration().as_secs() as i64;
            timestamp <= not_after_ts
        }
    };

    not_before_valid && not_after_valid
}

/// Verify the signature of a certificate using the issuer's public key
pub fn verify_signature(cert: &Certificate, issuer: &Certificate) -> Result<(), X509Error> {
    use der::referenced::OwnedToRef;

    let issuer_spki = (&issuer.tbs_certificate.subject_public_key_info).owned_to_ref();
    verify_signature_with_spki(cert, &issuer_spki)
}

// Signature algorithm OIDs
const OID_RSA_SHA256: &str = "1.2.840.113549.1.1.11";
const OID_RSA_SHA384: &str = "1.2.840.113549.1.1.12";
const OID_ECDSA_SHA256: &str = "1.2.840.10045.4.3.2";
const OID_ECDSA_SHA384: &str = "1.2.840.10045.4.3.3";

/// Verify signature using SPKI (Subject Public Key Info)
fn verify_signature_with_spki(
    cert: &Certificate,
    issuer_spki: &spki::SubjectPublicKeyInfoRef,
) -> Result<(), X509Error> {
    use der::Encode;

    let tbs_bytes = cert
        .tbs_certificate
        .to_der()
        .map_err(|e| X509Error::VerificationError(e.to_string()))?;

    let signature_bytes = cert.signature.raw_bytes();
    let sig_alg_oid = cert.signature_algorithm.oid.to_string();

    match sig_alg_oid.as_str() {
        OID_RSA_SHA256 => verify_rsa_sha256_signature(&tbs_bytes, signature_bytes, issuer_spki),
        OID_RSA_SHA384 => verify_rsa_sha384_signature(&tbs_bytes, signature_bytes, issuer_spki),
        OID_ECDSA_SHA256 => verify_ecdsa_p256_sha256_signature(&tbs_bytes, signature_bytes, issuer_spki),
        OID_ECDSA_SHA384 => verify_ecdsa_p384_sha384_signature(&tbs_bytes, signature_bytes, issuer_spki),
        _ => Err(X509Error::InvalidCertificate(format!(
            "Unsupported signature algorithm: {}",
            sig_alg_oid
        ))),
    }
}

/// Verify RSA-SHA256 signature
fn verify_rsa_sha256_signature(
    message: &[u8],
    signature: &[u8],
    spki: &spki::SubjectPublicKeyInfoRef,
) -> Result<(), X509Error> {
    use der::Encode;
    use rsa::pkcs8::DecodePublicKey;
    use rsa::RsaPublicKey;
    use rsa::pkcs1v15;
    use sha2::Sha256;

    let spki_der = spki.to_der()
        .map_err(|e| X509Error::VerificationError(format!("Failed to encode SPKI: {:?}", e)))?;

    let public_key = RsaPublicKey::from_public_key_der(&spki_der)
        .map_err(|e| X509Error::VerificationError(format!("Failed to parse RSA public key: {:?}", e)))?;

    let verifying_key = pkcs1v15::VerifyingKey::<Sha256>::new(public_key);
    let sig = pkcs1v15::Signature::try_from(signature)
        .map_err(|e| X509Error::VerificationError(format!("Invalid RSA signature: {:?}", e)))?;

    signature::Verifier::verify(&verifying_key, message, &sig)
        .map_err(|e| X509Error::VerificationError(format!("RSA-SHA256 verification failed: {:?}", e)))
}

/// Verify RSA-SHA384 signature
fn verify_rsa_sha384_signature(
    message: &[u8],
    signature: &[u8],
    spki: &spki::SubjectPublicKeyInfoRef,
) -> Result<(), X509Error> {
    use der::Encode;
    use rsa::pkcs8::DecodePublicKey;
    use rsa::RsaPublicKey;
    use rsa::pkcs1v15;
    use sha2::Sha384;

    let spki_der = spki.to_der()
        .map_err(|e| X509Error::VerificationError(format!("Failed to encode SPKI: {:?}", e)))?;

    let public_key = RsaPublicKey::from_public_key_der(&spki_der)
        .map_err(|e| X509Error::VerificationError(format!("Failed to parse RSA public key: {:?}", e)))?;

    let verifying_key = pkcs1v15::VerifyingKey::<Sha384>::new(public_key);
    let sig = pkcs1v15::Signature::try_from(signature)
        .map_err(|e| X509Error::VerificationError(format!("Invalid RSA signature: {:?}", e)))?;

    signature::Verifier::verify(&verifying_key, message, &sig)
        .map_err(|e| X509Error::VerificationError(format!("RSA-SHA384 verification failed: {:?}", e)))
}

/// Verify ECDSA P-256 SHA-256 signature
fn verify_ecdsa_p256_sha256_signature(
    message: &[u8],
    signature: &[u8],
    spki: &spki::SubjectPublicKeyInfoRef,
) -> Result<(), X509Error> {
    use p256::ecdsa::VerifyingKey;

    let public_key_bytes = spki.subject_public_key.raw_bytes();

    let verifying_key = VerifyingKey::from_sec1_bytes(public_key_bytes)
        .map_err(|e| X509Error::VerificationError(
            format!("Failed to parse P-256 public key: {:?}", e)
        ))?;

    let der_sig = p256::ecdsa::DerSignature::from_bytes(signature)
        .map_err(|e| X509Error::VerificationError(
            format!("Invalid ECDSA signature: {:?}", e)
        ))?;

    signature::Verifier::verify(&verifying_key, message, &der_sig)
        .map_err(|e| X509Error::VerificationError(
            format!("ECDSA-P256-SHA256 verification failed: {:?}", e)
        ))
}

/// Verify ECDSA SHA-384 signature (supports both P-256 and P-384 keys)
fn verify_ecdsa_p384_sha384_signature(
    message: &[u8],
    signature: &[u8],
    spki: &spki::SubjectPublicKeyInfoRef,
) -> Result<(), X509Error> {
    let public_key_bytes = spki.subject_public_key.raw_bytes();

    // Check the key size to determine if this is actually P-256 or P-384
    // P-256: 65 bytes (1 prefix + 32*2)
    // P-384: 97 bytes (1 prefix + 48*2)
    if public_key_bytes.len() == 65 {
        verify_ecdsa_p256_sha384_signature(message, signature, public_key_bytes)
    } else if public_key_bytes.len() == 97 {
        verify_ecdsa_p384_sha384_standard(message, signature, public_key_bytes)
    } else {
        Err(X509Error::VerificationError(format!(
            "Unexpected ECDSA key length: {} bytes",
            public_key_bytes.len()
        )))
    }
}

/// Verify ECDSA P-384 SHA-384 signature with a P-384 key
fn verify_ecdsa_p384_sha384_standard(
    message: &[u8],
    signature: &[u8],
    public_key_bytes: &[u8],
) -> Result<(), X509Error> {
    use p384::ecdsa::VerifyingKey;

    let verifying_key = VerifyingKey::from_sec1_bytes(public_key_bytes)
        .map_err(|e| X509Error::VerificationError(
            format!("Failed to parse P-384 public key: {:?}", e)
        ))?;

    let der_sig = p384::ecdsa::DerSignature::from_bytes(signature)
        .map_err(|e| X509Error::VerificationError(
            format!("Invalid ECDSA signature: {:?}", e)
        ))?;

    signature::Verifier::verify(&verifying_key, message, &der_sig)
        .map_err(|e| X509Error::VerificationError(
            format!("ECDSA-P384-SHA384 verification failed: {:?}", e)
        ))
}

/// Verify ECDSA P-256 key with SHA-384 signature (edge case for some test certificates)
fn verify_ecdsa_p256_sha384_signature(
    message: &[u8],
    signature: &[u8],
    public_key_bytes: &[u8],
) -> Result<(), X509Error> {
    use p256::ecdsa::VerifyingKey;
    use sha2::{Sha384, Digest};
    use signature::hazmat::PrehashVerifier;

    let verifying_key = VerifyingKey::from_sec1_bytes(public_key_bytes)
        .map_err(|e| X509Error::VerificationError(
            format!("Failed to parse P-256 public key: {:?}", e)
        ))?;

    let der_sig = p256::ecdsa::DerSignature::from_bytes(signature)
        .map_err(|e| X509Error::VerificationError(
            format!("Invalid ECDSA signature: {:?}", e)
        ))?;

    let prehash = Sha384::digest(message);
    verifying_key.verify_prehash(&prehash, &der_sig)
        .map_err(|e| X509Error::VerificationError(
            format!("ECDSA-P256-SHA384 verification failed: {:?}", e)
        ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oid_creation() {
        // Apple-specific OIDs
        let leaf_oid = ObjectIdentifier::new("1.2.840.113635.100.6.11.1");
        assert!(leaf_oid.is_ok());

        let intermediate_oid = ObjectIdentifier::new("1.2.840.113635.100.6.2.1");
        assert!(intermediate_oid.is_ok());
    }

    #[test]
    fn test_signature_algorithm_oids() {
        let rsa_sha256 = ObjectIdentifier::new("1.2.840.113549.1.1.11");
        assert!(rsa_sha256.is_ok());

        let ecdsa_sha256 = ObjectIdentifier::new("1.2.840.10045.4.3.2");
        assert!(ecdsa_sha256.is_ok());
    }
}