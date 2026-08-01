//! Pluggable HTTP transport for OCSP (Online Certificate Status Protocol) requests.

use thiserror::Error;

/// Performs the HTTP transport for an OCSP request/response exchange.
///
/// Implementations POST a DER-encoded OCSP request to `responder_url` and
/// return the DER-encoded OCSP response bytes. `SignedDataVerifier::new`
/// requires one. A default blocking-`reqwest` implementation is available
/// whenever `api-client-reqwest` or `api-client-reqwest-native-tls` is
/// enabled (reusing that dependency rather than adding a new one) — see
/// [`crate::ocsp_requester::reqwest_requester::ReqwestOCSPRequester`].
/// With neither feature enabled, callers must supply their own
/// `OCSPRequester` implementation.
pub trait OCSPRequester: Send + Sync {
    /// * `request` - DER-encoded OCSP request bytes.
    /// * `responder_url` - The OCSP responder URI taken from the certificate's
    ///   Authority Information Access extension.
    fn query(&self, request: &[u8], responder_url: &str) -> Result<Vec<u8>, OCSPRequesterError>;
}

#[derive(Error, Debug)]
pub enum OCSPRequesterError {
    #[error("NetworkError: [{0}]")]
    Network(String),

    #[error("HttpError: [{0}]")]
    Http(u16),

    #[error("ReadBodyError: [{0}]")]
    ReadBody(String),
}

#[cfg(any(feature = "api-client-reqwest", feature = "api-client-reqwest-native-tls"))]
pub mod reqwest_requester;