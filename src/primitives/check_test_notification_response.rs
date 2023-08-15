use serde::{Deserialize, Serialize};
use crate::primitives::send_attempt_item::SendAttemptItem;

/// A response that contains the contents of the test notification sent by the App Store server and the result from your server.
///
/// [CheckTestNotificationResponse](https://developer.apple.com/documentation/appstoreserverapi/checktestnotificationresponse)
#[derive(Debug, Clone, Deserialize, Serialize, Hash)]
pub struct CheckTestNotificationResponse {
    /// A cryptographically signed payload, in JSON Web Signature (JWS) format, containing the response body for a version 2 notification.
    ///
    /// [signedPayload](https://developer.apple.com/documentation/appstoreservernotifications/signedpayload)
    #[serde(rename = "signedPayload")]
    pub signed_payload: Option<String>,

    /// An array of information the App Store server records for its attempts to send the TEST notification to your server. The array may contain a maximum of six sendAttemptItem objects.
    ///
    /// [sendAttemptItem](https://developer.apple.com/documentation/appstoreserverapi/sendattemptitem)
    #[serde(rename = "sendAttemptItem")]
    pub send_attempts: Option<Vec<SendAttemptItem>>,
}