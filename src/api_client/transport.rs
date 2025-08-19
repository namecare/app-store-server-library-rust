use std::future::Future;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransportError {
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Invalid HTTP method")]
    InvalidMethod,

    #[error("Invalid status code: {0}")]
    InvalidStatusCode(#[from] http::status::InvalidStatusCode),

    #[error("Request failed: {0}")]
    RequestFailed(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Timeout error")]
    Timeout,

    #[error("Other error: {0}")]
    Other(String),
}

pub trait Transport: Send + Sync {
    fn send(&self, req: http::Request<Vec<u8>>) -> impl Future<Output = Result<http::Response<Vec<u8>>, TransportError>> + Send;
}