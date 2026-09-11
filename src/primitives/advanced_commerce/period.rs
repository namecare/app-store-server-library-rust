use serde::{Deserialize, Serialize};

/// The duration of a single cycle of an auto-renewable subscription.
///
/// Wire values are ISO 8601 durations (`P1M`), so each variant is renamed explicitly:
/// `rename_all = "SCREAMING_SNAKE_CASE"` would split the digit boundary into `P1_M`.
///
/// [period](https://developer.apple.com/documentation/advancedcommerceapi/period)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Period {
    /// One week period
    #[serde(rename = "P1W")]
    P1W,
    /// One months period
    #[serde(rename = "P1M")]
    P1M,
    /// Two months period
    #[serde(rename = "P2M")]
    P2M,
    /// Three months period
    #[serde(rename = "P3M")]
    P3M,
    /// Six months period
    #[serde(rename = "P6M")]
    P6M,
    /// One year period
    #[serde(rename = "P1Y")]
    P1Y,
}
