use serde::{Deserialize, Serialize};

/// A response that contains an array of signed JSON Web Signature (JWS) refunded transactions, and paging information.
///
/// [RefundHistoryResponse](https://developer.apple.com/documentation/appstoreserverapi/refundhistoryresponse)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct RefundHistoryResponse {
    /// A list of up to 20 JWS transactions, or an empty array if the customer hasn't received any refunds in your app. The transactions are sorted in ascending order by revocationDate.
    #[serde(rename = "signedTransactions")]
    pub signed_transactions: Vec<String>,

    /// A token you use in a query to request the next set of transactions for the customer.
    ///
    /// [revision](https://developer.apple.com/documentation/appstoreserverapi/revision)
    pub revision: String,

    /// A Boolean value indicating whether the App Store has more transaction data.
    ///
    /// [hasMore](https://developer.apple.com/documentation/appstoreserverapi/hasmore)
    #[serde(rename = "hasMore")]
    pub has_more: bool,
}
