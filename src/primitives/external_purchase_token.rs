use crate::primitives::token_type::TokenType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::formats::Flexible;
use serde_with::TimestampMilliSeconds;

/// The payload data that contains an external purchase token.
///
/// [externalPurchaseToken](https://developer.apple.com/documentation/appstoreservernotifications/externalpurchasetoken)
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExternalPurchaseToken {
    /// The field of an external purchase token that uniquely identifies the token.
    ///
    /// [externalPurchaseId](https://developer.apple.com/documentation/appstoreservernotifications/externalpurchaseid)
    #[serde(rename = "externalPurchaseId")]
    pub external_purchase_id: Option<String>,

    /// The field of an external purchase token that contains the UNIX date, in milliseconds,
    /// when the system created the token.
    ///
    /// [tokenCreationDate](https://developer.apple.com/documentation/appstoreservernotifications/tokencreationdate)
    #[serde(rename = "tokenCreationDate")]
    #[serde_as(as = "Option<TimestampMilliSeconds<String, Flexible>>")]
    pub token_creation_date: Option<DateTime<Utc>>,

    /// The unique identifier of an app in the App Store.
    ///
    /// [appAppleId](https://developer.apple.com/documentation/appstoreservernotifications/appappleid)
    #[serde(rename = "appAppleId")]
    pub app_apple_id: Option<i64>,

    /// The bundle identifier of an app.
    ///
    /// [bundleId](https://developer.apple.com/documentation/appstoreservernotifications/bundleid)
    #[serde(rename = "bundleId")]
    pub bundle_id: Option<String>,

    /// The UNIX time, in milliseconds, when a token expires. This field is present only for custom link tokens.
    ///
    /// [tokenExpirationDate](https://developer.apple.com/documentation/appstoreservernotifications/tokenexpirationdate)
    #[serde(rename = "tokenExpirationDate")]
    #[serde_as(as = "Option<TimestampMilliSeconds<String, Flexible>>")]
    pub token_expiration_date: Option<DateTime<Utc>>,

    /// The type of an external purchase custom link token.
    ///
    /// [tokenType](https://developer.apple.com/documentation/appstoreservernotifications/tokentype)
    #[serde(rename = "tokenType")]
    pub token_type: Option<TokenType>,
}
