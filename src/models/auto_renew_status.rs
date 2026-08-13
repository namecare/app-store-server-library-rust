use serde::{Deserialize, Serialize};

/// The renewal status for an auto-renewable subscription.
///
/// [autoRenewStatus](https://developer.apple.com/documentation/appstoreserverapi/autorenewstatus)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(from = "i64", into = "i64")]
pub enum AutoRenewStatus {
    Off,
    On,

    /// A value the App Store sent that this version of the
    /// library does not support, preserved as received.
    NotSupported(i64),
}

impl From<i64> for AutoRenewStatus {
    fn from(value: i64) -> Self {
        match value {
            0 => AutoRenewStatus::Off,
            1 => AutoRenewStatus::On,
            other => AutoRenewStatus::NotSupported(other),
        }
    }
}

impl From<AutoRenewStatus> for i64 {
    fn from(value: AutoRenewStatus) -> Self {
        match value {
            AutoRenewStatus::Off => 0,
            AutoRenewStatus::On => 1,
            AutoRenewStatus::NotSupported(other) => other,
        }
    }
}
