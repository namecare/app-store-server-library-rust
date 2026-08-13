use serde::{Deserialize, Serialize};

/// A value that indicates the amount of time that the customer used the app.
///
/// [playTime](https://developer.apple.com/documentation/appstoreserverapi/playtime)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(from = "i64", into = "i64")]
pub enum PlayTime {
    Undeclared,
    ZeroToFiveMinutes,
    FiveToSixtyMinutes,
    OneToSixHours,
    SixHoursToTwentyFourHours,
    OneDayToFourDays,
    FourDaysToSixteenDays,
    OverSixteenDays,

    /// A value the App Store sent that this version of the
    /// library does not support, preserved as received.
    NotSupported(i64),
}

impl From<i64> for PlayTime {
    fn from(value: i64) -> Self {
        match value {
            0 => PlayTime::Undeclared,
            1 => PlayTime::ZeroToFiveMinutes,
            2 => PlayTime::FiveToSixtyMinutes,
            3 => PlayTime::OneToSixHours,
            4 => PlayTime::SixHoursToTwentyFourHours,
            5 => PlayTime::OneDayToFourDays,
            6 => PlayTime::FourDaysToSixteenDays,
            7 => PlayTime::OverSixteenDays,
            other => PlayTime::NotSupported(other),
        }
    }
}

impl From<PlayTime> for i64 {
    fn from(value: PlayTime) -> Self {
        match value {
            PlayTime::Undeclared => 0,
            PlayTime::ZeroToFiveMinutes => 1,
            PlayTime::FiveToSixtyMinutes => 2,
            PlayTime::OneToSixHours => 3,
            PlayTime::SixHoursToTwentyFourHours => 4,
            PlayTime::OneDayToFourDays => 5,
            PlayTime::FourDaysToSixteenDays => 6,
            PlayTime::OverSixteenDays => 7,
            PlayTime::NotSupported(other) => other,
        }
    }
}
