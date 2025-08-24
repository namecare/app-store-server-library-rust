use crate::primitives::advanced_commerce::request_info::RequestInfo;
use crate::primitives::advanced_commerce::subscription_reactivate_item::SubscriptionReactivateItem;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::primitives::advanced_commerce::request_operation::RequestOperation;
use crate::primitives::advanced_commerce::request_version::RequestVersion;

/// The metadata your app provides to reactivate an auto-renewable subscription.
///
/// [SubscriptionReactivateInAppRequest](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionreactivateinapprequest)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionReactivateInAppRequest {
    /// The operation type for this request.
    /// Value: REACTIVATE_SUBSCRIPTION
    pub operation: RequestOperation,

    /// The version of this request.
    pub version: RequestVersion,

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

impl SubscriptionReactivateInAppRequest {
    pub fn new(
        request_reference_id: Uuid,
        transaction_id: String,
    ) -> Self {
        Self {
            operation: RequestOperation::ReactivateSubscription,
            version: RequestVersion::V1,
            items: None,
            request_info: RequestInfo::new(request_reference_id),
            storefront: None,
            transaction_id,
        }
    }

    pub fn with_items(mut self, items: Vec<SubscriptionReactivateItem>) -> Self {
        self.items = Some(items);
        self
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