use serde::{Deserialize, Serialize};
use crate::primitives::environment::Environment;

/// The app metadata and the signed renewal and transaction information.
///
/// [data](https://developer.apple.com/documentation/appstoreservernotifications/data)
#[derive(Debug, Deserialize, Serialize, Hash)]
pub struct Data {
    /// The server environment that the notification applies to, either sandbox or production.
    ///
    /// [environment](https://developer.apple.com/documentation/appstoreservernotifications/environment)
    pub environment: Option<Environment>,

    /// The unique identifier of an app in the App Store.
    ///
    /// [app_apple_id](https://developer.apple.com/documentation/appstoreservernotifications/appappleid)
    pub app_apple_id: Option<i64>,

    /// The bundle identifier of an app.
    ///
    /// [bundle_id](https://developer.apple.com/documentation/appstoreserverapi/bundleid)
    pub bundle_id: Option<String>,

    /// The version of the build that identifies an iteration of the bundle.
    ///
    /// [bundleVersion](https://developer.apple.com/documentation/appstoreservernotifications/bundleversion)
    pub bundle_version: Option<String>,

    /// Transaction information signed by the App Store, in JSON Web Signature (JWS) format.
    ///
    /// [JWSTransaction](https://developer.apple.com/documentation/appstoreserverapi/jwstransaction)
    pub signed_transaction_info: Option<String>,

    /// Subscription renewal information, signed by the App Store, in JSON Web Signature (JWS) format.
    ///
    /// [JWSRenewalInfo](https://developer.apple.com/documentation/appstoreserverapi/jwsrenewalinfo)
    pub signed_renewal_info: Option<String>,
}