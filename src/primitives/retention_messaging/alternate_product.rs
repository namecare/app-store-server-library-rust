use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A switch-plan message and product ID you provide in a real-time response to your Get Retention Message endpoint.
///
/// [alternateProduct](https://developer.apple.com/documentation/retentionmessaging/alternateproduct)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct AlternateProduct {
    /// The message identifier of the text to display in the switch-plan retention message.
    ///
    /// [messageIdentifier](https://developer.apple.com/documentation/retentionmessaging/messageidentifier)
    #[serde(rename = "messageIdentifier")]
    pub message_identifier: Option<Uuid>,

    /// The product identifier of the subscription the retention message suggests for your customer to switch to.
    ///
    /// [productId](https://developer.apple.com/documentation/retentionmessaging/productid)
    #[serde(rename = "productId")]
    pub product_id: Option<String>,
}