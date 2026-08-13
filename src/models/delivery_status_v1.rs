use serde::{Deserialize, Serialize};

/// A value that indicates whether the app successfully delivered an in-app purchase that works properly.
///
/// [deliveryStatus](https://developer.apple.com/documentation/appstoreserverapi/deliverystatus)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(from = "i64", into = "i64")]
pub enum DeliveryStatusV1 {
    DeliveredAndWorkingProperly,
    DidNotDeliverDueToQualityIssue,
    DeliveredWrongItem,
    DidNotDeliverDueToServerOutage,
    DidNotDeliverDueToIngameCurrencyChange,
    DidNotDeliverForOtherReason,

    /// A value the App Store sent that this version of the
    /// library does not support, preserved as received.
    NotSupported(i64),
}

impl From<i64> for DeliveryStatusV1 {
    fn from(value: i64) -> Self {
        match value {
            0 => DeliveryStatusV1::DeliveredAndWorkingProperly,
            1 => DeliveryStatusV1::DidNotDeliverDueToQualityIssue,
            2 => DeliveryStatusV1::DeliveredWrongItem,
            3 => DeliveryStatusV1::DidNotDeliverDueToServerOutage,
            4 => DeliveryStatusV1::DidNotDeliverDueToIngameCurrencyChange,
            5 => DeliveryStatusV1::DidNotDeliverForOtherReason,
            other => DeliveryStatusV1::NotSupported(other),
        }
    }
}

impl From<DeliveryStatusV1> for i64 {
    fn from(value: DeliveryStatusV1) -> Self {
        match value {
            DeliveryStatusV1::DeliveredAndWorkingProperly => 0,
            DeliveryStatusV1::DidNotDeliverDueToQualityIssue => 1,
            DeliveryStatusV1::DeliveredWrongItem => 2,
            DeliveryStatusV1::DidNotDeliverDueToServerOutage => 3,
            DeliveryStatusV1::DidNotDeliverDueToIngameCurrencyChange => 4,
            DeliveryStatusV1::DidNotDeliverForOtherReason => 5,
            DeliveryStatusV1::NotSupported(other) => other,
        }
    }
}
