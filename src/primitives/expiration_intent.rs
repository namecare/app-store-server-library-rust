use serde_repr::{Serialize_repr, Deserialize_repr};

/// The reason an auto-renewable subscription expired.
///
/// [expirationIntent](https://developer.apple.com/documentation/appstoreserverapi/expirationintent)
#[derive(Debug, Clone, Deserialize_repr, Serialize_repr, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum ExpirationIntent {
    CustomerCancelled = 1,
    BillingError = 2,
    CustomerDidNotConsentToPriceIncrease = 3,
    ProductNotAvailable = 4,
    Other = 5,
}
