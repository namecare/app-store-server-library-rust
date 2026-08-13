use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(from = "i64", into = "i64")]
pub enum UserStatus {
    Undeclared,
    Active,
    Suspended,
    Terminated,
    LimitedAccess,

    /// A value the App Store sent that this version of the
    /// library does not support, preserved as received.
    NotSupported(i64),
}

impl From<i64> for UserStatus {
    fn from(value: i64) -> Self {
        match value {
            0 => UserStatus::Undeclared,
            1 => UserStatus::Active,
            2 => UserStatus::Suspended,
            3 => UserStatus::Terminated,
            4 => UserStatus::LimitedAccess,
            other => UserStatus::NotSupported(other),
        }
    }
}

impl From<UserStatus> for i64 {
    fn from(value: UserStatus) -> Self {
        match value {
            UserStatus::Undeclared => 0,
            UserStatus::Active => 1,
            UserStatus::Suspended => 2,
            UserStatus::Terminated => 3,
            UserStatus::LimitedAccess => 4,
            UserStatus::NotSupported(other) => other,
        }
    }
}
