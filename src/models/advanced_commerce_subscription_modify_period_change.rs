use crate::models::advanced_commerce_effective::Effective;
use crate::models::advanced_commerce_period::Period;
use serde::{Deserialize, Serialize};

/// A period change for Advanced Commerce subscription modifications.
///
/// [SubscriptionModifyPeriodChange](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionmodifyperiodchange)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionModifyPeriodChange {
    /// The new period for the subscription.
    ///
    /// [Period](https://developer.apple.com/documentation/advancedcommerceapi/period)
    pub period: Period,

    /// When the period change takes effect.
    ///
    /// [effective](https://developer.apple.com/documentation/advancedcommerceapi/effective)
    pub effective: Effective,
}

impl SubscriptionModifyPeriodChange {
    pub fn new(period: Period, effective: Effective) -> Self {
        Self { period, effective }
    }
}