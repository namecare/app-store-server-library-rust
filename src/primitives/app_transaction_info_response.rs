// Copyright (c) 2025 Apple Inc. Licensed under MIT License.

use serde::{Deserialize, Serialize};

/// A response that contains signed app transaction information for a customer.
///
/// [AppTransactionInfoResponse](https://developer.apple.com/documentation/appstoreserverapi/apptransactioninforesponse)
#[derive(Debug, Clone, Deserialize, Serialize, Hash)]
pub struct AppTransactionInfoResponse {
    /// A customer's app transaction information, signed by Apple, in JSON Web Signature (JWS) format.
    ///
    /// [JWSAppTransaction](https://developer.apple.com/documentation/appstoreserverapi/jwsapptransaction)
    #[serde(rename = "signedAppTransactionInfo")]
    pub signed_app_transaction_info: Option<String>,
}