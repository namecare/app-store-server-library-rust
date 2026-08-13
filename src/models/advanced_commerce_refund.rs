use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::formats::Flexible;
use serde_with::TimestampMilliSeconds;

use crate::models::advanced_commerce_refund_reason::AdvancedCommerceRefundReason;
use crate::models::advanced_commerce_refund_type::AdvancedCommerceRefundType;

#[serde_with::serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedCommerceRefund {
    pub refund_amount: Option<i64>,

    #[serde_as(as = "Option<TimestampMilliSeconds<String, Flexible>>")]
    pub refund_date: Option<DateTime<Utc>>,

    pub refund_reason: Option<AdvancedCommerceRefundReason>,

    pub refund_type: Option<AdvancedCommerceRefundType>,
}
