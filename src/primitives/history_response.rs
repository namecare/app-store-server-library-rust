use serde::{Deserialize, Serialize};
use crate::primitives::environment::Environment;

/// A response that contains the customer’s transaction history for an app.
///
/// [HistoryResponse](https://developer.apple.com/documentation/appstoreserverapi/historyresponse)
#[derive(Debug, Deserialize, Serialize, Hash)]
pub struct HistoryResponse {
    /// A token you use in a query to request the next set of transactions for the customer.
    ///
    /// [revision](https://developer.apple.com/documentation/appstoreserverapi/revision)
    pub revision: Option<String>,

    /// A Boolean value indicating whether the App Store has more transaction data.
    ///
    /// [hasMore](https://developer.apple.com/documentation/appstoreserverapi/hasmore)
    pub has_more: Option<bool>,

    /// The bundle identifier of an app.
    ///
    /// [bundle_id](https://developer.apple.com/documentation/appstoreserverapi/bundleid)
    pub bundle_id: Option<String>,

    /// The unique identifier of an app in the App Store.
    ///
    /// [app_apple_id](https://developer.apple.com/documentation/appstoreservernotifications/appappleid)
    pub app_apple_id: Option<i64>,

    /// The server environment in which you’re making the request, whether sandbox or production.
    ///
    /// [environment](https://developer.apple.com/documentation/appstoreserverapi/environment)
    pub environment: Option<Environment>,

    /// An array of in-app purchase transactions for the customer, signed by Apple, in JSON Web Signature format.
    ///
    /// [JWSTransaction](https://developer.apple.com/documentation/appstoreserverapi/jwstransaction)
    pub signed_transactions: Option<Vec<String>>,
}
