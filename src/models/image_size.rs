use serde::{Deserialize, Serialize};

/// The size of an image you upload for use in retention messages.
///
/// [imageSize](https://developer.apple.com/documentation/retentionmessaging/imagesize)
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum ImageSize {
    /// A full-size image for use as a message's main image.
    #[serde(rename = "FULL_SIZE")]
    FullSize,

    /// A small image for use as a bullet point.
    #[serde(rename = "BULLET_POINT")]
    BulletPoint,
}

impl ImageSize {
    pub fn raw_value(&self) -> &str {
        match self {
            ImageSize::FullSize => "FULL_SIZE",
            ImageSize::BulletPoint => "BULLET_POINT",
        }
    }
}