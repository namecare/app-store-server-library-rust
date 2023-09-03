use crate::primitives::environment::Environment;
use serde::{Deserialize, Serialize};

/// The payload data for a subscription-renewal-date extension notification.
///
/// [Summary](https://developer.apple.com/documentation/appstoreservernotifications/summary)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct Summary {
    /// The server environment that the notification applies to, either sandbox or production.
    ///
    /// [environment](https://developer.apple.com/documentation/appstoreservernotifications/environment)
    pub environment: Option<Environment>,

    /// The unique identifier of an app in the App Store.
    ///
    /// [appAppleId](https://developer.apple.com/documentation/appstoreservernotifications/appappleid)
    #[serde(rename = "appAppleId")]
    pub app_apple_id: Option<i64>,

    /// The bundle identifier of an app.
    ///
    /// [bundleId](https://developer.apple.com/documentation/appstoreserverapi/bundleid)
    #[serde(rename = "bundleId")]
    pub bundle_id: Option<String>,

    /// The unique identifier for the product, that you create in App Store Connect.
    ///
    /// [productId](https://developer.apple.com/documentation/appstoreserverapi/productid)
    #[serde(rename = "productId")]
    pub product_id: Option<String>,

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

    /// The count of subscriptions that successfully receive a subscription-renewal-date extension.
    ///
    /// [succeededCount](https://developer.apple.com/documentation/appstoreserverapi/succeededcount)
    #[serde(rename = "succeededCount")]
    pub succeeded_count: i64,

    /// The count of subscriptions that fail to receive a subscription-renewal-date extension.
    ///
    /// [failedCount](https://developer.apple.com/documentation/appstoreserverapi/failedcount)
    #[serde(rename = "failedCount")]
    pub failed_count: i64,
}
