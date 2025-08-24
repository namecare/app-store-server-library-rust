use crate::primitives::advanced_commerce::request_info::RequestInfo;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The request data your app provides to cancel an auto-renewable subscription.
///
/// [SubscriptionCancelRequest](https://developer.apple.com/documentation/advancedcommerceapi/subscriptioncancelrequest)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionCancelRequest {
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

impl SubscriptionCancelRequest {
    pub fn new(request_reference_id: Uuid) -> Self {
        Self {
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