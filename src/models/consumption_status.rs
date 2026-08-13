use serde::{Deserialize, Serialize};

/// A value that indicates the extent to which the customer consumed the in-app purchase.
///
/// [consumptionStatus](https://developer.apple.com/documentation/appstoreserverapi/consumptionstatus)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(from = "i64", into = "i64")]
pub enum ConsumptionStatus {
    Undeclared,
    NotConsumed,
    PartiallyConsumed,
    FullyConsumed,

    /// A value the App Store sent that this version of the
    /// library does not support, preserved as received.
    NotSupported(i64),
}

impl From<i64> for ConsumptionStatus {
    fn from(value: i64) -> Self {
        match value {
            0 => ConsumptionStatus::Undeclared,
            1 => ConsumptionStatus::NotConsumed,
            2 => ConsumptionStatus::PartiallyConsumed,
            3 => ConsumptionStatus::FullyConsumed,
            other => ConsumptionStatus::NotSupported(other),
        }
    }
}

impl From<ConsumptionStatus> for i64 {
    fn from(value: ConsumptionStatus) -> Self {
        match value {
            ConsumptionStatus::Undeclared => 0,
            ConsumptionStatus::NotConsumed => 1,
            ConsumptionStatus::PartiallyConsumed => 2,
            ConsumptionStatus::FullyConsumed => 3,
            ConsumptionStatus::NotSupported(other) => other,
        }
    }
}
