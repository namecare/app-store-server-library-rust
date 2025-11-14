use serde::{Deserialize, Serialize};
use uuid::Uuid;

const MAXIMUM_ALT_TEXT_LENGTH: usize = 150;

/// The definition of an image with its alternative text.
///
/// [UploadMessageImage](https://developer.apple.com/documentation/retentionmessaging/uploadmessageimage)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct UploadMessageImage {
    /// The unique identifier of an image.
    ///
    /// [imageIdentifier](https://developer.apple.com/documentation/retentionmessaging/imageidentifier)
    #[serde(rename = "imageIdentifier")]
    pub image_identifier: Uuid,

    /// The alternative text you provide for the corresponding image.
    ///
    /// [altText](https://developer.apple.com/documentation/retentionmessaging/alttext)
    #[serde(rename = "altText")]
    pub alt_text: String,
}

impl UploadMessageImage {
    /// Creates a new UploadMessageImage with validation.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::AltTextTooLong` if alt_text exceeds 150 characters.
    pub fn new(image_identifier: Uuid, alt_text: String) -> Result<Self, ValidationError> {
        if alt_text.len() > MAXIMUM_ALT_TEXT_LENGTH {
            return Err(ValidationError::AltTextTooLong);
        }
        Ok(Self {
            image_identifier,
            alt_text,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    AltTextTooLong,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::AltTextTooLong => {
                write!(f, "Alt text exceeds maximum length of {} characters", MAXIMUM_ALT_TEXT_LENGTH)
            }
        }
    }
}

impl std::error::Error for ValidationError {}