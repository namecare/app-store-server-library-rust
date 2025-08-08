use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The request body for the Set App Account Token endpoint.
///
/// # References
/// [UpdateAppAccountTokenRequest](https://developer.apple.com/documentation/appstoreserverapi/updateappaccounttokenrequest)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAppAccountTokenRequest {
    /// The UUID that an app optionally generates to map a customer's in-app purchase with its resulting App Store transaction.
    ///
    /// # References
    /// [appAccountToken](https://developer.apple.com/documentation/appstoreserverapi/appaccounttoken)
    pub app_account_token: Uuid,
}

impl UpdateAppAccountTokenRequest {
    /// Creates a new UpdateAppAccountTokenRequest with the specified app account token.
    pub fn new(app_account_token: Uuid) -> Self {
        Self { app_account_token }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialization() {
        let token = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let request = UpdateAppAccountTokenRequest::new(token);
        
        let json = serde_json::to_string(&request).unwrap();
        let expected = r#"{"appAccountToken":"550e8400-e29b-41d4-a716-446655440000"}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{"appAccountToken":"550e8400-e29b-41d4-a716-446655440000"}"#;
        let request: UpdateAppAccountTokenRequest = serde_json::from_str(json).unwrap();
        
        assert_eq!(
            request.app_account_token.to_string(),
            "550e8400-e29b-41d4-a716-446655440000"
        );
    }
}