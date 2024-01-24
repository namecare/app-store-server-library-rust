use serde_repr::{Serialize_repr, Deserialize_repr};

/// The age of the customer’s account.
///
/// [accountTenure](https://developer.apple.com/documentation/appstoreserverapi/accounttenure)
#[derive(Debug, Clone, Deserialize_repr, Serialize_repr, Hash, PartialEq, Eq)]
#[repr(u8)]
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
