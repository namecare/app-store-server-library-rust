use serde::{Deserialize, Serialize};

/// A value that indicates the amount of time that the customer used the app.
///
/// [playTime](https://developer.apple.com/documentation/appstoreserverapi/playtime)
#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum PlayTime {
    Undeclared = 0,
    ZeroToFiveMinutes = 1,
    FiveToSixtyMinutes = 2,
    OneToSixHours = 3,
    SixHoursToTwentyFourHours = 4,
    OneDayToFourDays = 5,
    FourDaysToSixteenDays = 6,
    OverSixteenDays = 7,
}
