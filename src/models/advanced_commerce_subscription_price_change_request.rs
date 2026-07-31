use crate::models::advanced_commerce_request_info::AdvancedCommerceRequestInfo;
use crate::models::advanced_commerce_subscription_price_change_item::AdvancedCommerceSubscriptionPriceChangeItem;
use crate::models::helper_validation_utils::{validate_items, ValidationError};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The metadata your app provides to change the price of an auto-renewable subscription.
///
/// [AdvancedCommerceSubscriptionPriceChangeRequest](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionpricechangerequest)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedCommerceSubscriptionPriceChangeRequest {
    /// The currency of the price of the product.
    ///
    /// [currency](https://developer.apple.com/documentation/advancedcommerceapi/currency)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// The details of the price change items.
    ///
    /// [AdvancedCommerceSubscriptionPriceChangeItem](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionpricechangeitem)
    pub items: Vec<AdvancedCommerceSubscriptionPriceChangeItem>,

    /// The metadata to include in server requests.
    ///
    /// [requestInfo](https://developer.apple.com/documentation/advancedcommerceapi/requestinfo)
    pub request_info: AdvancedCommerceRequestInfo,

    /// The storefront for the transaction.
    ///
    /// [storefront](https://developer.apple.com/documentation/advancedcommerceapi/onetimechargecreaterequest)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storefront: Option<String>,
}

impl AdvancedCommerceSubscriptionPriceChangeRequest {
    pub fn new(
        currency: String,
        items: Vec<AdvancedCommerceSubscriptionPriceChangeItem>,
        request_reference_id: Uuid,
    ) -> Result<Self, ValidationError> {
        Ok(Self {
            currency: Some(currency),
            items: validate_items(items)?,
            request_info: AdvancedCommerceRequestInfo::new(request_reference_id),
            storefront: None,
        })
    }

    pub fn with_storefront(mut self, storefront: String) -> Self {
        self.storefront = Some(storefront);
        self
    }

    pub fn with_request_info(mut self, request_info: AdvancedCommerceRequestInfo) -> Self {
        self.request_info = request_info;
        self
    }
}
