use serde::{Deserialize, Serialize};

/// The type of revocation for a transaction.
///
/// [revocationType](https://developer.apple.com/documentation/appstoreserverapi/revocationtype)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum RevocationType {
    /// The transaction has a full refund.
    #[serde(rename = "REFUND_FULL")]
    RefundFull,
    /// The transaction has a prorated refund.
    #[serde(rename = "REFUND_PRORATED")]
    RefundProrated,
    /// The transaction is revoked from Family Sharing.
    #[serde(rename = "FAMILY_REVOKE")]
    FamilyRevoke,
}