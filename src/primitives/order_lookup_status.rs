use serde_repr::{Serialize_repr, Deserialize_repr};

/// A value that indicates whether the order ID in the request is valid for your app.
///
/// [OrderLookupStatus](https://developer.apple.com/documentation/appstoreserverapi/orderlookupstatus)
#[derive(Debug, Clone, Deserialize_repr, Serialize_repr, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum OrderLookupStatus {
    Valid = 0,
    Invalid = 1,
}
