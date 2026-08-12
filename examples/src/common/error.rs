//! One error type, mapped once to HTTP status codes for both frameworks.

use app_store_server_library::signed_data_verifier::SignedDataVerifierError;

#[derive(Debug)]
pub enum AppError {
    /// The payload failed cryptographic or business-rule verification.
    Unauthorized(String),
    /// The request body was missing or malformed.
    BadRequest(String),
    /// Something failed on our side.
    Internal(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Unauthorized(message) => write!(f, "{}", message),
            AppError::BadRequest(message) => write!(f, "{}", message),
            AppError::Internal(message) => write!(f, "{}", message),
        }
    }
}

impl std::error::Error for AppError {}

impl From<SignedDataVerifierError> for AppError {
    fn from(error: SignedDataVerifierError) -> Self {
        // Every verifier failure is a rejected payload from the caller's
        // perspective, so they all map to 401.
        AppError::Unauthorized(format!("signed payload verification failed: {:?}", error))
    }
}

/// The HTTP status this error maps to.
pub fn status_code(error: &AppError) -> u16 {
    match error {
        AppError::Unauthorized(_) => 401,
        AppError::BadRequest(_) => 400,
        AppError::Internal(_) => 500,
    }
}

fn body(error: &AppError) -> String {
    serde_json::json!({ "error": error.to_string() }).to_string()
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = axum::http::StatusCode::from_u16(status_code(&self))
            .unwrap_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR);

        (
            status,
            [(axum::http::header::CONTENT_TYPE, "application/json")],
            body(&self),
        )
            .into_response()
    }
}

impl actix_web::ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::from_u16(status_code(self))
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(actix_web::ResponseError::status_code(self))
            .content_type("application/json")
            .body(body(self))
    }
}
