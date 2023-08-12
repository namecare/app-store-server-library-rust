use serde::{Deserialize, Serialize};

/// The reason for a refunded transaction.
///
/// [revocationReason](https://developer.apple.com/documentation/appstoreserverapi/revocationreason)
#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum RevocationReason {
    RefundedDueToIssue = 1,
    RefundedForOtherReason = 0,
}
