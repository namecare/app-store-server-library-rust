use serde::{Deserialize, Serialize};

/// A value that indicates whether the order ID in the request is valid for your app.
///
/// [OrderLookupStatus](https://developer.apple.com/documentation/appstoreserverapi/orderlookupstatus)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(from = "i64", into = "i64")]
pub enum OrderLookupStatus {
    Valid,
    Invalid,

    /// A value the App Store sent that this version of the
    /// library does not support, preserved as received.
    NotSupported(i64),
}

impl From<i64> for OrderLookupStatus {
    fn from(value: i64) -> Self {
        match value {
            0 => OrderLookupStatus::Valid,
            1 => OrderLookupStatus::Invalid,
            other => OrderLookupStatus::NotSupported(other),
        }
    }
}

impl From<OrderLookupStatus> for i64 {
    fn from(value: OrderLookupStatus) -> Self {
        match value {
            OrderLookupStatus::Valid => 0,
            OrderLookupStatus::Invalid => 1,
            OrderLookupStatus::NotSupported(other) => other,
        }
    }
}
