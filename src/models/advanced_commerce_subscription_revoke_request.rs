use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::advanced_commerce_refund_reason::AdvancedCommerceRefundReason;
use crate::models::advanced_commerce_refund_type::AdvancedCommerceRefundType;
use crate::models::advanced_commerce_request_info::AdvancedCommerceRequestInfo;

/// The request data your app provides to revoke an auto-renewable subscription.
///
/// [AdvancedCommerceSubscriptionRevokeRequest](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionrevokerequest)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedCommerceSubscriptionRevokeRequest {
    /// The metadata to include in server requests.
    ///
    /// [requestInfo](https://developer.apple.com/documentation/advancedcommerceapi/requestinfo)
    pub request_info: AdvancedCommerceRequestInfo,

    /// The reason for the refund.
    ///
    /// [refundReason](https://developer.apple.com/documentation/advancedcommerceapi/refundreason)
    pub refund_reason: AdvancedCommerceRefundReason,

    /// The refund risking preference.
    ///
    /// [refundRiskingPreference](https://developer.apple.com/documentation/advancedcommerceapi/refundriskingpreference)
    pub refund_risking_preference: bool,

    /// The type of refund.
    /// Possible Values: FULL, PRORATED
    ///
    /// [refundType](https://developer.apple.com/documentation/advancedcommerceapi/refundtype)
    pub refund_type: AdvancedCommerceRefundType,

    /// The storefront for the transaction.
    ///
    /// [storefront](https://developer.apple.com/documentation/advancedcommerceapi/storefront)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storefront: Option<String>,
}

impl AdvancedCommerceSubscriptionRevokeRequest {
    pub fn new(
        request_reference_id: Uuid,
        refund_reason: AdvancedCommerceRefundReason,
        refund_risking_preference: bool,
        refund_type: AdvancedCommerceRefundType,
    ) -> Self {
        Self {
            request_info: AdvancedCommerceRequestInfo::new(request_reference_id),
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

    pub fn with_request_info(mut self, request_info: AdvancedCommerceRequestInfo) -> Self {
        self.request_info = request_info;
        self
    }
}
