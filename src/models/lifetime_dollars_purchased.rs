use serde::{Deserialize, Serialize};

/// A value that indicates the total amount, in USD, of in-app purchases the customer has made in your app, across all platforms.
///
/// [lifetimeDollarsPurchased](https://developer.apple.com/documentation/appstoreserverapi/lifetimedollarspurchased)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(from = "i64", into = "i64")]
pub enum LifetimeDollarsPurchased {
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

impl From<i64> for LifetimeDollarsPurchased {
    fn from(value: i64) -> Self {
        match value {
            0 => LifetimeDollarsPurchased::Undeclared,
            1 => LifetimeDollarsPurchased::ZeroDollars,
            2 => LifetimeDollarsPurchased::OneCentToFortyNineDollarsAndNinetyNineCents,
            3 => LifetimeDollarsPurchased::FiftyDollarsToNinetyNineDollarsAndNinetyNineCents,
            4 => LifetimeDollarsPurchased::OneHundredDollarsToFourHundredNinetyNineDollarsAndNinetyNineCents,
            5 => LifetimeDollarsPurchased::FiveHundredDollarsToNineHundredNinetyNineDollarsAndNinetyNineCents,
            6 => {
                LifetimeDollarsPurchased::OneThousandDollarsToOneThousandNineHundredNinetyNineDollarsAndNinetyNineCents
            }
            7 => LifetimeDollarsPurchased::TwoThousandDollarsOrGreater,
            other => LifetimeDollarsPurchased::NotSupported(other),
        }
    }
}

impl From<LifetimeDollarsPurchased> for i64 {
    fn from(value: LifetimeDollarsPurchased) -> Self {
        match value {
            LifetimeDollarsPurchased::Undeclared => 0,
            LifetimeDollarsPurchased::ZeroDollars => 1,
            LifetimeDollarsPurchased::OneCentToFortyNineDollarsAndNinetyNineCents => 2,
            LifetimeDollarsPurchased::FiftyDollarsToNinetyNineDollarsAndNinetyNineCents => 3,
            LifetimeDollarsPurchased::OneHundredDollarsToFourHundredNinetyNineDollarsAndNinetyNineCents => 4,
            LifetimeDollarsPurchased::FiveHundredDollarsToNineHundredNinetyNineDollarsAndNinetyNineCents => 5,
            LifetimeDollarsPurchased::OneThousandDollarsToOneThousandNineHundredNinetyNineDollarsAndNinetyNineCents => {
                6
            }
            LifetimeDollarsPurchased::TwoThousandDollarsOrGreater => 7,
            LifetimeDollarsPurchased::NotSupported(other) => other,
        }
    }
}
