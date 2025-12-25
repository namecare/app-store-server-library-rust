use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A response object you provide to present an offer or switch-plan recommendation message.
///
/// [advancedCommerceInfo](https://developer.apple.com/documentation/retentionmessaging/advancedcommerceinfo)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedCommerceInfo {

    /// The identifier of the message to display to the customer, along with the offer or switch-plan recommendation provided in advancedCommerceData.
    ///
    /// [messageIdentifier](https://developer.apple.com/documentation/retentionmessaging/messageidentifier)
    pub message_identifier: Uuid,

    /// A Base64-encoded JSON object which contains a JWS describing an offer or switch-plan recommendation.
    ///
    /// [advancedCommerceData](https://developer.apple.com/documentation/retentionmessaging/advancedcommercedata)
    pub advanced_commerce_data: String,
}