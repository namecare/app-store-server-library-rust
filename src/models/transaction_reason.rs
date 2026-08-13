use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum TransactionReason {
    #[serde(rename = "PURCHASE")]
    Purchase,
    #[serde(rename = "RENEWAL")]
    Renewal,

    /// A value the App Store sent that this version of the
    /// library does not support, preserved as received.
    #[serde(untagged)]
    NotSupported(String),
}
