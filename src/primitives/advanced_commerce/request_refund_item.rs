use crate::primitives::advanced_commerce::refund_reason::RefundReason;
use serde::{Deserialize, Serialize};
use crate::primitives::advanced_commerce::refund_type::RefundType;

/// The data your app provides to request a refund for an item.
///
/// [RequestRefundItem](https://developer.apple.com/documentation/advancedcommerceapi/requestrefunditem)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RequestRefundItem {
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
    /// [RefundReason](https://developer.apple.com/documentation/advancedcommerceapi/refundreason)
    pub refund_reason: RefundReason,
    
    /// The type of refund. Possible values: FULL, PRORATED, CUSTOM.
    ///
    /// [RefundType](https://developer.apple.com/documentation/advancedcommerceapi/refundtype)
    pub refund_type: RefundType,
    
    /// A Boolean value that indicates whether to revoke the item.
    ///
    /// [Revoke](https://developer.apple.com/documentation/advancedcommerceapi/revoke)
    pub revoke: bool,
}