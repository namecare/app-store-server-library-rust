use serde::{Deserialize, Serialize};

use crate::models::bullet_point::BulletPoint;
use crate::models::header_position::HeaderPosition;
use crate::models::upload_message_image::UploadMessageImage;

const MAXIMUM_HEADER_LENGTH: usize = 66;
const MAXIMUM_BODY_LENGTH: usize = 144;
const MAXIMUM_BULLET_POINTS_COUNT: usize = 5;

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

    /// The bulleted list to display as part of the retention message.
    ///
    /// [bulletPoints](https://developer.apple.com/documentation/retentionmessaging/bulletpoints)
    #[serde(rename = "bulletPoints")]
    pub bullet_points: Option<Vec<BulletPoint>>,

    /// The position of the header relative to the body and image.
    ///
    /// [headerPosition](https://developer.apple.com/documentation/retentionmessaging/headerposition)
    #[serde(rename = "headerPosition")]
    pub header_position: Option<HeaderPosition>,
}

impl UploadMessageRequestBody {
    /// Creates a new UploadMessageRequestBody with validation.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::HeaderTooLong` if header exceeds 66 characters.
    /// Returns `ValidationError::BodyTooLong` if body exceeds 144 characters.
    /// Returns `ValidationError::TooManyBulletPoints` if more than 5 bullet points are provided.
    pub fn new(
        header: String,
        body: String,
        image: Option<UploadMessageImage>,
        bullet_points: Option<Vec<BulletPoint>>,
        header_position: Option<HeaderPosition>,
    ) -> Result<Self, ValidationError> {
        if header.chars().count() > MAXIMUM_HEADER_LENGTH {
            return Err(ValidationError::HeaderTooLong);
        }
        if body.chars().count() > MAXIMUM_BODY_LENGTH {
            return Err(ValidationError::BodyTooLong);
        }
        if let Some(bullet_points) = &bullet_points {
            if bullet_points.len() > MAXIMUM_BULLET_POINTS_COUNT {
                return Err(ValidationError::TooManyBulletPoints);
            }
        }
        Ok(Self {
            header,
            body,
            image,
            bullet_points,
            header_position,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    HeaderTooLong,
    BodyTooLong,
    TooManyBulletPoints,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::HeaderTooLong => {
                write!(
                    f,
                    "Header exceeds maximum length of {} characters",
                    MAXIMUM_HEADER_LENGTH
                )
            }
            ValidationError::BodyTooLong => {
                write!(
                    f,
                    "Body exceeds maximum length of {} characters",
                    MAXIMUM_BODY_LENGTH
                )
            }
            ValidationError::TooManyBulletPoints => {
                write!(
                    f,
                    "Bullet points exceed maximum count of {}",
                    MAXIMUM_BULLET_POINTS_COUNT
                )
            }
        }
    }
}

impl std::error::Error for ValidationError {}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn bullet_point() -> BulletPoint {
        BulletPoint::new("text".to_string(), Uuid::new_v4(), "alt".to_string()).unwrap()
    }

    #[test]
    fn test_header_and_body_count_characters_not_bytes() {
        // "日" is 3 bytes: a byte-based check would reject these at a third of the real limit.
        let header = "日".repeat(MAXIMUM_HEADER_LENGTH);
        let body = "日".repeat(MAXIMUM_BODY_LENGTH);
        assert_eq!(header.len(), MAXIMUM_HEADER_LENGTH * 3);
        assert_eq!(body.len(), MAXIMUM_BODY_LENGTH * 3);
        assert!(UploadMessageRequestBody::new(header, body, None, None, None).is_ok());
    }

    #[test]
    fn test_header_too_long() {
        let header = "日".repeat(MAXIMUM_HEADER_LENGTH + 1);
        assert_eq!(
            UploadMessageRequestBody::new(header, "body".to_string(), None, None, None),
            Err(ValidationError::HeaderTooLong)
        );
    }

    #[test]
    fn test_body_too_long() {
        let body = "日".repeat(MAXIMUM_BODY_LENGTH + 1);
        assert_eq!(
            UploadMessageRequestBody::new("header".to_string(), body, None, None, None),
            Err(ValidationError::BodyTooLong)
        );
    }

    #[test]
    fn test_bullet_points_at_maximum_allowed() {
        let bullet_points = vec![bullet_point(); MAXIMUM_BULLET_POINTS_COUNT];
        assert!(UploadMessageRequestBody::new(
            "header".to_string(),
            "body".to_string(),
            None,
            Some(bullet_points),
            None,
        )
        .is_ok());
    }

    #[test]
    fn test_too_many_bullet_points() {
        let bullet_points = vec![bullet_point(); MAXIMUM_BULLET_POINTS_COUNT + 1];
        assert_eq!(
            UploadMessageRequestBody::new(
                "header".to_string(),
                "body".to_string(),
                None,
                Some(bullet_points),
                None,
            ),
            Err(ValidationError::TooManyBulletPoints)
        );
    }

    #[test]
    fn test_bullet_points_absent_is_allowed() {
        assert!(UploadMessageRequestBody::new(
            "header".to_string(),
            "body".to_string(),
            None,
            None,
            None,
        )
        .is_ok());
    }
}
