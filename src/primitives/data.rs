use crate::primitives::environment::Environment;
use serde::{Deserialize, Serialize};
use crate::primitives::status::Status;

/// The app metadata and the signed renewal and transaction information.
///
/// [data](https://developer.apple.com/documentation/appstoreservernotifications/data)
#[derive(Debug, Clone, Deserialize, Serialize, Hash)]
pub struct Data {
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

    /// The version of the build that identifies an iteration of the bundle.
    ///
    /// [bundleVersion](https://developer.apple.com/documentation/appstoreservernotifications/bundleversion)
    #[serde(rename = "bundleVersion")]
    pub bundle_version: Option<String>,

    /// Transaction information signed by the App Store, in JSON Web Signature (JWS) format.
    ///
    /// [JWSTransaction](https://developer.apple.com/documentation/appstoreserverapi/jwstransaction)
    #[serde(rename = "signedTransactionInfo")]
    pub signed_transaction_info: Option<String>,

    /// Subscription renewal information, signed by the App Store, in JSON Web Signature (JWS) format.
    ///
    /// [JWSRenewalInfo](https://developer.apple.com/documentation/appstoreserverapi/jwsrenewalinfo)
    #[serde(rename = "signedRenewalInfo")]
    pub signed_renewal_info: Option<String>,

    /// The status of an auto-renewable subscription at the time the App Store signs the notification.
    ///
    /// [JWSRenewalInfo](https://developer.apple.com/documentation/appstoreservernotifications/status)
    #[serde(rename = "status")]
    pub status: Option<Status>
}
