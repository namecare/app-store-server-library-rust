use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::primitives::notification_type_v2::NotificationTypeV2;
use crate::primitives::subtype::Subtype;

/// The request body for notification history.
///
/// [NotificationHistoryRequest](https://developer.apple.com/documentation/appstoreserverapi/notificationhistoryrequest)
#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct NotificationHistoryRequest {
    /// The start date of the timespan for the requested App Store Server Notification history records.
    /// The startDate needs to precede the endDate. Choose a startDate that’s within the past 180 days from the current date.
    ///
    /// [startDate](https://developer.apple.com/documentation/appstoreserverapi/startdate)
    #[serde(rename = "startDate")]
    pub start_date: Option<NaiveDateTime>,

    /// The end date of the timespan for the requested App Store Server Notification history records.
    /// Choose an endDate that’s later than the startDate. If you choose an endDate in the future, the endpoint automatically uses the current date as the endDate.
    ///
    /// [endDate](https://developer.apple.com/documentation/appstoreserverapi/enddate)
    #[serde(rename = "endDate")]
    pub end_date: Option<NaiveDateTime>,

    /// A notification type. Provide this field to limit the notification history records to those with this one notification type.
    /// For a list of notifications types, see notificationType.
    /// Include either the transactionId or the notificationType in your query, but not both.
    ///
    /// [notificationType](https://developer.apple.com/documentation/appstoreserverapi/notificationtype)
    #[serde(rename = "notificationType")]
    pub notification_type: Option<NotificationTypeV2>,

    /// A notification subtype. Provide this field to limit the notification history records to those with this one notification subtype.
    /// For a list of subtypes, see subtype. If you specify a notificationSubtype, you need to also specify its related notificationType.
    ///
    /// [notificationSubtype](https://developer.apple.com/documentation/appstoreserverapi/notificationsubtype)
    #[serde(rename = "notificationSubtype")]
    pub notification_subtype: Option<Subtype>,

    /// The transaction identifier, which may be an original transaction identifier, of any transaction belonging to the customer.
    /// Provide this field to limit the notification history request to this one customer.
    /// Include either the transactionId or the notificationType in your query, but not both.
    ///
    /// [transactionId](https://developer.apple.com/documentation/appstoreserverapi/transactionid)
    #[serde(rename = "transactionId")]
    pub transaction_id: Option<String>,

    /// A Boolean value you set to true to request only the notifications that haven’t reached your server successfully.
    /// The response also includes notifications that the App Store server is currently retrying to send to your server.
    ///
    /// [onlyFailures](https://developer.apple.com/documentation/appstoreserverapi/onlyfailures)
    #[serde(rename = "onlyFailures")]
    pub only_failures: Option<bool>,
}
