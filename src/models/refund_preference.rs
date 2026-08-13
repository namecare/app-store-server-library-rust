use serde::{Deserialize, Serialize};

/// A value that indicates your preferred outcome for the refund request.
///
/// [refundPreference](https://developer.apple.com/documentation/appstoreserverapi/refundpreference)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum RefundPreference {
    #[serde(rename = "DECLINE")]
    Declined,
    #[serde(rename = "GRANT_FULL")]
    GrantFull,
    #[serde(rename = "GRANT_PRORATED")]
    GrantProrated,

    /// A value the App Store sent that this version of the
    /// library does not support, preserved as received.
    #[serde(untagged)]
    NotSupported(String),
}
