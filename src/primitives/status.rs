use serde::{Deserialize, Serialize};

/// The status of an auto-renewable subscription.
///
/// [status](https://developer.apple.com/documentation/appstoreserverapi/status)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum Status {
    Active = 1,
    Expired = 2,
    BillingRetry = 3,
    BillingGracePeriod = 4,
    Revoked = 5,
}
