use serde::{Deserialize, Serialize};

/// The platform on which the customer consumed the in-app purchase.
///
/// [platform](https://developer.apple.com/documentation/appstoreserverapi/platform)
#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum Platform {
    Undeclared = 0,
    Apple = 1,
    NonApple = 2,
}
