use crate::models::advanced_commerce_request_info::AdvancedCommerceRequestInfo;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The request data your app provides to cancel an auto-renewable subscription.
///
/// [AdvancedCommerceSubscriptionCancelRequest](https://developer.apple.com/documentation/advancedcommerceapi/subscriptioncancelrequest)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedCommerceSubscriptionCancelRequest {
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

impl AdvancedCommerceSubscriptionCancelRequest {
    pub fn new(request_reference_id: Uuid) -> Self {
        Self {
            request_info: AdvancedCommerceRequestInfo::new(request_reference_id),
            storefront: None,
        }
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
