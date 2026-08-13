use serde::{Deserialize, Serialize};

/// The status that indicates whether an auto-renewable subscription is subject to a price increase.
///
/// [PriceIncreaseStatus](https://developer.apple.com/documentation/appstoreserverapi/priceincreasestatus)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(from = "i64", into = "i64")]
pub enum PriceIncreaseStatus {
    CustomerHasNotResponded,
    CustomerConsentedOrWasNotifiedWithoutNeedingConsent,

    /// A value the App Store sent that this version of the
    /// library does not support, preserved as received.
    NotSupported(i64),
}

impl From<i64> for PriceIncreaseStatus {
    fn from(value: i64) -> Self {
        match value {
            0 => PriceIncreaseStatus::CustomerHasNotResponded,
            1 => PriceIncreaseStatus::CustomerConsentedOrWasNotifiedWithoutNeedingConsent,
            other => PriceIncreaseStatus::NotSupported(other),
        }
    }
}

impl From<PriceIncreaseStatus> for i64 {
    fn from(value: PriceIncreaseStatus) -> Self {
        match value {
            PriceIncreaseStatus::CustomerHasNotResponded => 0,
            PriceIncreaseStatus::CustomerConsentedOrWasNotifiedWithoutNeedingConsent => 1,
            PriceIncreaseStatus::NotSupported(other) => other,
        }
    }
}
