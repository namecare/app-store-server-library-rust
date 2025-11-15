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
/// to maintain compatibility with x509-parser behavior
pub fn public_key_bytes(cert: &Certificate) -> Vec<u8> {
    use der::referenced::OwnedToRef;
    use der::Encode;

    // Get SPKI and convert to reference
    let spki_owned = &cert.tbs_certificate.subject_public_key_info;
    let spki_ref = spki_owned.owned_to_ref();

    // Return the full SPKI DER encoding (not just the raw key bytes)
    // This matches the behavior of x509-parser
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

    // Get the issuer's public key info
    let issuer_spki = (&issuer.tbs_certificate.subject_public_key_info).owned_to_ref();

    // Verify the signature based on the algorithm
    verify_signature_with_spki(cert, &issuer_spki)
}

/// Verify signature using SPKI (Subject Public Key Info)
fn verify_signature_with_spki(
    cert: &Certificate,
    issuer_spki: &spki::SubjectPublicKeyInfoRef,
) -> Result<(), X509Error> {
    use der::Encode;

    // Get the signed data (TBS certificate)
    let tbs_bytes = cert
        .tbs_certificate
        .to_der()
        .map_err(|e| X509Error::VerificationError(e.to_string()))?;

    let signature_bytes = cert.signature.raw_bytes();

    // Determine the signature algorithm
    let sig_alg_oid = &cert.signature_algorithm.oid;

    // RSA with SHA-256: 1.2.840.113549.1.1.11
    let rsa_sha256_oid = ObjectIdentifier::new("1.2.840.113549.1.1.11")
        .map_err(|e| X509Error::InvalidCertificate(e.to_string()))?;

    // ECDSA with SHA-256: 1.2.840.10045.4.3.2
    let ecdsa_sha256_oid = ObjectIdentifier::new("1.2.840.10045.4.3.2")
        .map_err(|e| X509Error::InvalidCertificate(e.to_string()))?;

    // RSA with SHA-384: 1.2.840.113549.1.1.12
    let rsa_sha384_oid = ObjectIdentifier::new("1.2.840.113549.1.1.12")
        .map_err(|e| X509Error::InvalidCertificate(e.to_string()))?;

    // ECDSA with SHA-384: 1.2.840.10045.4.3.3
    let ecdsa_sha384_oid = ObjectIdentifier::new("1.2.840.10045.4.3.3")
        .map_err(|e| X509Error::InvalidCertificate(e.to_string()))?;

    if *sig_alg_oid == rsa_sha256_oid {
        verify_rsa_sha256_signature(&tbs_bytes, signature_bytes, issuer_spki)?;
    } else if *sig_alg_oid == ecdsa_sha256_oid {
        verify_ecdsa_p256_sha256_signature(&tbs_bytes, signature_bytes, issuer_spki)?;
    } else if *sig_alg_oid == rsa_sha384_oid {
        verify_rsa_sha384_signature(&tbs_bytes, signature_bytes, issuer_spki)?;
    } else if *sig_alg_oid == ecdsa_sha384_oid {
        verify_ecdsa_p384_sha384_signature(&tbs_bytes, signature_bytes, issuer_spki)?;
    } else {
        return Err(X509Error::InvalidCertificate(format!(
            "Unsupported signature algorithm: {}",
            sig_alg_oid
        )));
    }

    Ok(())
}

/// Verify RSA-SHA256 signature using ring
fn verify_rsa_sha256_signature(
    message: &[u8],
    signature: &[u8],
    spki: &spki::SubjectPublicKeyInfoRef,
) -> Result<(), X509Error> {
    use der::Encode;

    // For RSA, ring needs the full SPKI DER encoding, not just the raw key bytes
    let spki_der = spki.to_der()
        .map_err(|e| X509Error::VerificationError(format!("Failed to encode SPKI: {:?}", e)))?;

    let public_key = ring::signature::UnparsedPublicKey::new(
        &ring::signature::RSA_PKCS1_2048_8192_SHA256,
        &spki_der,
    );

    public_key
        .verify(message, signature)
        .map_err(|e| X509Error::VerificationError(format!("RSA-SHA256 verification failed: {:?}", e)))
}

/// Verify RSA-SHA384 signature using ring
fn verify_rsa_sha384_signature(
    message: &[u8],
    signature: &[u8],
    spki: &spki::SubjectPublicKeyInfoRef,
) -> Result<(), X509Error> {
    use der::Encode;

    // For RSA, ring needs the full SPKI DER encoding, not just the raw key bytes
    let spki_der = spki.to_der()
        .map_err(|e| X509Error::VerificationError(format!("Failed to encode SPKI: {:?}", e)))?;

    let public_key = ring::signature::UnparsedPublicKey::new(
        &ring::signature::RSA_PKCS1_2048_8192_SHA384,
        &spki_der,
    );

    public_key
        .verify(message, signature)
        .map_err(|e| X509Error::VerificationError(format!("RSA-SHA384 verification failed: {:?}", e)))
}

/// Verify ECDSA P-256 SHA-256 signature using ring
fn verify_ecdsa_p256_sha256_signature(
    message: &[u8],
    signature: &[u8],
    spki: &spki::SubjectPublicKeyInfoRef,
) -> Result<(), X509Error> {
    // For ECDSA, ring expects the raw public key bytes (EC point), not the full SPKI
    let public_key_bytes = spki.subject_public_key.raw_bytes();

    let public_key = ring::signature::UnparsedPublicKey::new(
        &ring::signature::ECDSA_P256_SHA256_ASN1,
        public_key_bytes,
    );

    public_key
        .verify(message, signature)
        .map_err(|e| {
            X509Error::VerificationError(format!("ECDSA-P256-SHA256 verification failed: {:?}", e))
        })
}

/// Verify ECDSA P-384 SHA-384 signature using ring
fn verify_ecdsa_p384_sha384_signature(
    message: &[u8],
    signature: &[u8],
    spki: &spki::SubjectPublicKeyInfoRef,
) -> Result<(), X509Error> {
    // For ECDSA, ring expects the raw public key bytes (EC point), not the full SPKI
    let public_key_bytes = spki.subject_public_key.raw_bytes();

    // Check the key size to determine if this is actually P-256 or P-384
    // P-256: 65 bytes (1 prefix + 32*2)
    // P-384: 97 bytes (1 prefix + 48*2)
    let (algorithm, key_type) = if public_key_bytes.len() == 65 {
        // Some test certificates use P-256 keys with SHA-384 signatures
        (&ring::signature::ECDSA_P256_SHA384_ASN1, "P-256")
    } else if public_key_bytes.len() == 97 {
        (&ring::signature::ECDSA_P384_SHA384_ASN1, "P-384")
    } else {
        return Err(X509Error::VerificationError(format!(
            "Unexpected ECDSA key length: {} bytes",
            public_key_bytes.len()
        )));
    };

    let public_key = ring::signature::UnparsedPublicKey::new(
        algorithm,
        public_key_bytes,
    );

    public_key
        .verify(message, signature)
        .map_err(|e| X509Error::VerificationError(format!("ECDSA-{}-SHA384 verification failed: {:?}", key_type, e)))
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