use serde_repr::{Serialize_repr, Deserialize_repr};

/// The renewal status for an auto-renewable subscription.
///
/// [autoRenewStatus](https://developer.apple.com/documentation/appstoreserverapi/autorenewstatus)
#[derive(Debug, Clone, Deserialize_repr, Serialize_repr, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum AutoRenewStatus {
    Off = 0,
    On = 1,
}
