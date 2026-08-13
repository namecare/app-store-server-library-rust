use serde::{Deserialize, Serialize};

/// The reason an auto-renewable subscription expired.
///
/// [expirationIntent](https://developer.apple.com/documentation/appstoreserverapi/expirationintent)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(from = "i64", into = "i64")]
pub enum ExpirationIntent {
    CustomerCancelled,
    BillingError,
    CustomerDidNotConsentToPriceIncrease,
    ProductNotAvailable,
    Other,

    /// A value the App Store sent that this version of the
    /// library does not support, preserved as received.
    NotSupported(i64),
}

impl From<i64> for ExpirationIntent {
    fn from(value: i64) -> Self {
        match value {
            1 => ExpirationIntent::CustomerCancelled,
            2 => ExpirationIntent::BillingError,
            3 => ExpirationIntent::CustomerDidNotConsentToPriceIncrease,
            4 => ExpirationIntent::ProductNotAvailable,
            5 => ExpirationIntent::Other,
            other => ExpirationIntent::NotSupported(other),
        }
    }
}

impl From<ExpirationIntent> for i64 {
    fn from(value: ExpirationIntent) -> Self {
        match value {
            ExpirationIntent::CustomerCancelled => 1,
            ExpirationIntent::BillingError => 2,
            ExpirationIntent::CustomerDidNotConsentToPriceIncrease => 3,
            ExpirationIntent::ProductNotAvailable => 4,
            ExpirationIntent::Other => 5,
            ExpirationIntent::NotSupported(other) => other,
        }
    }
}
