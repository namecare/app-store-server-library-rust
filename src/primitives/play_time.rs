use serde_repr::{Serialize_repr, Deserialize_repr};

/// A value that indicates the amount of time that the customer used the app.
///
/// [playTime](https://developer.apple.com/documentation/appstoreserverapi/playtime)
#[derive(Debug, Clone, Deserialize_repr, Serialize_repr, Hash, PartialEq, Eq)]
#[repr(u8)]
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
