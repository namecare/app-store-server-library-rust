use serde::{Deserialize, Serialize};
use crate::primitives::send_attempt_item::SendAttemptItem;

/// The App Store server notification history record, including the signed notification payload and the result of the server’s first send attempt.
///
/// [notificationHistoryResponseItem](https://developer.apple.com/documentation/appstoreserverapi/notificationhistoryresponseitem)
#[derive(Debug, Clone, Deserialize, Serialize, Hash)]
pub struct NotificationHistoryResponseItem {
    /// A cryptographically signed payload, in JSON Web Signature (JWS) format, containing the response body for a version 2 notification.
    ///
    /// [signedPayload](https://developer.apple.com/documentation/appstoreservernotifications/signedpayload)
    #[serde(rename = "signedPayload")]
    pub signed_payload: Option<String>,

    /// An array of information the App Store server records for its attempts to send a notification to your server. The maximum number of entries in the array is six.
    ///
    /// [sendAttemptItem](https://developer.apple.com/documentation/appstoreserverapi/sendattemptitem)
    #[serde(rename = "sendAttempts")]
    pub send_attempts: Option<Vec<SendAttemptItem>>,
}
