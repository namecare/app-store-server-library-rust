use serde::{Deserialize, Serialize};

/// The age of the customer’s account.
///
/// [accountTenure](https://developer.apple.com/documentation/appstoreserverapi/accounttenure)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(from = "i64", into = "i64")]
pub enum AccountTenure {
    Undeclared,
    ZeroToThreeDays,
    ThreeDaysToTenDays,
    TenDaysToThirtyDays,
    ThirtyDaysToNinetyDays,
    NinetyDaysToOneHundredEightyDays,
    OneHundredEightyDaysToThreeHundredSixtyFiveDays,
    GreaterThanThreeHundredSixtyFiveDays,

    /// A value the App Store sent that this version of the
    /// library does not support, preserved as received.
    NotSupported(i64),
}

impl From<i64> for AccountTenure {
    fn from(value: i64) -> Self {
        match value {
            0 => AccountTenure::Undeclared,
            1 => AccountTenure::ZeroToThreeDays,
            2 => AccountTenure::ThreeDaysToTenDays,
            3 => AccountTenure::TenDaysToThirtyDays,
            4 => AccountTenure::ThirtyDaysToNinetyDays,
            5 => AccountTenure::NinetyDaysToOneHundredEightyDays,
            6 => AccountTenure::OneHundredEightyDaysToThreeHundredSixtyFiveDays,
            7 => AccountTenure::GreaterThanThreeHundredSixtyFiveDays,
            other => AccountTenure::NotSupported(other),
        }
    }
}

impl From<AccountTenure> for i64 {
    fn from(value: AccountTenure) -> Self {
        match value {
            AccountTenure::Undeclared => 0,
            AccountTenure::ZeroToThreeDays => 1,
            AccountTenure::ThreeDaysToTenDays => 2,
            AccountTenure::TenDaysToThirtyDays => 3,
            AccountTenure::ThirtyDaysToNinetyDays => 4,
            AccountTenure::NinetyDaysToOneHundredEightyDays => 5,
            AccountTenure::OneHundredEightyDaysToThreeHundredSixtyFiveDays => 6,
            AccountTenure::GreaterThanThreeHundredSixtyFiveDays => 7,
            AccountTenure::NotSupported(other) => other,
        }
    }
}
