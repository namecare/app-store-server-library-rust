use serde::{Deserialize, Serialize};

/// The response data for a subscription metadata change request.
///
/// [SubscriptionChangeMetadataResponse](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionchangemetadataresponse)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionChangeMetadataResponse {
    /// A response that contains signed renewal and transaction information.
    ///
    /// [JWSRenewalInfo](https://developer.apple.com/documentation/appstoreserverapi/jwsrenewalinfo)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_renewal_info: Option<String>,
    
    /// Transaction information signed by the App Store, in JSON Web Signature (JWS) format.
    ///
    /// [JWSTransaction](https://developer.apple.com/documentation/appstoreserverapi/jwstransaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_transaction_info: Option<String>,
}

impl SubscriptionChangeMetadataResponse {
    pub fn new() -> Self {
        Self {
            signed_renewal_info: None,
            signed_transaction_info: None,
        }
    }
    
    pub fn with_signed_renewal_info(mut self, info: String) -> Self {
        self.signed_renewal_info = Some(info);
        self
    }
    
    pub fn with_signed_transaction_info(mut self, info: String) -> Self {
        self.signed_transaction_info = Some(info);
        self
    }
}

impl Default for SubscriptionChangeMetadataResponse {
    fn default() -> Self {
        Self::new()
    }
}