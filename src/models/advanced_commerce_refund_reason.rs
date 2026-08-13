use serde::{Deserialize, Serialize};

/// The reason to request a refund.
/// [AdvancedCommerceRefundReason](https://developer.apple.com/documentation/advancedcommerceapi/refundreason)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AdvancedCommerceRefundReason {
    UnintendedPurchase,
    FulfillmentIssue,
    UnsatisfiedWithPurchase,
    Legal,
    Other,
    ModifyItemsRefund,
    SimulateRefundDecline,

    /// A value the App Store sent that this version of the
    /// library does not support, preserved as received.
    #[serde(untagged)]
    NotSupported(String),
}
