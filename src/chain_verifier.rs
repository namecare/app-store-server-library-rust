use crate::x509::x509::X509Error;
use crate::chain_verifier::ChainVerificationFailureReason::{
    CertificateExpired, InvalidCertificate, InvalidEffectiveDate,
};
use thiserror::Error;

use x509_cert::Certificate;
use const_oid::ObjectIdentifier;
use crate::x509::x509;

#[derive(Error, Debug, PartialEq)]
pub enum ChainVerifierError {
    #[error("VerificationFailure: [{0}]")]
    VerificationFailure(ChainVerificationFailureReason),

    #[error("InternalX509Error: [{0}]")]
    InternalX509Error(String),

    #[error("InternalDecodeError: [{0}]")]
    InternalDecodeError(#[from] base64::DecodeError),
}

impl From<X509Error> for ChainVerifierError {
    fn from(err: X509Error) -> Self {
        ChainVerifierError::InternalX509Error(err.to_string())
    }
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

        let leaf_certificate = x509::parse_certificate(leaf_certificate.as_slice())
            .map_err(|_| ChainVerifierError::VerificationFailure(InvalidCertificate))?;

        // Check for Apple-specific leaf certificate extension (1.2.840.113635.100.6.11.1)
        let leaf_oid = ObjectIdentifier::new("1.2.840.113635.100.6.11.1")
            .map_err(|_| ChainVerifierError::VerificationFailure(InvalidCertificate))?;

        if !x509::has_extension(&leaf_certificate, &leaf_oid) {
            return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
        }

        let intermediate_certificate = x509::parse_certificate(intermediate_certificate.as_slice())
            .map_err(|_| ChainVerifierError::VerificationFailure(InvalidCertificate))?;

        // Check for Apple-specific intermediate certificate extension (1.2.840.113635.100.6.2.1)
        let intermediate_oid = ObjectIdentifier::new("1.2.840.113635.100.6.2.1")
            .map_err(|_| ChainVerifierError::VerificationFailure(InvalidCertificate))?;

        if !x509::has_extension(&intermediate_certificate, &intermediate_oid) {
            return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
        }

        let mut root_certificate: Option<Certificate> = None;

        for cert in &self.root_certificates {
            let cert = x509::parse_certificate(&cert)
                .map_err(|_| ChainVerifierError::VerificationFailure(InvalidCertificate))?;

            if x509::verify_signature(&intermediate_certificate, &cert).is_ok() {
                root_certificate = Some(cert);
                break;
            }
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
        leaf: &Certificate,
        intermediate: &Certificate,
        root_certificate: &Certificate,
        effective_date: Option<u64>,
    ) -> Result<Vec<u8>, ChainVerifierError> {
        x509::verify_signature(leaf, intermediate)?;

        if let Some(date) = effective_date {
            let timestamp = i64::try_from(date)
                .map_err(|_| ChainVerifierError::VerificationFailure(InvalidEffectiveDate))?;

            if !x509::is_valid_at(leaf, timestamp) ||
               !x509::is_valid_at(intermediate, timestamp) ||
               !x509::is_valid_at(root_certificate, timestamp)
            {
                return Err(ChainVerifierError::VerificationFailure(CertificateExpired));
            }
        }

        let public_key_bytes = x509::public_key_bytes(leaf);

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

        Ok(public_key_bytes)
    }
}