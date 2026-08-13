use serde::{Deserialize, Serialize};

/// The approval state of an image.
///
/// [imageState](https://developer.apple.com/documentation/retentionmessaging/imagestate)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum ImageState {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "APPROVED")]
    Approved,
    #[serde(rename = "REJECTED")]
    Rejected,

    /// A value the App Store sent that this version of the
    /// library does not support, preserved as received.
    #[serde(untagged)]
    NotSupported(String),
}
