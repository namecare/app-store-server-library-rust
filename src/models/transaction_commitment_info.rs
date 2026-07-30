use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::formats::Flexible;
use serde_with::TimestampMilliSeconds;

/// The inclusive bounds Apple enforces on a commitment's billing period number.
const MINIMUM_PERIOD: i32 = 1;
const MAXIMUM_PERIOD: i32 = 12;

/// Information about a subscription commitment.
///
/// [TransactionCommitmentInfo](https://developer.apple.com/documentation/appstoreserverapi/transactioncommitmentinfo)
#[serde_with::serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, Hash)]
#[serde(rename_all = "camelCase")]
pub struct TransactionCommitmentInfo {
    /// The number of the billing period when the commitment expires.
    ///
    /// [billingPeriodNumber](https://developer.apple.com/documentation/appstoreserverapi/billingperiodnumber)
    pub billing_period_number: Option<i32>,

    /// The UNIX time, in milliseconds, when the commitment expires.
    ///
    /// [commitmentExpiresDate](https://developer.apple.com/documentation/appstoreserverapi/commitmentexpiresdate)
    #[serde_as(as = "Option<TimestampMilliSeconds<String, Flexible>>")]
    pub commitment_expires_date: Option<DateTime<Utc>>,

    /// The price of the subscription after the commitment period ends.
    ///
    /// [commitmentPrice](https://developer.apple.com/documentation/appstoreserverapi/commitmentprice)
    pub commitment_price: Option<i64>,

    /// The total number of billing periods in the commitment.
    ///
    /// [totalBillingPeriods](https://developer.apple.com/documentation/appstoreserverapi/totalbillingperiods)
    pub total_billing_periods: Option<i32>,
}

impl TransactionCommitmentInfo {
    /// Validates `billing_period_number`, the only field Apple's library validates.
    ///
    /// # Errors
    ///
    /// Returns `TransactionCommitmentInfoValidationError::BillingPeriodNumberOutOfRange`
    /// when `billing_period_number` is present and outside 1..=12. A `None` value is
    /// valid, matching Swift, which skips validation when the field is absent.
    pub fn validate(&self) -> Result<(), TransactionCommitmentInfoValidationError> {
        if let Some(billing_period) = self.billing_period_number {
            if billing_period < MINIMUM_PERIOD || billing_period > MAXIMUM_PERIOD {
                return Err(
                    TransactionCommitmentInfoValidationError::BillingPeriodNumberOutOfRange {
                        value: billing_period,
                    },
                );
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionCommitmentInfoValidationError {
    BillingPeriodNumberOutOfRange { value: i32 },
}

impl std::fmt::Display for TransactionCommitmentInfoValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionCommitmentInfoValidationError::BillingPeriodNumberOutOfRange { value } => {
                write!(
                    f,
                    "billingPeriodNumber must be between {} and {} inclusive, got {}",
                    MINIMUM_PERIOD, MAXIMUM_PERIOD, value
                )
            }
        }
    }
}

impl std::error::Error for TransactionCommitmentInfoValidationError {}
