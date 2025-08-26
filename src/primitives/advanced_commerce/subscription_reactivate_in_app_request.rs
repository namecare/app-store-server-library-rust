use crate::primitives::advanced_commerce::request_info::RequestInfo;
use crate::primitives::advanced_commerce::subscription_reactivate_item::SubscriptionReactivateItem;
use serde::{Deserialize, Serialize};
use crate::primitives::advanced_commerce::in_app_request::AdvancedCommerceInAppRequest;
use crate::primitives::advanced_commerce::in_app_request_operation::InAppRequestOperation;
use crate::primitives::advanced_commerce::in_app_request_version::InAppRequestVersion;

/// The metadata your app provides to reactivate an auto-renewable subscription.
///
/// [SubscriptionReactivateInAppRequest](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionreactivateinapprequest)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionReactivateInAppRequest {
    /// The operation type for this request.
    /// Value: REACTIVATE_SUBSCRIPTION
    pub operation: InAppRequestOperation,

    /// The version of this request.
    pub version: InAppRequestVersion,

    /// The details of the reactivation items.
    ///
    /// [SubscriptionReactivateItem](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionreactivateitem)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<SubscriptionReactivateItem>>,

    /// The metadata to include in server requests.
    ///
    /// [requestInfo](https://developer.apple.com/documentation/advancedcommerceapi/requestinfo)
    pub request_info: RequestInfo,

    /// The storefront for the transaction.
    ///
    /// [storefront](https://developer.apple.com/documentation/advancedcommerceapi/storefront)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storefront: Option<String>,

    /// The original transaction ID of the subscription.
    ///
    /// [transactionId](https://developer.apple.com/documentation/advancedcommerceapi/transactionid)
    pub transaction_id: String,
}

impl AdvancedCommerceInAppRequest for SubscriptionReactivateInAppRequest {}