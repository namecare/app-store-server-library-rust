use serde::{Deserialize, Serialize};
use uuid::Uuid;

const MAXIMUM_TEXT_LENGTH: usize = 66;
const MAXIMUM_ALT_TEXT_LENGTH: usize = 150;

/// The text and its bullet-point image to include in a retention message's bulleted list.
///
/// [BulletPoint](https://developer.apple.com/documentation/retentionmessaging/bulletpoint)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BulletPoint {
    /// The text of the individual bullet point.
    ///
    /// [text](https://developer.apple.com/documentation/retentionmessaging/text)
    pub text: String,

    /// The identifier of the image to use as the bullet point.
    ///
    /// [imageIdentifier](https://developer.apple.com/documentation/retentionmessaging/imageidentifier)
    pub image_identifier: Uuid,

    /// The alternative text you provide for the corresponding image of the bullet point.
    ///
    /// [altText](https://developer.apple.com/documentation/retentionmessaging/alttext)
    pub alt_text: String,
}

impl BulletPoint {
    /// Creates a new `BulletPoint`, validating the text lengths.
    ///
    /// # Errors
    ///
    /// Returns `BulletPointValidationError::TextTooLong` if `text` exceeds 66 characters.
    /// Returns `BulletPointValidationError::AltTextTooLong` if `alt_text` exceeds 150 characters.
    pub fn new(
        text: String,
        image_identifier: Uuid,
        alt_text: String,
    ) -> Result<Self, BulletPointValidationError> {
        if text.chars().count() > MAXIMUM_TEXT_LENGTH {
            return Err(BulletPointValidationError::TextTooLong);
        }
        if alt_text.chars().count() > MAXIMUM_ALT_TEXT_LENGTH {
            return Err(BulletPointValidationError::AltTextTooLong);
        }
        Ok(Self {
            text,
            image_identifier,
            alt_text,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BulletPointValidationError {
    TextTooLong,
    AltTextTooLong,
}

impl std::fmt::Display for BulletPointValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BulletPointValidationError::TextTooLong => write!(
                f,
                "Text exceeds maximum length of {} characters",
                MAXIMUM_TEXT_LENGTH
            ),
            BulletPointValidationError::AltTextTooLong => write!(
                f,
                "Alt text exceeds maximum length of {} characters",
                MAXIMUM_ALT_TEXT_LENGTH
            ),
        }
    }
}

impl std::error::Error for BulletPointValidationError {}