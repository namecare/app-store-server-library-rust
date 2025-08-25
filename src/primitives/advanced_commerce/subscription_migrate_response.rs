use serde::{Deserialize, Serialize};

/// The response body for a successful migrate-subscription request.
///
/// [SubscriptionMigrateResponse](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionmigrateresponse)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionMigrateResponse {
    /// Subscription renewal information signed by the App Store, in JSON Web Signature (JWS) format.
    ///
    /// [signedRenewalInfo](https://developer.apple.com/documentation/advancedcommerceapi/jwsrenewalinfo)
    pub signed_renewal_info: String,

    /// Transaction information signed by the App Store, in JWS Compact Serialization format.
    ///
    /// [signedTransactionInfo](https://developer.apple.com/documentation/advancedcommerceapi/jwstransaction)
    pub signed_transaction_info: String,
}