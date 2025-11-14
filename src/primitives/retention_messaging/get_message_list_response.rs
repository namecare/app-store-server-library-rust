use crate::primitives::retention_messaging::get_message_list_response_item::GetMessageListResponseItem;
use serde::{Deserialize, Serialize};

/// A response that contains status information for all messages.
///
/// [GetMessageListResponse](https://developer.apple.com/documentation/retentionmessaging/getmessagelistresponse)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct GetMessageListResponse {
    /// An array of all message identifiers and their message state.
    ///
    /// [messageIdentifiers](https://developer.apple.com/documentation/retentionmessaging/getmessagelistresponseitem)
    #[serde(rename = "messageIdentifiers")]
    pub message_identifiers: Option<Vec<GetMessageListResponseItem>>,
}