use ::chrono::{DateTime, Utc};
use serde_with::formats::Flexible;
use serde_with::TimestampMilliSeconds;

use crate::models::app_data::AppData;
use crate::models::decoded_signed_data::DecodedSignedData;
use crate::models::external_purchase_token::ExternalPurchaseToken;
use crate::models::notification_data::Data;
use crate::models::notification_type_v2::NotificationTypeV2;
use crate::models::subtype::Subtype;
use crate::models::summary::Summary;

/// A decoded payload containing the version 2 notification data.
///
/// [responseBodyV2DecodedPayload](https://developer.apple.com/documentation/appstoreservernotifications/responsebodyv2decodedpayload)
#[serde_with::serde_as]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Hash)]
pub struct ResponseBodyV2DecodedPayload {
    /// The in-app purchase event for which the App Store sends this version 2 notification.
    ///
    /// [notificationType](https://developer.apple.com/documentation/appstoreservernotifications/notificationtype)
    #[serde(rename = "notificationType")]
    pub notification_type: Option<NotificationTypeV2>,

    /// Additional information that identifies the notification event.
    /// The subtype field is present only for specific version 2 notifications.
    ///
    /// [subtype](https://developer.apple.com/documentation/appstoreservernotifications/subtype)
    pub subtype: Option<Subtype>,

    /// A unique identifier for the notification.
    ///
    /// [notificationUUID](https://developer.apple.com/documentation/appstoreservernotifications/notificationuuid)
    #[serde(rename = "notificationUUID")]
    pub notification_uuid: Option<String>,

    /// The object that contains the app metadata and signed renewal and transaction information.
    /// The data, summary, and externalPurchaseToken fields are mutually exclusive. The payload contains only one of these fields.
    ///
    /// [data](https://developer.apple.com/documentation/appstoreservernotifications/data)
    pub data: Option<Data>,

    /// A string that indicates the notification’s App Store Server Notifications version number.
    ///
    /// [version](https://developer.apple.com/documentation/appstoreservernotifications/version)
    pub version: Option<String>,

    /// The UNIX time, in milliseconds, that the App Store signed the JSON Web Signature data.
    ///
    /// [signedDate](https://developer.apple.com/documentation/appstoreserverapi/signeddate)
    #[serde(rename = "signedDate")]
    #[serde_as(as = "Option<TimestampMilliSeconds<String, Flexible>>")]
    pub signed_date: Option<DateTime<Utc>>,

    /// The summary data that appears when the App Store server completes your request to extend a subscription renewal date for eligible subscribers.
    /// The data, summary, and externalPurchaseToken fields are mutually exclusive. The payload contains only one of these fields.
    ///
    /// [summary](https://developer.apple.com/documentation/appstoreservernotifications/summary)
    pub summary: Option<Summary>,

    /// This field appears when the notificationType is EXTERNAL_PURCHASE_TOKEN.
    /// The data, summary, and externalPurchaseToken fields are mutually exclusive. The payload contains only one of these fields.
    ///
    /// [externalPurchaseToken](https://developer.apple.com/documentation/appstoreservernotifications/externalpurchasetoken)
    #[serde(rename = "externalPurchaseToken")]
    pub external_purchase_token: Option<ExternalPurchaseToken>,

    /// App data that appears in version 2 notifications.
    /// The data, summary, externalPurchaseToken, and appData fields are mutually exclusive.
    ///
    /// [appData](https://developer.apple.com/documentation/appstoreservernotifications/appdata)
    #[serde(rename = "appData")]
    pub app_data: Option<AppData>,
}

impl DecodedSignedData for ResponseBodyV2DecodedPayload {
    fn signed_date_optional(&self) -> Option<DateTime<Utc>> {
        self.signed_date
    }
}
