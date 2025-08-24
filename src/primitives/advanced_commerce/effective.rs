use serde::{Deserialize, Serialize};

/// A string value that indicates when a requested change to an auto-renewable subscription goes into effect.
///
/// [effective](https://developer.apple.com/documentation/advancedcommerceapi/effective)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Effective {
    Immediately,
    NextBillCycle,
}