use crate::models::advanced_commerce_effective::AdvancedCommerceEffective;
use crate::models::advanced_commerce_period::AdvancedCommercePeriod;
use serde::{Deserialize, Serialize};

/// A period change for Advanced Commerce subscription modifications.
///
/// [AdvancedCommerceSubscriptionModifyPeriodChange](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionmodifyperiodchange)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedCommerceSubscriptionModifyPeriodChange {
    /// The new period for the subscription.
    ///
    /// [AdvancedCommercePeriod](https://developer.apple.com/documentation/advancedcommerceapi/period)
    pub period: AdvancedCommercePeriod,

    /// When the period change takes effect.
    ///
    /// [effective](https://developer.apple.com/documentation/advancedcommerceapi/effective)
    pub effective: AdvancedCommerceEffective,
}

impl AdvancedCommerceSubscriptionModifyPeriodChange {
    pub fn new(period: AdvancedCommercePeriod, effective: AdvancedCommerceEffective) -> Self {
        Self { period, effective }
    }
}