use serde_repr::{Deserialize_repr, Serialize_repr};

/// The status that indicates whether an auto-renewable subscription is subject to a price increase.
///
/// [PriceIncreaseStatus](https://developer.apple.com/documentation/appstoreserverapi/priceincreasestatus)
#[derive(Debug, Clone, Deserialize_repr, Serialize_repr, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum PriceIncreaseStatus {
    CustomerHasNotResponded = 0,
    CustomerConsentedOrWasNotifiedWithoutNeedingConsent = 1,
}
