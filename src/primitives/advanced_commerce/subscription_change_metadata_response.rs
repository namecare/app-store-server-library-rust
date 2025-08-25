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
    pub signed_renewal_info: String,
    
    /// Transaction information signed by the App Store, in JSON Web Signature (JWS) format.
    ///
    /// [JWSTransaction](https://developer.apple.com/documentation/appstoreserverapi/jwstransaction)
    pub signed_transaction_info: String,
}