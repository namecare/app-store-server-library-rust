use serde::{Deserialize, Serialize};

/// The request body the App Store server sends to your Get Retention Message endpoint.
///
/// [RealtimeRequestBody](https://developer.apple.com/documentation/retentionmessaging/realtimerequestbody)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct RealtimeRequestBody {
    /// The payload in JSON Web Signature (JWS) format, signed by the App Store.
    ///
    /// [signedPayload](https://developer.apple.com/documentation/retentionmessaging/signedpayload)
    #[serde(rename = "signedPayload")]
    pub signed_payload: Option<String>,
}