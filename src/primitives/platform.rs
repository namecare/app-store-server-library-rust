use serde_repr::{Serialize_repr, Deserialize_repr};

/// The platform on which the customer consumed the in-app purchase.
///
/// [platform](https://developer.apple.com/documentation/appstoreserverapi/platform)
#[derive(Debug, Clone, Deserialize_repr, Serialize_repr, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum Platform {
    Undeclared = 0,
    Apple = 1,
    NonApple = 2,
}
