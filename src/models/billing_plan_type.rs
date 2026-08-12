use serde::{Deserialize, Serialize};

/// The billing plan type for a monthly subscription with a 12-month commitment.
///
/// [BillingPlanType](https://developer.apple.com/documentation/appstoreserverapi/billingplantype)
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BillingPlanType {
    #[serde(rename = "BILLED_UPFRONT")]
    BilledUpfront,
    #[serde(rename = "MONTHLY")]
    Monthly,
}
