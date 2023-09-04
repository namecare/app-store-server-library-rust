use serde::{Deserialize, Serialize};

/// The response body the App Store sends in a version 2 server notification.
///
/// [responseBodyV2](https://developer.apple.com/documentation/appstoreservernotifications/responsebodyv2)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct ResponseBodyV2 {
    /// A cryptographically signed payload, in JSON Web Signature (JWS) format, containing the response body for a version 2 notification.
    ///
    /// [signedPayload](https://developer.apple.com/documentation/appstoreservernotifications/signedpayload)
    #[serde(rename = "signedPayload")]
    pub signed_payload: Option<String>,
}
