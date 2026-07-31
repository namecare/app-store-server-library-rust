use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The response body that contains the default configuration information.
///
/// [DefaultConfigurationResponse](https://developer.apple.com/documentation/retentionmessaging/defaultconfigurationresponse)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct DefaultConfigurationResponse {
    /// The message identifier of the retention message you configured as a default.
    ///
    /// [messageIdentifier](https://developer.apple.com/documentation/retentionmessaging/messageidentifier)
    #[serde(rename = "messageIdentifier")]
    pub message_identifier: Uuid,
}
