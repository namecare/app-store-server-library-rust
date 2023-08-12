use serde::{Deserialize, Serialize};

/// A value that indicates the dollar amount of refunds the customer has received in your app, since purchasing the app, across all platforms.
///
/// [lifetimeDollarsRefunded](https://developer.apple.com/documentation/appstoreserverapi/lifetimedollarsrefunded)
#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum LifetimeDollarsRefunded {
    Undeclared = 0,
    ZeroDollars = 1,
    OneCentToFortyNineDollarsAndNinetyNineCents = 2,
    FiftyDollarsToNinetyNineDollarsAndNinetyNineCents = 3,
    OneHundredDollarsToFourHundredNinetyNineDollarsAndNinetyNineCents = 4,
    FiveHundredDollarsToNineHundredNinetyNineDollarsAndNinetyNineCents = 5,
    OneThousandDollarsToOneThousandNineHundredNinetyNineDollarsAndNinetyNineCents = 6,
    TwoThousandDollarsOrGreater = 7,
}
