use crate::primitives::advanced_commerce::period::Period;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::formats::Flexible;
use serde_with::TimestampMilliSeconds;

/// A period change for Advanced Commerce subscription modifications.
///
/// [SubscriptionModifyPeriodChange](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionmodifyperiodchange)
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionModifyPeriodChange {
    /// The new period for the subscription.
    ///
    /// [Period](https://developer.apple.com/documentation/advancedcommerceapi/period)
    pub period: Period,

    /// The effective date for the period change.
    ///
    /// [Effective Date](https://developer.apple.com/documentation/advancedcommerceapi/effectivedate)
    #[serde_as(as = "TimestampMilliSeconds<i64, Flexible>")]
    pub effective: DateTime<Utc>,
}