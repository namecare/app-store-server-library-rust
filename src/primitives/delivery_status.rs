use serde::{Deserialize, Serialize};

/// A value that indicates whether the app successfully delivered an in-app purchase that works properly.
///
/// [deliveryStatus](https://developer.apple.com/documentation/appstoreserverapi/deliverystatus)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum DeliveryStatus {
    DeliveredAndWorkingProperly = 0,
    DidNotDeliverDueToQualityIssue = 1,
    DeliveredWrongItem = 2,
    DidNotDeliverDueToServerOutage = 3,
    DidNotDeliverDueToIngameCurrencyChange = 4,
    DidNotDeliverForOtherReason = 5,
}
