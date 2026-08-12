use serde::{Deserialize, Serialize};

use crate::models::advanced_commerce_in_app_request::AdvancedCommerceInAppRequest;
use crate::models::advanced_commerce_in_app_request_operation::AdvancedCommerceInAppRequestOperation;
use crate::models::advanced_commerce_in_app_request_version::AdvancedCommerceInAppRequestVersion;
use crate::models::advanced_commerce_request_info::AdvancedCommerceRequestInfo;
use crate::models::advanced_commerce_subscription_reactivate_item::AdvancedCommerceSubscriptionReactivateItem;

/// The metadata your app provides to reactivate an auto-renewable subscription.
///
/// [AdvancedCommerceSubscriptionReactivateInAppRequest](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionreactivateinapprequest)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedCommerceSubscriptionReactivateInAppRequest {
    /// The operation type for this request.
    /// Value: REACTIVATE_SUBSCRIPTION
    #[serde(default = "default_operation")]
    pub operation: AdvancedCommerceInAppRequestOperation,

    /// The version of this request.
    #[serde(default = "default_version")]
    pub version: AdvancedCommerceInAppRequestVersion,

    /// The details of the reactivation items.
    ///
    /// [AdvancedCommerceSubscriptionReactivateItem](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionreactivateitem)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<AdvancedCommerceSubscriptionReactivateItem>>,

    /// The metadata to include in server requests.
    ///
    /// [requestInfo](https://developer.apple.com/documentation/advancedcommerceapi/requestinfo)
    pub request_info: AdvancedCommerceRequestInfo,

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

impl AdvancedCommerceInAppRequest for AdvancedCommerceSubscriptionReactivateInAppRequest {}

/// Apple fixes this request's operation; it is emitted on encode and defaulted on decode,
/// matching the Swift library where `operation` is a computed constant that is never decoded.
fn default_operation() -> AdvancedCommerceInAppRequestOperation {
    AdvancedCommerceInAppRequestOperation::ReactivateSubscription
}

/// Apple fixes this request's version; see [`default_operation`].
fn default_version() -> AdvancedCommerceInAppRequestVersion {
    AdvancedCommerceInAppRequestVersion::V1
}
