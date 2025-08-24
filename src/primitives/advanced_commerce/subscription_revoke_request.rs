use crate::primitives::advanced_commerce::request_info::RequestInfo;
use crate::primitives::advanced_commerce::refund_reason::RefundReason;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::primitives::advanced_commerce::RefundType;

/// The request data your app provides to revoke an auto-renewable subscription.
///
/// [SubscriptionRevokeRequest](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionrevokerequest)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionRevokeRequest {
    /// The metadata to include in server requests.
    ///
    /// [requestInfo](https://developer.apple.com/documentation/advancedcommerceapi/requestinfo)
    pub request_info: RequestInfo,

    /// The reason for the refund.
    ///
    /// [refundReason](https://developer.apple.com/documentation/advancedcommerceapi/refundreason)
    pub refund_reason: RefundReason,

    /// The refund risking preference.
    ///
    /// [refundRiskingPreference](https://developer.apple.com/documentation/advancedcommerceapi/refundriskingpreference)
    pub refund_risking_preference: String,

    /// The type of refund.
    /// Possible Values: FULL, PRORATED
    ///
    /// [refundType](https://developer.apple.com/documentation/advancedcommerceapi/refundtype)
    pub refund_type: RefundType,

    /// The storefront for the transaction.
    ///
    /// [storefront](https://developer.apple.com/documentation/advancedcommerceapi/storefront)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storefront: Option<String>,
}

impl SubscriptionRevokeRequest {
    pub fn new(
        request_reference_id: Uuid,
        refund_reason: RefundReason,
        refund_risking_preference: String,
        refund_type: RefundType,
    ) -> Self {
        Self {
            request_info: RequestInfo::new(request_reference_id),
            refund_reason,
            refund_risking_preference,
            refund_type,
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