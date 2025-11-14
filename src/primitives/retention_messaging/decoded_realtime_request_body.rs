use crate::primitives::environment::Environment;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::formats::Flexible;
use serde_with::TimestampMilliSeconds;
use uuid::Uuid;

/// The decoded request body the App Store sends to your server to request a real-time retention message.
///
/// [DecodedRealtimeRequestBody](https://developer.apple.com/documentation/retentionmessaging/decodedrealtimerequestbody)
#[serde_with::serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct DecodedRealtimeRequestBody {
    /// The original transaction identifier of the customer's subscription.
    ///
    /// [originalTransactionId](https://developer.apple.com/documentation/retentionmessaging/originaltransactionid)
    #[serde(rename = "originalTransactionId")]
    pub original_transaction_id: String,

    /// The unique identifier of the app in the App Store.
    ///
    /// [appAppleId](https://developer.apple.com/documentation/retentionmessaging/appappleid)
    #[serde(rename = "appAppleId")]
    pub app_apple_id: i64,

    /// The unique identifier of the auto-renewable subscription.
    ///
    /// [productId](https://developer.apple.com/documentation/retentionmessaging/productid)
    #[serde(rename = "productId")]
    pub product_id: String,

    /// The device's locale.
    ///
    /// [locale](https://developer.apple.com/documentation/retentionmessaging/locale)
    #[serde(rename = "userLocale")]
    pub user_locale: String,

    /// A UUID the App Store server creates to uniquely identify each request.
    ///
    /// [requestIdentifier](https://developer.apple.com/documentation/retentionmessaging/requestidentifier)
    #[serde(rename = "requestIdentifier")]
    pub request_identifier: Uuid,

    /// The UNIX time, in milliseconds, that the App Store signed the JSON Web Signature (JWS) data.
    ///
    /// [signedDate](https://developer.apple.com/documentation/retentionmessaging/signeddate)
    #[serde(rename = "signedDate")]
    #[serde_as(as = "TimestampMilliSeconds<String, Flexible>")]
    pub signed_date: DateTime<Utc>,

    /// The server environment, either sandbox or production.
    ///
    /// [environment](https://developer.apple.com/documentation/retentionmessaging/environment)
    pub environment: Environment,
}