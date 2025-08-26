use crate::primitives::advanced_commerce::request_info::RequestInfo;
use crate::primitives::advanced_commerce::request_refund_item::RequestRefundItem;
use serde::{Deserialize, Serialize};

/// The request data your app provides to request refunds for items.
///
/// [RequestRefundRequest](https://developer.apple.com/documentation/advancedcommerceapi/requestrefundrequest)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RequestRefundRequest {
    /// The metadata to include in server requests.
    ///
    /// [requestInfo](https://developer.apple.com/documentation/advancedcommerceapi/requestinfo)
    pub request_info: RequestInfo,
    
    /// The currency of the refund amount.
    ///
    /// [currency](https://developer.apple.com/documentation/advancedcommerceapi/currency)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    
    /// The list of items to request refunds for.
    ///
    /// [RequestRefundItem](https://developer.apple.com/documentation/advancedcommerceapi/requestrefunditem)
    pub items: Vec<RequestRefundItem>,
    
    /// A Boolean value that indicates the refund risking preference.
    ///
    /// [RefundRiskingPreference](https://developer.apple.com/documentation/advancedcommerceapi/refundriskingpreference)
    pub refund_risking_preference: bool,
    
    /// The storefront for the transaction.
    ///
    /// [storefront](https://developer.apple.com/documentation/advancedcommerceapi/storefront)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storefront: Option<String>,
}