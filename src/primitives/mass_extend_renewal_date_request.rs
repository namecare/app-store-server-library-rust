use serde::{Deserialize, Serialize};
use crate::primitives::extend_reason_code::ExtendReasonCode;

/// The request body that contains subscription-renewal-extension data to apply for all eligible active subscribers.
///
/// [MassExtendRenewalDateRequest](https://developer.apple.com/documentation/appstoreserverapi/massextendrenewaldaterequest)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct MassExtendRenewalDateRequest {
    /// The number of days to extend the subscription renewal date.
    ///
    /// [extendByDays](https://developer.apple.com/documentation/appstoreserverapi/extendbydays)
    /// maximum: 90
    #[serde(rename = "extendByDays")]
    pub extend_by_days: i32,


    /// The reason code for the subscription-renewal-date extension.
    ///
    /// [extendReasonCode](https://developer.apple.com/documentation/appstoreserverapi/extendreasoncode)
    #[serde(rename = "extendReasonCode")]
    pub extend_reason_code: ExtendReasonCode,

    /// A string that contains a unique identifier you provide to track each subscription-renewal-date extension request.
    ///
    /// [requestIdentifier](https://developer.apple.com/documentation/appstoreserverapi/requestidentifier)
    #[serde(rename = "requestIdentifier")]
    pub request_identifier: String,

    /// A list of storefront country codes you provide to limit the storefronts for a subscription-renewal-date extension.
    ///
    /// [storefrontCountryCodes](https://developer.apple.com/documentation/appstoreserverapi/storefrontcountrycodes)
    #[serde(rename = "storefrontCountryCodes")]
    pub storefront_country_codes: Vec<String>,

    /// The unique identifier for the product, that you create in App Store Connect.
    ///
    /// [productId](https://developer.apple.com/documentation/appstoreserverapi/productid)
    #[serde(rename = "productId")]
    pub product_id: String,
}
