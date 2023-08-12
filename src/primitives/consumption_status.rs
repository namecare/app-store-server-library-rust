use serde::{Deserialize, Serialize};

/// A value that indicates the extent to which the customer consumed the in-app purchase.
///
/// [consumptionStatus](https://developer.apple.com/documentation/appstoreserverapi/consumptionstatus)
#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum ConsumptionStatus {
    Undeclared = 0,
    NotConsumed = 1,
    PartiallyConsumed = 2,
    FullyConsumed = 3,
}