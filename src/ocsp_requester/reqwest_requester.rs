//! Default blocking-`reqwest` implementation of [`OCSPRequester`].

use crate::ocsp_requester::{OCSPRequester, OCSPRequesterError};

/// Default [`OCSPRequester`] implementation using `reqwest`'s blocking client.
///
/// Builds a fresh blocking client per call with a 5 second timeout, matching
/// the timeout used by the Swift and Python reference implementations'
/// default OCSP requesters.
pub struct ReqwestOCSPRequester;

impl OCSPRequester for ReqwestOCSPRequester {
    fn query(&self, request: &[u8], responder_url: &str) -> Result<Vec<u8>, OCSPRequesterError> {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .map_err(|e| OCSPRequesterError::Network(format!("Failed to build HTTP client: {}", e)))?;

        let response = client
            .post(responder_url)
            .header("Content-Type", "application/ocsp-request")
            .body(request.to_vec())
            .send()
            .map_err(|e| OCSPRequesterError::Network(format!("OCSP request failed: {}", e)))?;

        let status = response.status();
        if !status.is_success() {
            return Err(OCSPRequesterError::Http(status.as_u16()));
        }

        let bytes = response
            .bytes()
            .map_err(|e| OCSPRequesterError::ReadBody(e.to_string()))?;

        Ok(bytes.to_vec())
    }
}