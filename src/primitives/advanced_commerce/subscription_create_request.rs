use crate::primitives::advanced_commerce::descriptors::Descriptors;
use crate::primitives::advanced_commerce::period::Period;
use crate::primitives::advanced_commerce::request_info::RequestInfo;
use crate::primitives::advanced_commerce::subscription_create_item::SubscriptionCreateItem;
use serde::{Deserialize, Serialize};
use crate::primitives::advanced_commerce::in_app_request::AdvancedCommerceInAppRequest;
use crate::primitives::advanced_commerce::in_app_request_operation::InAppRequestOperation;
use crate::primitives::advanced_commerce::in_app_request_version::InAppRequestVersion;

/// The metadata your app provides when a customer purchases an auto-renewable subscription.
///
/// [SubscriptionCreateRequest](https://developer.apple.com/documentation/advancedcommerceapi/subscriptioncreaterequest)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionCreateRequest {
    /// The operation type for this request.
    /// Value: CREATE_SUBSCRIPTION
    pub operation: InAppRequestOperation,

    /// The version of this request.
    pub version: InAppRequestVersion,

    /// The currency of the price of the product.
    ///
    /// [currency](https://developer.apple.com/documentation/advancedcommerceapi/currency)
    pub currency: String,

    /// The display name and description of a subscription product.
    ///
    /// [Descriptors](https://developer.apple.com/documentation/advancedcommerceapi/descriptors)
    pub descriptors: Descriptors,

    /// The details of the subscription product for purchase.
    ///
    /// [SubscriptionCreateItem](https://developer.apple.com/documentation/advancedcommerceapi/subscriptioncreateitem)
    pub items: Vec<SubscriptionCreateItem>,

    /// The duration of a single cycle of an auto-renewable subscription.
    ///
    /// [period](https://developer.apple.com/documentation/advancedcommerceapi/period)
    pub period: Period,

    /// The identifier of a previous transaction for the subscription.
    ///
    /// [transactionId](https://developer.apple.com/documentation/advancedcommerceapi/transactionid)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_transaction_id: Option<String>,

    /// The metadata to include in server requests.
    ///
    /// [requestInfo](https://developer.apple.com/documentation/advancedcommerceapi/requestinfo)
    pub request_info: RequestInfo,

    /// The storefront for the transaction.
    ///
    /// [storefront](https://developer.apple.com/documentation/advancedcommerceapi/onetimechargecreaterequest)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storefront: Option<String>,

    /// The tax code for this product.
    ///
    /// [taxCode](https://developer.apple.com/documentation/advancedcommerceapi/onetimechargecreaterequest)
    pub tax_code: String,
}

impl AdvancedCommerceInAppRequest for SubscriptionCreateRequest {}