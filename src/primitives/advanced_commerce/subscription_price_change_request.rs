use crate::primitives::advanced_commerce::request_info::RequestInfo;
use crate::primitives::advanced_commerce::subscription_price_change_item::SubscriptionPriceChangeItem;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The metadata your app provides to change the price of an auto-renewable subscription.
///
/// [SubscriptionPriceChangeRequest](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionpricechangerequest)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionPriceChangeRequest {
    /// The currency of the price of the product.
    ///
    /// [currency](https://developer.apple.com/documentation/advancedcommerceapi/currency)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// The details of the price change items.
    ///
    /// [SubscriptionPriceChangeItem](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionpricechangeitem)
    pub items: Vec<SubscriptionPriceChangeItem>,

    /// The metadata to include in server requests.
    ///
    /// [requestInfo](https://developer.apple.com/documentation/advancedcommerceapi/requestinfo)
    pub request_info: RequestInfo,

    /// The storefront for the transaction.
    ///
    /// [storefront](https://developer.apple.com/documentation/advancedcommerceapi/onetimechargecreaterequest)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storefront: Option<String>,
}

impl SubscriptionPriceChangeRequest {
    pub fn new(
        currency: String,
        items: Vec<SubscriptionPriceChangeItem>,
        request_reference_id: Uuid,
    ) -> Self {
        Self {
            currency: Some(currency),
            items: items,
            request_info: RequestInfo::new(request_reference_id),
            storefront: None,
        }
    }

    pub fn with_storefront(mut self, storefront: String) -> Self {
        self.storefront = Some(storefront);
        self
    }

    pub fn with_request_info(mut self, request_info: RequestInfo) -> Self {
        self.request_info = request_info;
        self
    }
}