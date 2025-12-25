use serde::{Deserialize, Serialize};

/// A value that indicates your preferred outcome for the refund request.
///
/// [refundPreference](https://developer.apple.com/documentation/appstoreserverapi/refundpreference)
#[derive(Debug, Clone, PartialEq, Eq, Hash,  Deserialize, Serialize)]
pub enum RefundPreference {
    #[serde(rename = "MIGRATION")]
    Migration,
    #[serde(rename = "GRANT_FULL")]
    GrantFull,
    #[serde(rename = "GRANT_PRORATED")]
    GrantProrated,
}
