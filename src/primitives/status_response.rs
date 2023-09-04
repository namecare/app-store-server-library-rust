use crate::primitives::environment::Environment;
use crate::primitives::subscription_group_identifier_item::SubscriptionGroupIdentifierItem;
use serde::{Deserialize, Serialize};

/// The response that contains status information for all of a customer’s auto-renewable subscriptions in your app.
///
/// [StatusResponse](https://developer.apple.com/documentation/appstoreserverapi/statusresponse)
#[derive(Debug, Clone, Deserialize, Serialize, Hash)]
pub struct StatusResponse {
    /// The server environment, sandbox or production, in which the App Store generated the response.
    ///
    /// [environment](https://developer.apple.com/documentation/appstoreserverapi/environment)
    pub environment: Option<Environment>,

    /// The bundle identifier of an app.
    ///
    /// [bundleId](https://developer.apple.com/documentation/appstoreserverapi/bundleid)
    #[serde(rename = "bundleId")]
    pub bundle_id: String,

    /// The unique identifier of an app in the App Store.
    ///
    /// [appAppleId](https://developer.apple.com/documentation/appstoreservernotifications/appappleid)
    #[serde(rename = "appAppleId")]
    pub app_apple_id: i64,

    /// An array of information for auto-renewable subscriptions, including App Store-signed transaction information and App Store-signed renewal information.
    pub data: Vec<SubscriptionGroupIdentifierItem>,
}
