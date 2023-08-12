use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::primitives::data::Data;
use crate::primitives::notification_type_v2::NotificationTypeV2;
use crate::primitives::subtype::Subtype;
use crate::primitives::summary::Summary;

/// A decoded payload containing the version 2 notification data.
///
/// [responseBodyV2DecodedPayload](https://developer.apple.com/documentation/appstoreservernotifications/responsebodyv2decodedpayload)
#[derive(Debug, Deserialize, Serialize, Hash)]
pub struct ResponseBodyV2DecodedPayload {
    /// The in-app purchase event for which the App Store sends this version 2 notification.
    ///
    /// [notificationType](https://developer.apple.com/documentation/appstoreservernotifications/notificationtype)
    #[serde(rename = "notificationType")]
    pub notification_type: NotificationTypeV2,

    /// Additional information that identifies the notification event.
    /// The subtype field is present only for specific version 2 notifications.
    ///
    /// [subtype](https://developer.apple.com/documentation/appstoreservernotifications/subtype)
    pub subtype: Subtype,

    /// A unique identifier for the notification.
    ///
    /// [notificationUUID](https://developer.apple.com/documentation/appstoreservernotifications/notificationuuid)
    #[serde(rename = "notificationUUID")]
    pub notification_uuid: String,

    /// The object that contains the app metadata and signed renewal and transaction information.
    /// The data and summary fields are mutually exclusive. The payload contains one of the fields, but not both.
    ///
    /// [data](https://developer.apple.com/documentation/appstoreservernotifications/data)
    pub data: Data,

    /// A string that indicates the notification’s App Store Server Notifications version number.
    ///
    /// [version](https://developer.apple.com/documentation/appstoreservernotifications/version)
    pub version: String,

    /// The UNIX time, in milliseconds, that the App Store signed the JSON Web Signature data.
    ///
    /// [signedDate](https://developer.apple.com/documentation/appstoreserverapi/signeddate)
    #[serde(rename = "signedDate")]
    pub signed_date: DateTime<Utc>,

    /// The summary data that appears when the App Store server completes your request to extend a subscription renewal date for eligible subscribers.
    /// The data and summary fields are mutually exclusive. The payload contains one of the fields, but not both.
    ///
    /// [summary](https://developer.apple.com/documentation/appstoreservernotifications/summary)
    pub summary: Summary,
}
