use serde::{Deserialize, Serialize};

use crate::models::get_image_list_response_item::GetImageListResponseItem;

/// A response that contains status information for all images.
///
/// [GetImageListResponse](https://developer.apple.com/documentation/retentionmessaging/getimagelistresponse)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct GetImageListResponse {
    /// An array of all image identifiers and their image state.
    ///
    /// [GetImageListResponseItem](https://developer.apple.com/documentation/retentionmessaging/getimagelistresponseitem)
    #[serde(rename = "imageIdentifiers")]
    pub image_identifiers: Option<Vec<GetImageListResponseItem>>,
}
