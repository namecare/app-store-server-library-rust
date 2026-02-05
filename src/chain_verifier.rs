use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ChainVerifierError {
    #[error("VerificationFailure: [{0}]")]
    VerificationFailure(ChainVerificationFailureReason),

    #[error("InternalError: [{0}]")]
    InternalError(String),

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

/// Trait for certificate chain verification
pub trait ChainVerifier: Send + Sync {
    /// Verifies a certificate chain and returns the leaf's public key
    ///
    /// # Arguments
    /// * `leaf_certificate` - DER-encoded leaf certificate
    /// * `intermediate_certificate` - DER-encoded intermediate certificate
    /// * `root_certificates` - List of trusted DER-encoded root certificates
    /// * `effective_date` - Optional Unix timestamp for validity check
    ///
    /// # Returns
    /// The public key bytes from the leaf certificate if verification succeeds
    fn verify(
        &self,
        leaf_certificate: &[u8],
        intermediate_certificate: &[u8],
        root_certificates: &[Vec<u8>],
        effective_date: Option<u64>,
    ) -> Result<Vec<u8>, ChainVerifierError>;
}
