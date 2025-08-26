use std::fmt;
use std::fmt::Debug;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::DeserializeOwned;
use crate::api_client::transport::TransportError;

#[derive(Debug, Clone, Serialize, Hash)]
pub struct ErrorPayload<E: APIServiceErrorCode> {
    #[serde(rename = "errorCode")]
    pub error_code: E,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_error_code: Option<i64>,

    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
}

impl<'de, E> Deserialize<'de> for ErrorPayload<E>
where
    E: APIServiceErrorCode + DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct _ErrorPayload {
            #[serde(rename = "errorCode")]
            error_code: Option<i64>,

            #[serde(rename = "errorMessage")]
            error_message: Option<String>,
        }

        let helper = _ErrorPayload::deserialize(deserializer)?;
        let raw_code = helper.error_code;

        let api_error_code = {
            match raw_code {
                Some(code) => {
                    serde_json::to_value(code)
                        .and_then(|v| serde_json::from_value::<E>(v))
                        .unwrap_or_else(|_| E::unknown())
                },
                None => E::unknown()
            }
        };

        Ok(ErrorPayload {
            error_code: api_error_code,
            raw_error_code: helper.error_code,
            error_message: helper.error_message,
        })
    }
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

pub trait APIServiceErrorCode: Debug + Sized {
    fn code(&self) -> i64;
    fn unknown() -> Self;
}

#[derive(Debug, Clone)]
pub struct ApiServiceError<E: APIServiceErrorCode> {
    pub http_status_code: u16,
    pub api_error: Option<E>,
    pub error_code: Option<i64>,
    pub error_message: Option<String>,
}

impl<E: APIServiceErrorCode> std::error::Error for ApiServiceError<E> {}
impl<E: APIServiceErrorCode> fmt::Display for ApiServiceError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "APIException: HTTP Status Code {}",
            self.http_status_code
        )?;
        if let Some(api_error) = &self.api_error {
            write!(f, ", API Error: {:?}", api_error)?;
        }
        if let Some(raw_api_error) = &self.error_code {
            write!(f, ", Raw API Error: {}", raw_api_error)?;
        }
        if let Some(error_message) = &self.error_message {
            write!(f, ", Error Message: {}", error_message)?;
        }
        Ok(())
    }
}

impl<E: APIServiceErrorCode> From<http::Error> for ApiServiceError<E> {
    fn from(e: http::Error) -> Self {
        use http::{header, method, status, uri};

        let (http_status_code, error_message) = if e.is::<status::InvalidStatusCode>() {
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

        Self {
            http_status_code,
            api_error: None,
            error_code: None,
            error_message: Some(format!("{}: {}", error_message, e)),
        }
    }
}

impl<E: APIServiceErrorCode> From<TransportError> for ApiServiceError<E> {
    fn from(err: TransportError) -> Self {
        match err {
            TransportError::Serialization(e) => ApiServiceError {
                http_status_code: 400,
                api_error: None,
                error_code: None,
                error_message: Some(format!("Serialization error: {}", e)),
            },
            TransportError::InvalidMethod => ApiServiceError {
                http_status_code: 400,
                api_error: None,
                error_code: None,
                error_message: Some("Invalid HTTP method".to_string()),
            },
            TransportError::InvalidStatusCode(e) => ApiServiceError {
                http_status_code: 500,
                api_error: None,
                error_code: None,
                error_message: Some(format!("Invalid status code: {}", e)),
            },
            TransportError::RequestFailed(msg) => ApiServiceError {
                http_status_code: 500,
                api_error: None,
                error_code: None,
                error_message: Some(format!("Request failed: {}", msg)),
            },
            TransportError::NetworkError(msg) => ApiServiceError {
                http_status_code: 503,
                api_error: None,
                error_code: None,
                error_message: Some(format!("Network error: {}", msg)),
            },
            TransportError::InvalidResponse(msg) => ApiServiceError {
                http_status_code: 502,
                api_error: None,
                error_code: None,
                error_message: Some(format!("Invalid response: {}", msg)),
            },
            TransportError::Timeout => ApiServiceError {
                http_status_code: 504,
                api_error: None,
                error_code: None,
                error_message: Some("Request timeout".to_string()),
            },
            TransportError::Other(msg) => ApiServiceError {
                http_status_code: 500,
                api_error: None,
                error_code: None,
                error_message: Some(format!("Unexpected error: {}", msg)),
            },
        }
    }
}
