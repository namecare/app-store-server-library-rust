use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use thiserror::Error;
use crate::chain_verifier::ChainVerificationFailureReason::{CertificateExpired, InvalidCertificate, InvalidChainLength, InvalidEffectiveDate};

use x509_parser::certificate::X509Certificate;
use x509_parser::prelude::{FromDer, ASN1Time};
use x509_parser::der_parser::asn1_rs::oid;
use x509_parser::error::X509Error;

#[derive(Error, Debug)]
pub enum ChainVerifierError {
    #[error("VerificationFailure: [{0}]")]
    VerificationFailure(ChainVerificationFailureReason),

    #[error("InternalX509Error: [{0}]")]
    InternalX509Error(#[from] X509Error),

    #[error("InternalDecodeError: [{0}]")]
    InternalDecodeError(#[from] base64::DecodeError)
}

#[derive(Error, Debug)]
pub enum ChainVerificationFailureReason {
    #[error("InvalidAppIdentifier")]
    InvalidAppIdentifier,

    #[error("InvalidIssuer")]
    InvalidIssuer,

    #[error("InvalidCertificate")]
    InvalidCertificate,

    #[error("InvalidChainLength")]
    InvalidChainLength,

    #[error("InvalidChain")]
    InvalidChain,

    #[error("InvalidEnvironment")]
    InvalidEffectiveDate,

    #[error("CertificateExpired")]
    CertificateExpired
}

const EXPECTED_CHAIN_LENGTH: usize = 3;

/// Verifies a certificate chain.
///
/// This function verifies a certificate chain consisting of multiple certificates. It performs various
/// checks to ensure the validity and integrity of the chain.
///
/// # Arguments
///
/// * `certificates`: A vector of byte slices containing the certificates in the chain.
/// * `root_certificates`: A vector of byte slices containing the root certificates.
/// * `effective_date`: An optional Unix timestamp representing the effective date for the chain validation.
///
/// # Returns
///
/// * `Ok(Vec<u8>)`: If the certificate chain is valid, it returns the public key data from the leaf certificate.
/// * `Err(ChainVerifierError)`: If the chain verification fails for any reason, it returns a `ChainVerifierError` enum.
///
/// # Example
///
/// ```rust
/// use app_store_server_library::chain_verifier::{verify_chain, ChainVerifierError};
///
/// fn main() {
///     let certificates: Vec<Vec<u8>> = vec![]; // Load your certificates here
///     let root_certificates: Vec<Vec<u8>> = vec![]; // Load your root certificates here
///     let effective_date: Option<u64> = None; // Provide an effective date if needed
///
///     match verify_chain(&certificates, &root_certificates, effective_date) {
///         Ok(public_key) => println!("Certificate chain is valid. Public key: {:?}", public_key),
///         Err(err) => eprintln!("Certificate chain verification failed: {}", err),
///     }
/// }
/// ```
///
/// TODO: Implement issuer checking
pub fn verify_chain(certificates: &Vec<Vec<u8>>, root_certificates: &Vec<Vec<u8>>, effective_date: Option<u64>) -> Result<Vec<u8>, ChainVerifierError> {
    if root_certificates.is_empty() {
        return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
    }

    if certificates.len() != EXPECTED_CHAIN_LENGTH {
        return Err(ChainVerifierError::VerificationFailure(InvalidChainLength));
    }

    let leaf_certificate = &certificates[0];
    let Ok(leaf_certificate) = X509Certificate::from_der(leaf_certificate.as_slice()) else {
        return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
    };
    let leaf_certificate = leaf_certificate.1;

    let Some(_) = leaf_certificate.get_extension_unique(&oid!(1.2.840.113635.100.6.11.1))? else {
        return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
    };

    let intermediate_certificate = &certificates[1];
    let Ok(intermediate_certificate) = X509Certificate::from_der(intermediate_certificate.as_slice()) else {
        return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
    };
    let intermediate_certificate = intermediate_certificate.1;

    let Some(_) = intermediate_certificate.get_extension_unique(&oid!(1.2.840.113635.100.6.2.1))? else {
        return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
    };

    let mut root_certificate: Option<X509Certificate> = None;

    for cert in root_certificates {
        let Ok(cert) = X509Certificate::from_der(&cert) else {
            return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
        };

        match intermediate_certificate.verify_signature(Some(cert.1.public_key())) {
            Ok(_) => (),
            Err(e) => return Err(ChainVerifierError::InternalX509Error(e))
        }

        root_certificate = Some(cert.1)
    }

    let Some(root_certificate) = root_certificate else {
        return Err(ChainVerifierError::VerificationFailure(InvalidCertificate))
    };

    leaf_certificate.verify_signature(Some(intermediate_certificate.public_key()))?;

    if let Some(date) = effective_date {
        let Ok(time) = ASN1Time::from_timestamp(i64::try_from(date).unwrap()) else {
            return Err(ChainVerifierError::VerificationFailure(InvalidEffectiveDate))
        };

        if !(root_certificate.validity.is_valid_at(time) &&
            leaf_certificate.validity.is_valid_at(time) &&
            intermediate_certificate.validity.is_valid_at(time)) {
            return Err(ChainVerifierError::VerificationFailure(CertificateExpired))
        }
    }

    let k = leaf_certificate.public_key().subject_public_key.data.to_vec();
    Ok(k)
}

#[cfg(test)]
mod tests {
    use base64::Engine;
    use base64::engine::general_purpose::STANDARD;
    use jsonwebtoken::{Algorithm, DecodingKey, Validation};
    use std::time::{SystemTime, UNIX_EPOCH};
    use crate::primitives::response_body_v2_decoded_payload::ResponseBodyV2DecodedPayload;
    use crate::utils::{StringExt, system_timestamp};
    use super::*;

    pub fn signed_payload() -> String {
        std::env::var("SIGNED_PAYLOAD").expect("SIGNED_PAYLOAD must be set")
    }

    pub fn apple_root_cert() -> String {
        std::env::var("APPLE_ROOT_BASE64_ENCODED").expect("APPLE_ROOT_BASE64_ENCODED must be set")
    }

    #[test]
    fn text_chain_verification() {
        dotenv::dotenv().ok();

        let payload = signed_payload();

        let header = jsonwebtoken::decode_header(payload.as_str()).expect("Expect header");
        let Some(x5c) = header.x5c else {
            return;
        };
        let x5c = x5c.iter().map(|c| c.as_der_bytes().unwrap()).collect();

        let root_cert = apple_root_cert().as_der_bytes().expect("Expect bytes");
        let root_certificates = vec![root_cert];

        let effective_date = Some(system_timestamp());

        let pub_key = verify_chain(&x5c, &root_certificates, effective_date).unwrap();
        assert_eq!(pub_key.len(), 65)
    }
}