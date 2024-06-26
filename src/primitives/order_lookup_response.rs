use crate::primitives::order_lookup_status::OrderLookupStatus;
use serde::{Deserialize, Serialize};

/// A response that includes the order lookup status and an array of signed transactions for the in-app purchases in the order.
///
/// [OrderLookupResponse](https://developer.apple.com/documentation/appstoreserverapi/orderlookupresponse)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct OrderLookupResponse {
    /// The status that indicates whether the order ID is valid.
    ///
    /// [OrderLookupStatus](https://developer.apple.com/documentation/appstoreserverapi/orderlookupstatus)
    #[serde(rename = "status")]
    pub status: OrderLookupStatus,

    /// An array of in-app purchase transactions that are part of the order, signed by Apple, in JSON Web Signature format.
    ///
    /// [JWSTransaction](https://developer.apple.com/documentation/appstoreserverapi/jwstransaction)
    #[serde(rename = "signedTransactions")]
    pub signed_transactions: Vec<String>,
}
