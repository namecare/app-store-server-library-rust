use serde::{Deserialize, Serialize};

/// The response data for a refund request.
///
/// [RequestRefundResponse](https://developer.apple.com/documentation/advancedcommerceapi/requestrefundresponse)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RequestRefundResponse {
    /// A response that contains signed renewal and transaction information after a refund request.
    ///
    /// [JWSRenewalInfo](https://developer.apple.com/documentation/appstoreserverapi/jwsrenewalinfo)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_renewal_info: Option<String>,
    
    /// Transaction information signed by the App Store, in JSON Web Signature (JWS) Compact Serialization format.
    ///
    /// [JWSTransaction](https://developer.apple.com/documentation/appstoreserverapi/jwstransaction)
    pub signed_transaction_info: String,
}