use crate::primitives::retention_messaging::image_state::ImageState;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// An image identifier and state information for an image.
///
/// [GetImageListResponseItem](https://developer.apple.com/documentation/retentionmessaging/getimagelistresponseitem)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct GetImageListResponseItem {
    /// The identifier of the image.
    ///
    /// [imageIdentifier](https://developer.apple.com/documentation/retentionmessaging/imageidentifier)
    #[serde(rename = "imageIdentifier")]
    pub image_identifier: Option<Uuid>,

    /// The current state of the image.
    ///
    /// [imageState](https://developer.apple.com/documentation/retentionmessaging/imagestate)
    #[serde(rename = "imageState")]
    pub image_state: Option<ImageState>,
}