use serde::{Deserialize, Serialize};

/// A response that indicates the server successfully received the subscription-renewal-date extension request.
///
/// [MassExtendRenewalDateResponse](https://developer.apple.com/documentation/appstoreserverapi/massextendrenewaldateresponse)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct MassExtendRenewalDateResponse {
    /// A string that contains a unique identifier you provide to track each subscription-renewal-date extension request.
    ///
    /// [requestIdentifier](https://developer.apple.com/documentation/appstoreserverapi/requestidentifier)
    #[serde(rename = "requestIdentifier")]
    pub request_identifier: String,
}
