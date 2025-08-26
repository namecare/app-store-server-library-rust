use serde::{Deserialize, Serialize};

/// The duration of a single cycle of an auto-renewable subscription.
///
/// [period](https://developer.apple.com/documentation/advancedcommerceapi/period)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Period {
    /// One week period
    P1W,
    /// One months period
    P1M,
    /// Two months period
    P2M,
    /// Three months period
    P3M,
    /// Six months period
    P6M,
    /// One year period
    P1Y,
}