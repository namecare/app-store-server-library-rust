use crate::primitives::send_attempt_result::SendAttemptResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// The success or error information and the date the App Store server records when it attempts to send a server notification to your server.
///
/// [sendAttemptItem](https://developer.apple.com/documentation/appstoreserverapi/sendattemptitem)
#[derive(Debug, Clone, Deserialize, Serialize, Hash)]
pub struct SendAttemptItem {
    /// The date the App Store server attempts to send a notification.
    ///
    /// [attemptDate](https://developer.apple.com/documentation/appstoreservernotifications/attemptdate)
    pub attempt_date: Option<DateTime<Utc>>,

    /// The success or error information the App Store server records when it attempts to send an App Store server notification to your server.
    ///
    /// [sendAttemptResult](https://developer.apple.com/documentation/appstoreserverapi/sendattemptresult)
    pub send_attempt_result: Option<SendAttemptResult>,
}
