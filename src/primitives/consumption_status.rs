use serde_repr::{Serialize_repr, Deserialize_repr};

/// A value that indicates the extent to which the customer consumed the in-app purchase.
///
/// [consumptionStatus](https://developer.apple.com/documentation/appstoreserverapi/consumptionstatus)
#[derive(Debug, Clone, Deserialize_repr, Serialize_repr, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum ConsumptionStatus {
    Undeclared = 0,
    NotConsumed = 1,
    PartiallyConsumed = 2,
    FullyConsumed = 3,
}
