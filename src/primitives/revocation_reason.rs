use serde_repr::{Deserialize_repr, Serialize_repr};

/// The reason for a refunded transaction.
///
/// [revocationReason](https://developer.apple.com/documentation/appstoreserverapi/revocationreason)
#[derive(Debug, Clone, Deserialize_repr, Serialize_repr, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum RevocationReason {
    RefundedDueToIssue = 1,
    RefundedForOtherReason = 0,
}
