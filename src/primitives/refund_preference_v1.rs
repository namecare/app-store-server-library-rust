use serde_repr::{Deserialize_repr, Serialize_repr};

/// A value that indicates your preferred outcome for the refund request.
///
/// [refundPreference](https://developer.apple.com/documentation/appstoreserverapi/refundpreference)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum RefundPreferenceV1 {
    Undeclared = 0,
    PreferGrant = 1,
    PreferDecline = 2,
    NoPreference = 3,
}
