use crate::primitives::status::Status;
use serde::{Deserialize, Serialize};

/// The most recent App Store-signed transaction information and App Store-signed renewal information for an auto-renewable subscription.
///
/// [lastTransactionsItem](https://developer.apple.com/documentation/appstoreserverapi/lasttransactionsitem)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq)]
pub struct LastTransactionsItem {
    /// The status of the auto-renewable subscription.
    ///
    /// [status](https://developer.apple.com/documentation/appstoreserverapi/status)
    #[serde(rename = "status")]
    pub status: Option<Status>,

    /// The original transaction identifier of a purchase.
    ///
    /// [originalTransactionId](https://developer.apple.com/documentation/appstoreserverapi/originaltransactionid)
    #[serde(rename = "originalTransactionId")]
    pub original_transaction_id: Option<String>,

    /// Transaction information signed by the App Store, in JSON Web Signature (JWS) format.
    ///
    /// [JWSTransaction](https://developer.apple.com/documentation/appstoreserverapi/jwstransaction)
    #[serde(rename = "signedTransactionInfo")]
    pub signed_transaction_info: Option<String>,

    /// Subscription renewal information, signed by the App Store, in JSON Web Signature (JWS) format.
    ///
    /// [JWSRenewalInfo](https://developer.apple.com/documentation/appstoreserverapi/jwsrenewalinfo)
    #[serde(rename = "signedRenewalInfo")]
    pub signed_renewal_info: Option<String>,
}
