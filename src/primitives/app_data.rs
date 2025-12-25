use crate::primitives::environment::Environment;
use serde::{Deserialize, Serialize};

/// App data that appears in version 2 notifications.
///
/// [appData](https://developer.apple.com/documentation/appstoreservernotifications/appdata)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AppData {
    /// The unique identifier of the app that the notification applies to.
    /// This property is available for apps that users download from the App Store.
    /// It isn't present in the sandbox environment.
    ///
    /// [appAppleId](https://developer.apple.com/documentation/appstoreservernotifications/appappleid)
    pub app_apple_id: Option<i64>,

    /// The bundle identifier of the app.
    ///
    /// [bundleId](https://developer.apple.com/documentation/appstoreservernotifications/bundleid)
    pub bundle_id: Option<String>,

    /// The server environment that the notification applies to, either sandbox or production.
    ///
    /// [environment](https://developer.apple.com/documentation/appstoreservernotifications/environment)
    pub environment: Option<Environment>,

    /// App transaction information signed by the App Store, in JSON Web Signature (JWS) format.
    ///
    /// [signedAppTransactionInfo](https://developer.apple.com/documentation/appstoreservernotifications/signedapptransactioninfo)
    pub signed_app_transaction_info: Option<String>,
}