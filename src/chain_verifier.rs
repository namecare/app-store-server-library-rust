use crate::chain_verifier::ChainVerificationFailureReason::{
    CertificateExpired, InvalidCertificate, InvalidEffectiveDate,
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

    #[error("RetryableVerificationFailure")]
    RetryableVerificationFailure,
}

/// A structure for verifying certificate chains.
///
/// This struct holds the root certificates and provides methods to verify certificate chains.
pub struct ChainVerifier {
    root_certificates: Vec<Vec<u8>>,
}

impl ChainVerifier {
    /// Creates a new `ChainVerifier` with the provided root certificates.
    ///
    /// # Arguments
    ///
    /// * `root_certificates`: A vector of byte slices containing the root certificates.
    ///
    /// # Returns
    ///
    /// A new instance of `ChainVerifier`.
    pub fn new(root_certificates: Vec<Vec<u8>>) -> Self {
        ChainVerifier { root_certificates }
    }

    /// Verifies a certificate pair (leaf and intermediate).
    ///
    /// This method verifies a leaf certificate against an intermediate certificate. It performs various
    /// checks to ensure the validity and integrity of the certificates.
    ///
    /// # Arguments
    ///
    /// * `leaf_certificate`: The leaf certificate as a byte slice.
    /// * `intermediate_certificate`: The intermediate certificate as a byte slice.
    /// * `effective_date`: An optional Unix timestamp representing the effective date for the chain validation.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<u8>)`: If the certificates are valid, it returns the public key data from the leaf certificate.
    /// * `Err(ChainVerifierError)`: If the verification fails for any reason, it returns a `ChainVerifierError` enum.
    /// TODO: Implement issuer checking
    pub fn verify(
        &self,
        leaf_certificate: &Vec<u8>,
        intermediate_certificate: &Vec<u8>,
        effective_date: Option<u64>,
    ) -> Result<Vec<u8>, ChainVerifierError> {
        if self.root_certificates.is_empty() {
            return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
        }
        let Ok(leaf_certificate) = X509Certificate::from_der(leaf_certificate.as_slice()) else {
            return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
        };
        let leaf_certificate = leaf_certificate.1;

        let Some(_) = leaf_certificate.get_extension_unique(&oid!(1.2.840.113635.100.6.11.1))? else {
            return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
        };

        let Ok(intermediate_certificate) = X509Certificate::from_der(intermediate_certificate.as_slice()) else {
            return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
        };
        let intermediate_certificate = intermediate_certificate.1;

        let Some(_) = intermediate_certificate.get_extension_unique(&oid!(1.2.840.113635.100.6.2.1))? else {
            return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
        };

        let mut root_certificate: Option<X509Certificate> = None;

        for cert in &self.root_certificates {
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

        self.verify_chain(
            &leaf_certificate,
            &intermediate_certificate,
            &root_certificate,
            effective_date,
        )
    }

    fn verify_chain(
        &self,
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
            // Perform OCSP check - this is best-effort, so we don't fail on OCSP errors
            match self.check_ocsp_status(leaf, intermediate) {
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
}