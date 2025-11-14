use crate::primitives::retention_messaging::upload_message_image::UploadMessageImage;
use serde::{Deserialize, Serialize};

const MAXIMUM_HEADER_LENGTH: usize = 66;
const MAXIMUM_BODY_LENGTH: usize = 144;

/// The request body for uploading a message, which includes the message text and an optional image reference.
///
/// [UploadMessageRequestBody](https://developer.apple.com/documentation/retentionmessaging/uploadmessagerequestbody)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct UploadMessageRequestBody {
    /// The header text of the retention message that the system displays to customers.
    ///
    /// [header](https://developer.apple.com/documentation/retentionmessaging/header)
    pub header: String,

    /// The body text of the retention message that the system displays to customers.
    ///
    /// [body](https://developer.apple.com/documentation/retentionmessaging/body)
    pub body: String,

    /// The optional image identifier and its alternative text to appear as part of a text-based message with an image.
    ///
    /// [UploadMessageImage](https://developer.apple.com/documentation/retentionmessaging/uploadmessageimage)
    pub image: Option<UploadMessageImage>,
}

impl UploadMessageRequestBody {
    /// Creates a new UploadMessageRequestBody with validation.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::HeaderTooLong` if header exceeds 66 characters.
    /// Returns `ValidationError::BodyTooLong` if body exceeds 144 characters.
    pub fn new(header: String, body: String, image: Option<UploadMessageImage>) -> Result<Self, ValidationError> {
        if header.len() > MAXIMUM_HEADER_LENGTH {
            return Err(ValidationError::HeaderTooLong);
        }
        if body.len() > MAXIMUM_BODY_LENGTH {
            return Err(ValidationError::BodyTooLong);
        }
        Ok(Self { header, body, image })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    HeaderTooLong,
    BodyTooLong,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::HeaderTooLong => {
                write!(f, "Header exceeds maximum length of {} characters", MAXIMUM_HEADER_LENGTH)
            }
            ValidationError::BodyTooLong => {
                write!(f, "Body exceeds maximum length of {} characters", MAXIMUM_BODY_LENGTH)
            }
        }
    }
}

impl std::error::Error for ValidationError {}