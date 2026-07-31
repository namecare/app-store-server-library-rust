use crate::models::advanced_commerce_request_info::AdvancedCommerceRequestInfo;
use crate::models::advanced_commerce_request_refund_item::AdvancedCommerceRequestRefundItem;
use crate::models::helper_validation_utils::{validate_items, ValidationError};
use serde::{Deserialize, Serialize};

/// The request data your app provides to request refunds for items.
///
/// [AdvancedCommerceRequestRefundRequest](https://developer.apple.com/documentation/advancedcommerceapi/requestrefundrequest)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedCommerceRequestRefundRequest {
    /// The metadata to include in server requests.
    ///
    /// [requestInfo](https://developer.apple.com/documentation/advancedcommerceapi/requestinfo)
    pub request_info: AdvancedCommerceRequestInfo,

    /// The currency of the refund amount.
    ///
    /// [currency](https://developer.apple.com/documentation/advancedcommerceapi/currency)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// The list of items to request refunds for.
    ///
    /// [AdvancedCommerceRequestRefundItem](https://developer.apple.com/documentation/advancedcommerceapi/requestrefunditem)
    pub items: Vec<AdvancedCommerceRequestRefundItem>,

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

impl AdvancedCommerceRequestRefundRequest {
    pub fn new(
        items: Vec<AdvancedCommerceRequestRefundItem>,
        refund_risking_preference: bool,
        request_info: AdvancedCommerceRequestInfo,
    ) -> Result<Self, ValidationError> {
        Ok(Self {
            request_info,
            currency: None,
            items: validate_items(items)?,
            refund_risking_preference,
            storefront: None,
        })
    }

    pub fn with_currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
        self
    }

    pub fn with_storefront(mut self, storefront: String) -> Self {
        self.storefront = Some(storefront);
        self
    }
}
