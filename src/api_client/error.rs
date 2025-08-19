use std::fmt;
use serde::{Deserialize, Serialize};
use crate::api_client::transport::TransportError;
use crate::primitives::error_payload::APIError;

#[derive(Debug)]
pub enum ConfigurationError {
    InvalidEnvironment(String),
}

impl fmt::Display for ConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigurationError::InvalidEnvironment(msg) => write!(f, "Invalid environment: {}", msg),
        }
    }
}

impl std::error::Error for ConfigurationError {}

#[derive(Debug, Serialize, Deserialize)]
pub struct APIException {
    pub http_status_code: u16,
    pub api_error: Option<APIError>,
    #[serde(rename = "errorCode")]
    pub raw_api_error: Option<i64>,
    pub error_message: Option<String>,
}

impl std::error::Error for APIException {}

impl fmt::Display for APIException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "APIException: HTTP Status Code {}",
            self.http_status_code
        )?;
        if let Some(api_error) = &self.api_error {
            write!(f, ", API Error: {:?}", api_error)?;
        }
        if let Some(raw_api_error) = &self.raw_api_error {
            write!(f, ", Raw API Error: {}", raw_api_error)?;
        }
        if let Some(error_message) = &self.error_message {
            write!(f, ", Error Message: {}", error_message)?;
        }
        Ok(())
    }
}

impl From<TransportError> for APIException {
    fn from(err: TransportError) -> Self {
        match err {
            TransportError::Serialization(e) => APIException {
                http_status_code: 400,
                api_error: None,
                raw_api_error: None,
                error_message: Some(format!("Serialization error: {}", e)),
            },
            TransportError::InvalidMethod => APIException {
                http_status_code: 400,
                api_error: None,
                raw_api_error: None,
                error_message: Some("Invalid HTTP method".to_string()),
            },
            TransportError::InvalidStatusCode(e) => APIException {
                http_status_code: 500,
                api_error: None,
                raw_api_error: None,
                error_message: Some(format!("Invalid status code: {}", e)),
            },
            TransportError::RequestFailed(msg) => APIException {
                http_status_code: 500,
                api_error: None,
                raw_api_error: None,
                error_message: Some(format!("Request failed: {}", msg)),
            },
            TransportError::NetworkError(msg) => APIException {
                http_status_code: 503,
                api_error: None,
                raw_api_error: None,
                error_message: Some(format!("Network error: {}", msg)),
            },
            TransportError::InvalidResponse(msg) => APIException {
                http_status_code: 502,
                api_error: None,
                raw_api_error: None,
                error_message: Some(format!("Invalid response: {}", msg)),
            },
            TransportError::Timeout => APIException {
                http_status_code: 504,
                api_error: None,
                raw_api_error: None,
                error_message: Some("Request timeout".to_string()),
            },
            TransportError::Other(msg) => APIException {
                http_status_code: 500,
                api_error: None,
                raw_api_error: None,
                error_message: Some(format!("Unexpected error: {}", msg)),
            },
        }
    }
}
