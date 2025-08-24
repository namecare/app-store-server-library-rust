use serde::{Deserialize, Serialize};

/// The type of refund.
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RefundType {
    Full,
    Prorated,
    Custom,
}