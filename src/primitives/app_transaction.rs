use crate::primitives::environment::Environment;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, Hash)]
pub struct AppTransaction {
    /// The server environment that signs the app transaction.
    /// [environment](https://developer.apple.com/documentation/storekit/apptransaction/3963901-environment)
    pub receipt_type: Option<Environment>,

    /// The unique identifier the App Store uses to identify the app.
    /// [appId](https://developer.apple.com/documentation/storekit/apptransaction/3954436-appid)
    pub app_apple_id: Option<i64>,

    /// The bundle identifier that the app transaction applies to.
    /// [bundle_id](https://developer.apple.com/documentation/storekit/apptransaction/3954439-bundleid)
    pub bundle_id: Option<String>,

    /// The app version that the app transaction applies to.
    /// [appVersion](https://developer.apple.com/documentation/storekit/apptransaction/3954437-appversion)
    pub application_version: Option<String>,

    /// The version external identifier of the app.
    /// [appVersionID](https://developer.apple.com/documentation/storekit/apptransaction/3954438-appversionid)
    pub version_external_identifier: Option<i64>,

    /// The date that the App Store signed the JWS app transaction.
    /// [signedDate](https://developer.apple.com/documentation/storekit/apptransaction/3954449-signeddate)
    pub receipt_creation_date: Option<DateTime<Utc>>,

    /// The date the user originally purchased the app from the App Store.
    /// [originalPurchaseDate](https://developer.apple.com/documentation/storekit/apptransaction/3954448-originalpurchasedate)
    pub original_purchase_date: Option<DateTime<Utc>>,

    /// The app version that the user originally purchased from the App Store.
    /// [originalAppVersion](https://developer.apple.com/documentation/storekit/apptransaction/3954447-originalappversion)
    pub original_application_version: Option<String>,

    /// The Base64 device verification value to use to verify whether the app transaction belongs to the device.
    /// [deviceVerification](https://developer.apple.com/documentation/storekit/apptransaction/3954441-deviceverification)
    pub device_verification: Option<String>,

    /// The UUID used to compute the device verification value.
    /// [deviceVerificationNonce](https://developer.apple.com/documentation/storekit/apptransaction/3954442-deviceverificationnonce)
    pub device_verification_nonce: Option<Uuid>,

    /// The date the customer placed an order for the app before it’s available in the App Store.
    /// [preorderDate](https://developer.apple.com/documentation/storekit/apptransaction/4013175-preorderdate)
    pub preorder_date: Option<DateTime<Utc>>,

    /// The date that the App Store signed the JWS app transaction.
    /// [signedDate](https://developer.apple.com/documentation/storekit/apptransaction/3954449-signeddate)
    pub signed_date: Option<DateTime<Utc>>,
}
