use std::fmt;

use crate::api_client::transport::TransportError;

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

#[derive(Debug, Clone)]
pub struct ApiClientError {
    status: u16,
    raw_code: Option<i64>,
    message: Option<String>,
}

impl ApiClientError {
    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn raw_code(&self) -> Option<i64> {
        self.raw_code
    }

    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}

impl std::error::Error for ApiClientError {}

impl fmt::Display for ApiClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "APIException: HTTP Status Code {}", self.status)?;
        if let Some(raw_code) = self.raw_code {
            write!(f, ", Raw API Error: {}", raw_code)?;
        }
        if let Some(message) = &self.message {
            write!(f, ", Error Message: {}", message)?;
        }
        Ok(())
    }
}

impl ApiClientError {
    pub(crate) fn new(status: u16, raw_code: Option<i64>, message: Option<String>) -> Self {
        Self {
            status,
            raw_code,
            message,
        }
    }
}

impl From<http::Error> for ApiClientError {
    fn from(e: http::Error) -> Self {
        use http::{header, method, status, uri};

        let (status, message) = if e.is::<status::InvalidStatusCode>() {
            (500, "Invalid status code")
        } else if e.is::<method::InvalidMethod>() {
            (400, "Invalid HTTP method")
        } else if e.is::<uri::InvalidUri>() {
            (400, "Invalid URI")
        } else if e.is::<header::InvalidHeaderName>() || e.is::<header::InvalidHeaderValue>() {
            (400, "Invalid header")
        } else if e.is::<header::MaxSizeReached>() {
            (431, "Request header fields too large")
        } else {
            (500, "Unknown HTTP error")
        };

        Self::new(status, None, Some(format!("{}: {}", message, e)))
    }
}

impl From<TransportError> for ApiClientError {
    fn from(err: TransportError) -> Self {
        match err {
            TransportError::Serialization(e) => Self::new(400, None, Some(format!("Serialization error: {}", e))),
            TransportError::InvalidMethod => Self::new(400, None, Some("Invalid HTTP method".to_string())),
            TransportError::InvalidStatusCode(e) => Self::new(500, None, Some(format!("Invalid status code: {}", e))),
            TransportError::RequestFailed(msg) => Self::new(500, None, Some(format!("Request failed: {}", msg))),
            TransportError::NetworkError(msg) => Self::new(503, None, Some(format!("Network error: {}", msg))),
            TransportError::InvalidResponse(msg) => Self::new(502, None, Some(format!("Invalid response: {}", msg))),
            TransportError::Timeout => Self::new(504, None, Some("Request timeout".to_string())),
            TransportError::Other(msg) => Self::new(500, None, Some(format!("Unexpected error: {}", msg))),
        }
    }
}
