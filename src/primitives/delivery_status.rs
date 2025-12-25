use serde::{Deserialize, Serialize};

/// A value that indicates whether the app successfully delivered an In-App Purchase that works properly.
///
/// [deliveryStatus](https://developer.apple.com/documentation/appstoreserverapi/deliverystatus)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum DeliveryStatus {
    /// The app delivered the In-App Purchase and it's working properly.
    #[serde(rename = "DELIVERED")]
    Delivered,
    /// The app didn't deliver the In-App Purchase due to a quality issue.
    #[serde(rename = "UNDELIVERED_QUALITY_ISSUE")]
    UndeliveredQualityIssue,
    /// The app delivered the wrong item.
    #[serde(rename = "UNDELIVERED_WRONG_ITEM")]
    UndeliveredWrongItem,
    /// The app didn't deliver the In-App Purchase due to a server outage.
    #[serde(rename = "UNDELIVERED_SERVER_OUTAGE")]
    UndeliveredServerOutage,
    /// The app didn't deliver the In-App Purchase for other reasons.
    #[serde(rename = "UNDELIVERED_OTHER")]
    UndeliveredOther,
}