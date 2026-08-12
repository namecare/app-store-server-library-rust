use serde::{Deserialize, Serialize};

/// The response data for a refund request.
///
/// Unlike the other Advanced Commerce responses, `signedRenewalInfo` is optional here:
/// a one-time-charge refund has no subscription renewal to report.
///
/// [AdvancedCommerceRequestRefundResponse](https://developer.apple.com/documentation/advancedcommerceapi/requestrefundresponse)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedCommerceRequestRefundResponse {
    /// Subscription renewal information signed by the App Store, in JSON Web Signature (JWS) format.
    ///
    /// [signedRenewalInfo](https://developer.apple.com/documentation/advancedcommerceapi/jwsrenewalinfo)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_renewal_info: Option<String>,

    /// Transaction information signed by the App Store, in JWS Compact Serialization format.
    ///
    /// [signedTransactionInfo](https://developer.apple.com/documentation/advancedcommerceapi/jwstransaction)
    pub signed_transaction_info: String,
}
