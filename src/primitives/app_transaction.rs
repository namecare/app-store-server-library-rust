use crate::primitives::environment::Environment;
use crate::primitives::purchase_platform::PurchasePlatform;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::formats::Flexible;
use serde_with::TimestampMilliSeconds;
use uuid::Uuid;

/// Information that represents the customer’s purchase of the app, cryptographically signed by the App Store.
///
/// [AppTransaction](https://developer.apple.com/documentation/storekit/apptransaction)
#[serde_with::serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, Hash)]
pub struct AppTransaction {
    /// The server environment that signs the app transaction.
    /// [environment](https://developer.apple.com/documentation/storekit/apptransaction/3963901-environment)
    #[serde(rename = "receiptType")]
    pub receipt_type: Option<Environment>,

    /// The unique identifier the App Store uses to identify the app.
    /// [appId](https://developer.apple.com/documentation/storekit/apptransaction/3954436-appid)
    #[serde(rename = "appAppleId")]
    pub app_apple_id: Option<i64>,

    /// The bundle identifier that the app transaction applies to.
    /// [bundle_id](https://developer.apple.com/documentation/storekit/apptransaction/3954439-bundleid)
    #[serde(rename = "bundleId")]
    pub bundle_id: Option<String>,

    /// The app version that the app transaction applies to.
    /// [appVersion](https://developer.apple.com/documentation/storekit/apptransaction/3954437-appversion)
    #[serde(rename = "applicationVersion")]
    pub application_version: Option<String>,

    /// The version external identifier of the app.
    /// [appVersionID](https://developer.apple.com/documentation/storekit/apptransaction/3954438-appversionid)
    #[serde(rename = "versionExternalIdentifier")]
    pub version_external_identifier: Option<i64>,

    /// The date that the App Store signed the JWS app transaction.
    /// [signedDate](https://developer.apple.com/documentation/storekit/apptransaction/3954449-signeddate)
    #[serde(rename = "receiptCreationDate")]
    #[serde_as(as = "Option<TimestampMilliSeconds<String, Flexible>>")]
    pub receipt_creation_date: Option<DateTime<Utc>>,

    /// The date the user originally purchased the app from the App Store.
    /// [originalPurchaseDate](https://developer.apple.com/documentation/storekit/apptransaction/3954448-originalpurchasedate)
    #[serde(rename = "originalPurchaseDate")]
    #[serde_as(as = "Option<TimestampMilliSeconds<String, Flexible>>")]
    pub original_purchase_date: Option<DateTime<Utc>>,

    /// The app version that the user originally purchased from the App Store.
    /// [originalAppVersion](https://developer.apple.com/documentation/storekit/apptransaction/3954447-originalappversion)
    #[serde(rename = "originalApplicationVersion")]
    pub original_application_version: Option<String>,

    /// The Base64 device verification value to use to verify whether the app transaction belongs to the device.
    /// [deviceVerification](https://developer.apple.com/documentation/storekit/apptransaction/3954441-deviceverification)
    #[serde(rename = "deviceVerification")]
    pub device_verification: Option<String>,

    /// The UUID used to compute the device verification value.
    /// [deviceVerificationNonce](https://developer.apple.com/documentation/storekit/apptransaction/3954442-deviceverificationnonce)
    #[serde(rename = "deviceVerificationNonce")]
    pub device_verification_nonce: Option<Uuid>,

    /// The date the customer placed an order for the app before it’s available in the App Store.
    /// [preorderDate](https://developer.apple.com/documentation/storekit/apptransaction/4013175-preorderdate)
    #[serde(rename = "preorderDate")]
    #[serde_as(as = "Option<TimestampMilliSeconds<String, Flexible>>")]
    pub preorder_date: Option<DateTime<Utc>>,

    /// The unique identifier of the app download transaction.
    ///
    /// [appTransactionId](https://developer.apple.com/documentation/storekit/apptransaction/apptransactionid)
    #[serde(rename = "appTransactionId")]
    pub app_transaction_id: Option<String>,

    /// The platform on which the customer originally purchased the app.
    ///
    /// [original_platform](https://developer.apple.com/documentation/storekit/apptransaction/originalplatform)
    #[serde(rename = "originalPlatform")]
    pub original_platform: Option<PurchasePlatform>,
}

impl AppTransaction {
    /// The date that the App Store signed the JWS app transaction.
    /// [signedDate](https://developer.apple.com/documentation/storekit/apptransaction/3954449-signeddate)
    pub fn signed_date(&self) -> Option<DateTime<Utc>> {
        self.receipt_creation_date
    }
}
