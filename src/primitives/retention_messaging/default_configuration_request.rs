use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The request body that contains the default configuration information.
///
/// [DefaultConfigurationRequest](https://developer.apple.com/documentation/retentionmessaging/defaultconfigurationrequest)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct DefaultConfigurationRequest {
    /// The message identifier of the message to configure as a default message.
    ///
    /// [messageIdentifier](https://developer.apple.com/documentation/retentionmessaging/messageidentifier)
    #[serde(rename = "messageIdentifier")]
    pub message_identifier: Option<Uuid>,
}