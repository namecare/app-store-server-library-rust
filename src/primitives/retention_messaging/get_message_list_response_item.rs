use crate::primitives::retention_messaging::message_state::MessageState;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A message identifier and status information for a message.
///
/// [GetMessageListResponseItem](https://developer.apple.com/documentation/retentionmessaging/getmessagelistresponseitem)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct GetMessageListResponseItem {
    /// The identifier of the message.
    ///
    /// [messageIdentifier](https://developer.apple.com/documentation/retentionmessaging/messageidentifier)
    #[serde(rename = "messageIdentifier")]
    pub message_identifier: Option<Uuid>,

    /// The current state of the message.
    ///
    /// [messageState](https://developer.apple.com/documentation/retentionmessaging/messagestate)
    #[serde(rename = "messageState")]
    pub message_state: Option<MessageState>,
}