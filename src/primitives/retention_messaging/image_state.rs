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
}