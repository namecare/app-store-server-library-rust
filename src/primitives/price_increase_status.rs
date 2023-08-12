use serde::{Deserialize, Serialize};

/// The status that indicates whether an auto-renewable subscription is subject to a price increase.
///
/// [PriceIncreaseStatus](https://developer.apple.com/documentation/appstoreserverapi/priceincreasestatus)
#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum PriceIncreaseStatus {
    CustomerHasNotResponded = 0,
    CustomerConsentedOrWasNotifiedWithoutNeedingConsent = 1,
}
