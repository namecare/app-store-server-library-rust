use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The metadata to include in Advanced Commerce server requests.
///
/// [RequestInfo](https://developer.apple.com/documentation/advancedcommerceapi/requestinfo)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RequestInfo {
    /// The app account token for the request.
    ///
    /// [App Account Token](https://developer.apple.com/documentation/advancedcommerceapi/appaccounttoken)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_account_token: Option<Uuid>,
    
    /// The consistency token for the request.
    ///
    /// [Consistency Token](https://developer.apple.com/documentation/advancedcommerceapi/consistencytoken)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_token: Option<String>,
    
    /// The request reference identifier.
    ///
    /// [Request Reference ID](https://developer.apple.com/documentation/advancedcommerceapi/requestreferenceid)
    pub request_reference_id: Uuid,
}

impl RequestInfo {
    pub fn new(request_reference_id: Uuid) -> Self {
        Self {
            app_account_token: None,
            consistency_token: None,
            request_reference_id,
        }
    }
    
    pub fn with_app_account_token(mut self, token: Uuid) -> Self {
        self.app_account_token = Some(token);
        self
    }
    
    pub fn with_consistency_token(mut self, token: String) -> Self {
        self.consistency_token = Some(token);
        self
    }
}