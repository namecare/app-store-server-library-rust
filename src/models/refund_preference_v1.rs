use serde::{Deserialize, Serialize};

/// A value that indicates your preferred outcome for the refund request.
///
/// [refundPreference](https://developer.apple.com/documentation/appstoreserverapi/refundpreference)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "i64", into = "i64")]
pub enum RefundPreferenceV1 {
    Undeclared,
    PreferGrant,
    PreferDecline,
    NoPreference,

    /// A value the App Store sent that this version of the
    /// library does not support, preserved as received.
    NotSupported(i64),
}

impl From<i64> for RefundPreferenceV1 {
    fn from(value: i64) -> Self {
        match value {
            0 => RefundPreferenceV1::Undeclared,
            1 => RefundPreferenceV1::PreferGrant,
            2 => RefundPreferenceV1::PreferDecline,
            3 => RefundPreferenceV1::NoPreference,
            other => RefundPreferenceV1::NotSupported(other),
        }
    }
}

impl From<RefundPreferenceV1> for i64 {
    fn from(value: RefundPreferenceV1) -> Self {
        match value {
            RefundPreferenceV1::Undeclared => 0,
            RefundPreferenceV1::PreferGrant => 1,
            RefundPreferenceV1::PreferDecline => 2,
            RefundPreferenceV1::NoPreference => 3,
            RefundPreferenceV1::NotSupported(other) => other,
        }
    }
}
