use serde::{Deserialize, Serialize};

/// The response body that contains the URL of your Get Retention Message endpoint.
///
/// [RealtimeUrlResponse](https://developer.apple.com/documentation/retentionmessaging/realtimeurlresponse)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct RealtimeUrlResponse {
    /// A string that contains the URL of your Get Retention Message endpoint.
    ///
    /// [realtimeURL](https://developer.apple.com/documentation/retentionmessaging/realtimeurl)
    #[serde(rename = "realtimeURL")]
    pub realtime_url: String,
}