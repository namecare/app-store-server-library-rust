use std::fmt;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use crate::api_client::transport::TransportError;

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct ErrorPayload<E: APIServiceErrorCode> {
    #[serde(rename = "errorCode")]
    pub error_code: Option<E>,

    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
}

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

pub trait APIServiceErrorCode: Debug {
    fn code(&self) -> i64;
}

#[derive(Debug, Clone)]
pub struct APIServiceError<E: APIServiceErrorCode> {
    pub http_status_code: u16,
    pub api_error: Option<E>,
    pub error_message: Option<String>,
}

impl<E: APIServiceErrorCode> APIServiceError<E> {
    pub fn raw_api_error(&self) -> Option<i64> {
        self.api_error.as_ref().map(|err| err.code())
    }
}

impl<E: APIServiceErrorCode> std::error::Error for APIServiceError<E> {}
impl<E: APIServiceErrorCode> fmt::Display for APIServiceError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "APIException: HTTP Status Code {}",
            self.http_status_code
        )?;
        if let Some(api_error) = &self.api_error {
            write!(f, ", API Error: {:?}", api_error)?;
        }
        if let Some(raw_api_error) = &self.raw_api_error() {
            write!(f, ", Raw API Error: {}", raw_api_error)?;
        }
        if let Some(error_message) = &self.error_message {
            write!(f, ", Error Message: {}", error_message)?;
        }
        Ok(())
    }
}

impl<E: APIServiceErrorCode> From<TransportError> for APIServiceError<E> {
    fn from(err: TransportError) -> Self {
        match err {
            TransportError::Serialization(e) => APIServiceError {
                http_status_code: 400,
                api_error: None,
                error_message: Some(format!("Serialization error: {}", e)),
            },
            TransportError::InvalidMethod => APIServiceError {
                http_status_code: 400,
                api_error: None,
                error_message: Some("Invalid HTTP method".to_string()),
            },
            TransportError::InvalidStatusCode(e) => APIServiceError {
                http_status_code: 500,
                api_error: None,
                error_message: Some(format!("Invalid status code: {}", e)),
            },
            TransportError::RequestFailed(msg) => APIServiceError {
                http_status_code: 500,
                api_error: None,
                error_message: Some(format!("Request failed: {}", msg)),
            },
            TransportError::NetworkError(msg) => APIServiceError {
                http_status_code: 503,
                api_error: None,
                error_message: Some(format!("Network error: {}", msg)),
            },
            TransportError::InvalidResponse(msg) => APIServiceError {
                http_status_code: 502,
                api_error: None,
                error_message: Some(format!("Invalid response: {}", msg)),
            },
            TransportError::Timeout => APIServiceError {
                http_status_code: 504,
                api_error: None,
                error_message: Some("Request timeout".to_string()),
            },
            TransportError::Other(msg) => APIServiceError {
                http_status_code: 500,
                api_error: None,
                error_message: Some(format!("Unexpected error: {}", msg)),
            },
        }
    }
}
