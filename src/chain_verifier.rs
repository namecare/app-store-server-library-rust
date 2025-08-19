use crate::chain_verifier::ChainVerificationFailureReason::{
    CertificateExpired, InvalidCertificate, InvalidChainLength, InvalidEffectiveDate,
};
use thiserror::Error;

use x509_parser::certificate::X509Certificate;
use x509_parser::der_parser::asn1_rs::oid;
use x509_parser::error::X509Error;
use x509_parser::prelude::{ASN1Time, FromDer};

#[derive(Error, Debug, PartialEq)]
pub enum ChainVerifierError {
    #[error("VerificationFailure: [{0}]")]
    VerificationFailure(ChainVerificationFailureReason),

    #[error("InternalX509Error: [{0}]")]
    InternalX509Error(#[from] X509Error),

    #[error("InternalDecodeError: [{0}]")]
    InternalDecodeError(#[from] base64::DecodeError),
}

#[derive(Error, Debug, PartialEq)]
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
    CertificateExpired,

    #[error("CertificateRevoked")]
    CertificateRevoked,
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
pub fn verify_chain(
    certificates: &Vec<Vec<u8>>,
    root_certificates: &Vec<Vec<u8>>,
    effective_date: Option<u64>,
) -> Result<Vec<u8>, ChainVerifierError> {
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
            Err(_) => continue,
        }

        root_certificate = Some(cert.1)
    }

    let Some(root_certificate) = root_certificate else {
        return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
    };

    verify_chain_impl(
        &leaf_certificate,
        &intermediate_certificate,
        &root_certificate,
        effective_date,
    )
}

fn verify_chain_impl(
    leaf: &X509Certificate,
    intermediate: &X509Certificate,
    root_certificate: &X509Certificate,
    effective_date: Option<u64>,
) -> Result<Vec<u8>, ChainVerifierError> {
    leaf.verify_signature(Some(intermediate.public_key()))?;

    if let Some(date) = effective_date {
        let Ok(time) = ASN1Time::from_timestamp(i64::try_from(date).unwrap()) else {
            return Err(ChainVerifierError::VerificationFailure(
                InvalidEffectiveDate,
            ));
        };

        if !(root_certificate.validity.is_valid_at(time) &&
            leaf.validity.is_valid_at(time) &&
            intermediate.validity.is_valid_at(time))
        {
            return Err(ChainVerifierError::VerificationFailure(CertificateExpired));
        }
    }

    let k = leaf.public_key().raw.to_vec();

    // Make online verification as additional step if ocsp flag enabled
    #[cfg(all(feature = "ocsp"))]
    {
        use crate::chain_verifier_ocsp::check_ocsp_status;
        // Perform OCSP check - this is best-effort, so we don't fail on OCSP errors
        match check_ocsp_status(leaf, intermediate) {
            Ok(()) => {
                // Certificate is valid according to OCSP
            }
            Err(ChainVerifierError::VerificationFailure(ChainVerificationFailureReason::CertificateRevoked)) => {
                // Certificate is revoked - this should fail
                return Err(ChainVerifierError::VerificationFailure(
                    ChainVerificationFailureReason::CertificateRevoked,
                ));
            }
            Err(e) => {
                // Other OCSP errors (network, parsing, etc.) - log but don't fail
                eprintln!("OCSP check failed (non-fatal): {:?}", e);
            }
        }
    };

    Ok(k)
}
