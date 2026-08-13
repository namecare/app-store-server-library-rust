use serde::{Deserialize, Serialize};

/// A value that indicates the dollar amount of refunds the customer has received in your app, since purchasing the app, across all platforms.
///
/// [lifetimeDollarsRefunded](https://developer.apple.com/documentation/appstoreserverapi/lifetimedollarsrefunded)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(from = "i64", into = "i64")]
pub enum LifetimeDollarsRefunded {
    Undeclared,
    ZeroDollars,
    OneCentToFortyNineDollarsAndNinetyNineCents,
    FiftyDollarsToNinetyNineDollarsAndNinetyNineCents,
    OneHundredDollarsToFourHundredNinetyNineDollarsAndNinetyNineCents,
    FiveHundredDollarsToNineHundredNinetyNineDollarsAndNinetyNineCents,
    OneThousandDollarsToOneThousandNineHundredNinetyNineDollarsAndNinetyNineCents,
    TwoThousandDollarsOrGreater,

    /// A value the App Store sent that this version of the
    /// library does not support, preserved as received.
    NotSupported(i64),
}

impl From<i64> for LifetimeDollarsRefunded {
    fn from(value: i64) -> Self {
        match value {
            0 => LifetimeDollarsRefunded::Undeclared,
            1 => LifetimeDollarsRefunded::ZeroDollars,
            2 => LifetimeDollarsRefunded::OneCentToFortyNineDollarsAndNinetyNineCents,
            3 => LifetimeDollarsRefunded::FiftyDollarsToNinetyNineDollarsAndNinetyNineCents,
            4 => LifetimeDollarsRefunded::OneHundredDollarsToFourHundredNinetyNineDollarsAndNinetyNineCents,
            5 => LifetimeDollarsRefunded::FiveHundredDollarsToNineHundredNinetyNineDollarsAndNinetyNineCents,
            6 => LifetimeDollarsRefunded::OneThousandDollarsToOneThousandNineHundredNinetyNineDollarsAndNinetyNineCents,
            7 => LifetimeDollarsRefunded::TwoThousandDollarsOrGreater,
            other => LifetimeDollarsRefunded::NotSupported(other),
        }
    }
}

impl From<LifetimeDollarsRefunded> for i64 {
    fn from(value: LifetimeDollarsRefunded) -> Self {
        match value {
            LifetimeDollarsRefunded::Undeclared => 0,
            LifetimeDollarsRefunded::ZeroDollars => 1,
            LifetimeDollarsRefunded::OneCentToFortyNineDollarsAndNinetyNineCents => 2,
            LifetimeDollarsRefunded::FiftyDollarsToNinetyNineDollarsAndNinetyNineCents => 3,
            LifetimeDollarsRefunded::OneHundredDollarsToFourHundredNinetyNineDollarsAndNinetyNineCents => 4,
            LifetimeDollarsRefunded::FiveHundredDollarsToNineHundredNinetyNineDollarsAndNinetyNineCents => 5,
            LifetimeDollarsRefunded::OneThousandDollarsToOneThousandNineHundredNinetyNineDollarsAndNinetyNineCents => 6,
            LifetimeDollarsRefunded::TwoThousandDollarsOrGreater => 7,
            LifetimeDollarsRefunded::NotSupported(other) => other,
        }
    }
}
