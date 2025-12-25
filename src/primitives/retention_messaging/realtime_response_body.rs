use crate::primitives::retention_messaging::alternate_product::AlternateProduct;
use crate::primitives::retention_messaging::message::Message;
use crate::primitives::retention_messaging::promotional_offer::PromotionalOffer;
use serde::{Deserialize, Serialize};
use crate::primitives::retention_messaging::advanced_commerce_info::AdvancedCommerceInfo;

/// A response you provide to choose, in real time, a retention message the system displays to the customer.
///
/// [RealtimeResponseBody](https://developer.apple.com/documentation/retentionmessaging/realtimeresponsebody)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct RealtimeResponseBody {
    /// A retention message that's text-based and can include an optional image.
    ///
    /// [message](https://developer.apple.com/documentation/retentionmessaging/message)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,

    /// A retention message with a switch-plan option.
    ///
    /// [alternateProduct](https://developer.apple.com/documentation/retentionmessaging/alternateproduct)
    #[serde(rename = "alternateProduct", skip_serializing_if = "Option::is_none")]
    pub alternate_product: Option<AlternateProduct>,

    /// A retention message that includes a promotional offer.
    ///
    /// [promotionalOffer](https://developer.apple.com/documentation/retentionmessaging/promotionaloffer)
    #[serde(rename = "promotionalOffer", skip_serializing_if = "Option::is_none")]
    pub promotional_offer: Option<PromotionalOffer>,

    /// A retention offer or switch plan option.
    /// If you pass this object for a subscription that’s not an Advanced Commerce subscription, the framework treats the request as invalid and ignores the response.
    /// If you supply this field, don’t include the other fields.
    ///
    /// [advancedCommerceInfo](https://developer.apple.com/documentation/retentionmessaging/promotionaloffer)
    #[serde(rename = "advancedCommerceInfo", skip_serializing_if = "Option::is_none")]
    pub advanced_commerce_info: Option<AdvancedCommerceInfo>,
}