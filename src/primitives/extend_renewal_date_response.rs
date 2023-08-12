use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// A response that indicates whether an individual renewal-date extension succeeded, and related details.
///
/// [ExtendRenewalDateResponse](https://developer.apple.com/documentation/appstoreserverapi/extendrenewaldateresponse)
#[derive(Debug, Deserialize, Serialize, Hash)]
pub struct ExtendRenewalDateResponse {
    /// The original transaction identifier of a purchase.
    ///
    /// [originalTransactionId](https://developer.apple.com/documentation/appstoreserverapi/originaltransactionid)
    pub original_transaction_id: Option<String>,

    /// The unique identifier of subscription-purchase events across devices, including renewals.
    ///
    /// [webOrderLineItemId](https://developer.apple.com/documentation/appstoreserverapi/weborderlineitemid)
    pub web_order_line_item_id: Option<String>,

    /// A Boolean value that indicates whether the subscription-renewal-date extension succeeded.
    ///
    /// [success](https://developer.apple.com/documentation/appstoreserverapi/success)
    pub success: Option<bool>,

    /// The new subscription expiration date for a subscription-renewal extension.
    ///
    /// [effectiveDate](https://developer.apple.com/documentation/appstoreserverapi/effectivedate)
    pub effective_date: Option<DateTime<Utc>>,
}
