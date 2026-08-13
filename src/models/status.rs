use serde::{Deserialize, Serialize};

/// The status of an auto-renewable subscription.
///
/// [status](https://developer.apple.com/documentation/appstoreserverapi/status)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(from = "i64", into = "i64")]
pub enum Status {
    Active,
    Expired,
    BillingRetry,
    BillingGracePeriod,
    Revoked,

    /// A value the App Store sent that this version of the
    /// library does not support, preserved as received.
    NotSupported(i64),
}

impl Status {
    pub fn raw_value(&self) -> i64 {
        i64::from(self.clone())
    }
}

impl From<i64> for Status {
    fn from(value: i64) -> Self {
        match value {
            1 => Status::Active,
            2 => Status::Expired,
            3 => Status::BillingRetry,
            4 => Status::BillingGracePeriod,
            5 => Status::Revoked,
            other => Status::NotSupported(other),
        }
    }
}

impl From<Status> for i64 {
    fn from(value: Status) -> Self {
        match value {
            Status::Active => 1,
            Status::Expired => 2,
            Status::BillingRetry => 3,
            Status::BillingGracePeriod => 4,
            Status::Revoked => 5,
            Status::NotSupported(other) => other,
        }
    }
}
