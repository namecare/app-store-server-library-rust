use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum TransactionReason {
    #[serde(rename = "PURCHASE")]
    Purchase,
    #[serde(rename = "RENEWAL")]
    Renewal,
}
