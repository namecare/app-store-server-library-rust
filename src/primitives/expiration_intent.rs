use serde::{Deserialize, Serialize};

/// The reason an auto-renewable subscription expired.
///
/// [expirationIntent](https://developer.apple.com/documentation/appstoreserverapi/expirationintent)
#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum ExpirationIntent {
    CustomerCancelled = 1,
    BillingError = 2,
    CustomerDidNotConsentToPriceIncrease = 3,
    ProductNotAvailable = 4,
    Other = 5,
}
