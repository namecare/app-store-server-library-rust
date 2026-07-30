use serde::{Deserialize, Serialize};

/// A response that contains the test notification token.
///
/// [SendTestNotificationResponse](https://developer.apple.com/documentation/appstoreserverapi/sendtestnotificationresponse)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct SendTestNotificationResponse {
    /// A unique identifier for a notification test that the App Store server sends to your server.
    ///
    /// [testNotificationToken](https://developer.apple.com/documentation/appstoreserverapi/testnotificationtoken)
    #[serde(rename = "testNotificationToken")]
    pub test_notification_token: Option<String>,
}
