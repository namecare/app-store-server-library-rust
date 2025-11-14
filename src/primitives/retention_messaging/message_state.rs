use serde::{Deserialize, Serialize};

/// The approval state of the message.
///
/// [messageState](https://developer.apple.com/documentation/retentionmessaging/messagestate)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum MessageState {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "APPROVED")]
    Approved,
    #[serde(rename = "REJECTED")]
    Rejected,
}