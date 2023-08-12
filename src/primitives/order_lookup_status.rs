use serde::{Deserialize, Serialize};

/// A value that indicates whether the order ID in the request is valid for your app.
///
/// [OrderLookupStatus](https://developer.apple.com/documentation/appstoreserverapi/orderlookupstatus)
#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum OrderLookupStatus {
    Valid = 0,
    Invalid = 1,
}
