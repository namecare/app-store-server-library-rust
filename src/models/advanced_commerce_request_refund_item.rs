use crate::models::advanced_commerce_refund_reason::AdvancedCommerceRefundReason;
use crate::models::advanced_commerce_refund_type::AdvancedCommerceRefundType;
use serde::{Deserialize, Serialize};

/// The data your app provides to request a refund for an item.
///
/// [AdvancedCommerceRequestRefundItem](https://developer.apple.com/documentation/advancedcommerceapi/requestrefunditem)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedCommerceRequestRefundItem {
    /// The SKU identifier for the item to refund.
    ///
    /// [SKU](https://developer.apple.com/documentation/advancedcommerceapi/sku)
    #[serde(rename = "SKU")]
    pub sku: String,

    /// A refund amount, in milliunits of the currency.
    ///
    /// [RefundAmount](https://developer.apple.com/documentation/advancedcommerceapi/refundamount)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_amount: Option<i32>,

    /// The reason for the refund.
    ///
    /// [AdvancedCommerceRefundReason](https://developer.apple.com/documentation/advancedcommerceapi/refundreason)
    pub refund_reason: AdvancedCommerceRefundReason,

    /// The type of refund. Possible values: FULL, PRORATED, CUSTOM.
    ///
    /// [AdvancedCommerceRefundType](https://developer.apple.com/documentation/advancedcommerceapi/refundtype)
    pub refund_type: AdvancedCommerceRefundType,

    /// A Boolean value that indicates whether to revoke the item.
    ///
    /// [Revoke](https://developer.apple.com/documentation/advancedcommerceapi/revoke)
    pub revoke: bool,
}
