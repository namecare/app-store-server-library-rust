use serde::{Deserialize, Serialize};

/// The reason for the Advanced Commerce request.
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Reason {
    Upgrade,
    Downgrade,
    ApplyOffer,
}