use serde::{Deserialize, Serialize};

/// A value that indicates the total amount, in USD, of in-app purchases the customer has made in your app, across all platforms.
///
/// [lifetimeDollarsPurchased](https://developer.apple.com/documentation/appstoreserverapi/lifetimedollarspurchased)
#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum LifetimeDollarsPurchased {
    Undeclared = 0,
    ZeroDollars = 1,
    OneCentToFortyNineDollarsAndNinetyNineCents = 2,
    FiftyDollarsToNinetyNineDollarsAndNinetyNineCents = 3,
    OneHundredDollarsToFourHundredNinetyNineDollarsAndNinetyNineCents = 4,
    FiveHundredDollarsToNineHundredNinetyNineDollarsAndNinetyNineCents = 5,
    OneThousandDollarsToOneThousandNineHundredNinetyNineDollarsAndNinetyNineCents = 6,
    TwoThousandDollarsOrGreater = 7,
}
