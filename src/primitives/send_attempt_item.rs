use crate::primitives::send_attempt_result::SendAttemptResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::formats::Flexible;
use serde_with::TimestampMilliSeconds;
/// The success or error information and the date the App Store server records when it attempts to send a server notification to your server.
///
/// [sendAttemptItem](https://developer.apple.com/documentation/appstoreserverapi/sendattemptitem)
#[serde_with::serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq)]
pub struct SendAttemptItem {
    /// The date the App Store server attempts to send a notification.
    ///
    /// [attemptDate](https://developer.apple.com/documentation/appstoreservernotifications/attemptdate)
    #[serde(rename = "attemptDate")]
    #[serde_as(as = "Option<TimestampMilliSeconds<String, Flexible>>")]
    pub attempt_date: Option<DateTime<Utc>>,

    /// The success or error information the App Store server records when it attempts to send an App Store server notification to your server.
    ///
    /// [sendAttemptResult](https://developer.apple.com/documentation/appstoreserverapi/sendattemptresult)
    #[serde(rename = "sendAttemptResult")]
    pub send_attempt_result: Option<SendAttemptResult>,
}
