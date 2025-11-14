use crate::primitives::retention_messaging::promotional_offer_signature_v1::PromotionalOfferSignatureV1;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A promotional offer and message you provide in a real-time response to your Get Retention Message endpoint.
///
/// [promotionalOffer](https://developer.apple.com/documentation/retentionmessaging/promotionaloffer)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct PromotionalOffer {
    /// The identifier of the message to display to the customer, along with the promotional offer.
    ///
    /// [messageIdentifier](https://developer.apple.com/documentation/retentionmessaging/messageidentifier)
    #[serde(rename = "messageIdentifier", skip_serializing_if = "Option::is_none")]
    pub message_identifier: Option<Uuid>,

    /// The promotional offer signature in V2 format.
    ///
    /// [promotionalOfferSignatureV2](https://developer.apple.com/documentation/retentionmessaging/promotionaloffersignaturev2)
    #[serde(rename = "promotionalOfferSignatureV2", skip_serializing_if = "Option::is_none")]
    pub promotional_offer_signature_v2: Option<String>,

    /// The promotional offer signature in V1 format.
    ///
    /// [promotionalOfferSignatureV1](https://developer.apple.com/documentation/retentionmessaging/promotionaloffersignaturev1)
    #[serde(rename = "promotionalOfferSignatureV1", skip_serializing_if = "Option::is_none")]
    pub promotional_offer_signature_v1: Option<PromotionalOfferSignatureV1>,
}