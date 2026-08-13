use serde::{Deserialize, Serialize};

/// The platform on which the customer consumed the in-app purchase.
///
/// [platform](https://developer.apple.com/documentation/appstoreserverapi/platform)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(from = "i64", into = "i64")]
pub enum Platform {
    Undeclared,
    Apple,
    NonApple,

    /// A value the App Store sent that this version of the
    /// library does not support, preserved as received.
    NotSupported(i64),
}

impl From<i64> for Platform {
    fn from(value: i64) -> Self {
        match value {
            0 => Platform::Undeclared,
            1 => Platform::Apple,
            2 => Platform::NonApple,
            other => Platform::NotSupported(other),
        }
    }
}

impl From<Platform> for i64 {
    fn from(value: Platform) -> Self {
        match value {
            Platform::Undeclared => 0,
            Platform::Apple => 1,
            Platform::NonApple => 2,
            Platform::NotSupported(other) => other,
        }
    }
}
