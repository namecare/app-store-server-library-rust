use serde::{Deserialize, Serialize};

/// A response for Advanced Commerce one-time charge creation.
///
/// [OneTimeChargeCreateResponse](https://developer.apple.com/documentation/advancedcommerceapi)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OneTimeChargeCreateResponse {
    /// A response that contains signed renewal and transaction information after a subscription successfully migrates to the Advanced Commerce API.
    ///
    /// [JWSRenewalInfo](https://developer.apple.com/documentation/appstoreserverapi/jwsrenewalinfo)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_renewal_info: Option<String>,
    
    /// Transaction information signed by the App Store, in JSON Web Signature (JWS) Compact Serialization format.
    ///
    /// [JWSTransaction](https://developer.apple.com/documentation/appstoreserverapi/jwstransaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_transaction_info: Option<String>,
}