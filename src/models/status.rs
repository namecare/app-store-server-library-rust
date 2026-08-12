use serde_repr::{Deserialize_repr, Serialize_repr};

/// The status of an auto-renewable subscription.
///
/// [status](https://developer.apple.com/documentation/appstoreserverapi/status)
#[derive(Debug, Clone, Deserialize_repr, Serialize_repr, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum Status {
    Active = 1,
    Expired = 2,
    BillingRetry = 3,
    BillingGracePeriod = 4,
    Revoked = 5,
}

impl Status {
    pub fn raw_value(&self) -> u8 {
        match &self {
            Status::Active => 1,
            Status::Expired => 2,
            Status::BillingRetry => 3,
            Status::BillingGracePeriod => 4,
            Status::Revoked => 5,
        }
    }
}
