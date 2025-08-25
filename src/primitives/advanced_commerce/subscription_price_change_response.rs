use serde::{Deserialize, Serialize};

/// A response that contains signed JWS renewal and JWS transaction information after a subscription price change request.
///
/// [SubscriptionPriceChangeResponse](https://developer.apple.com/documentation/advancedcommerceapi/subscriptionpricechangeresponse)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionPriceChangeResponse {
    /// Subscription renewal information signed by the App Store, in JSON Web Signature (JWS) format.
    ///
    /// [signedRenewalInfo](https://developer.apple.com/documentation/advancedcommerceapi/jwsrenewalinfo)
    pub signed_renewal_info: String,

    /// Transaction information signed by the App Store, in JWS Compact Serialization format.
    ///
    /// [signedTransactionInfo](https://developer.apple.com/documentation/advancedcommerceapi/jwstransaction)
    pub signed_transaction_info: String,
}