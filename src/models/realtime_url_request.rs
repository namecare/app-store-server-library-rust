use serde::{Deserialize, Serialize};

const MAXIMUM_REALTIME_URL_LENGTH: usize = 256;

/// The request body for configuring the URL of your Get Retention Message endpoint.
///
/// [RealtimeUrlRequest](https://developer.apple.com/documentation/retentionmessaging/realtimeurlrequest)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct RealtimeUrlRequest {
    /// A string that contains the URL of your Get Retention Message endpoint for configuration.
    ///
    /// [realtimeURL](https://developer.apple.com/documentation/retentionmessaging/realtimeurl)
    #[serde(rename = "realtimeURL")]
    pub realtime_url: String,
}

impl RealtimeUrlRequest {
    /// Creates a new `RealtimeUrlRequest`, validating the URL length.
    ///
    /// # Errors
    ///
    /// Returns `RealtimeUrlRequestValidationError::RealtimeUrlTooLong` if the URL
    /// exceeds 256 characters.
    pub fn new(realtime_url: String) -> Result<Self, RealtimeUrlRequestValidationError> {
        if realtime_url.chars().count() > MAXIMUM_REALTIME_URL_LENGTH {
            return Err(RealtimeUrlRequestValidationError::RealtimeUrlTooLong);
        }
        Ok(Self { realtime_url })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RealtimeUrlRequestValidationError {
    RealtimeUrlTooLong,
}

impl std::fmt::Display for RealtimeUrlRequestValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RealtimeUrlRequestValidationError::RealtimeUrlTooLong => write!(
                f,
                "Realtime URL exceeds maximum length of {} characters",
                MAXIMUM_REALTIME_URL_LENGTH
            ),
        }
    }
}

impl std::error::Error for RealtimeUrlRequestValidationError {}
