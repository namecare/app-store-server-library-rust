use serde::{Deserialize, Serialize};

/// The reason for a refunded transaction.
///
/// [revocationReason](https://developer.apple.com/documentation/appstoreserverapi/revocationreason)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(from = "i64", into = "i64")]
pub enum RevocationReason {
    RefundedDueToIssue,
    RefundedForOtherReason,

    /// A value the App Store sent that this version of the
    /// library does not support, preserved as received.
    NotSupported(i64),
}

impl From<i64> for RevocationReason {
    fn from(value: i64) -> Self {
        match value {
            1 => RevocationReason::RefundedDueToIssue,
            0 => RevocationReason::RefundedForOtherReason,
            other => RevocationReason::NotSupported(other),
        }
    }
}

impl From<RevocationReason> for i64 {
    fn from(value: RevocationReason) -> Self {
        match value {
            RevocationReason::RefundedDueToIssue => 1,
            RevocationReason::RefundedForOtherReason => 0,
            RevocationReason::NotSupported(other) => other,
        }
    }
}
