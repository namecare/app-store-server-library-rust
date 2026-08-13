use serde::{Deserialize, Serialize};

/// The position of the header relative to the body and image in a retention message.
///
/// [headerPosition](https://developer.apple.com/documentation/retentionmessaging/headerposition)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum HeaderPosition {
    /// The header appears above the body text.
    #[serde(rename = "ABOVE_BODY")]
    AboveBody,

    /// The header appears above the image.
    #[serde(rename = "ABOVE_IMAGE")]
    AboveImage,

    /// A value the App Store sent that this version of the
    /// library does not support, preserved as received.
    #[serde(untagged)]
    NotSupported(String),
}
