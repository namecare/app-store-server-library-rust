use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum AccountTenure {
    Undeclared = 0,
    ZeroToThreeDays = 1,
    ThreeDaysToTenDays = 2,
    TenDaysToThirtyDays = 3,
    ThirtyDaysToNinetyDays = 4,
    NinetyDaysToOneHundredEightyDays = 5,
    OneHundredEightyDaysToThreeHundredSixtyFiveDays = 6,
    GreaterThanThreeHundredSixtyFiveDays = 7,
}
