use serde::{Deserialize, Serialize};

use crate::models::extend_reason_code::ExtendReasonCode;

/// The request body that contains subscription-renewal-extension data for an individual subscription.
///
/// [ExtendRenewalDateRequest](https://developer.apple.com/documentation/appstoreserverapi/extendrenewaldaterequest)
#[derive(Debug, Clone, Deserialize, Serialize, Hash)]
pub struct ExtendRenewalDateRequest {
    /// (Required) The number of days to extend the subscription renewal date.
    ///
    /// [extendByDays](https://developer.apple.com/documentation/appstoreserverapi/extendbydays)
    ///
    /// maximum: 90
    #[serde(rename = "extendByDays")]
    pub extend_by_days: i32,

    /// (Required) The reason code for the subscription date extension.
    ///
    /// [extendReasonCode](https://developer.apple.com/documentation/appstoreserverapi/extendreasoncode)
    #[serde(rename = "extendReasonCode")]
    pub extend_reason_code: ExtendReasonCode,

    /// (Required) A string that contains a unique identifier you provide to track each subscription-renewal-date extension request.
    ///
    /// [requestIdentifier](https://developer.apple.com/documentation/appstoreserverapi/requestidentifier)
    #[serde(rename = "requestIdentifier")]
    pub request_identifier: String,
}
