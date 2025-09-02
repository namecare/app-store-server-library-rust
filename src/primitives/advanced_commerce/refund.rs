use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::primitives::advanced_commerce::refund_reason::RefundReason;
use crate::primitives::advanced_commerce::refund_type::RefundType;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde_with::serde_as]
#[serde(rename_all = "camelCase")]
pub struct Refund {
    pub refund_amount: i64,

    #[serde_as(as = "TimestampMilliSeconds<String, Flexible>")]
    pub refund_date: DateTime<Utc>,

    pub refund_reason: RefundReason,

    pub refund_type: RefundType,
}