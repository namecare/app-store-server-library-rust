use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A message identifier you provide in a real-time response to your Get Retention Message endpoint.
///
/// [message](https://developer.apple.com/documentation/retentionmessaging/message)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct Message {
    /// The identifier of the message to display to the customer.
    ///
    /// [messageIdentifier](https://developer.apple.com/documentation/retentionmessaging/messageidentifier)
    #[serde(rename = "messageIdentifier")]
    pub message_identifier: Option<Uuid>,
}