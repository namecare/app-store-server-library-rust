use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::formats::Flexible;
use serde_with::TimestampMilliSeconds;

/// A response that indicates whether an individual renewal-date extension succeeded, and related details.
///
/// [ExtendRenewalDateResponse](https://developer.apple.com/documentation/appstoreserverapi/extendrenewaldateresponse)
#[serde_with::serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, Hash)]
pub struct ExtendRenewalDateResponse {
    /// The original transaction identifier of a purchase.
    ///
    /// [originalTransactionId](https://developer.apple.com/documentation/appstoreserverapi/originaltransactionid)
    #[serde(rename = "originalTransactionId")]
    pub original_transaction_id: Option<String>,

    /// The unique identifier of subscription-purchase events across devices, including renewals.
    ///
    /// [webOrderLineItemId](https://developer.apple.com/documentation/appstoreserverapi/weborderlineitemid)
    #[serde(rename = "webOrderLineItemId")]
    pub web_order_line_item_id: Option<String>,

    /// A Boolean value that indicates whether the subscription-renewal-date extension succeeded.
    ///
    /// [success](https://developer.apple.com/documentation/appstoreserverapi/success)
    #[serde(rename = "success")]
    pub success: Option<bool>,

    /// The new subscription expiration date for a subscription-renewal extension.
    ///
    /// [effectiveDate](https://developer.apple.com/documentation/appstoreserverapi/effectivedate)
    #[serde(rename = "effectiveDate")]
    #[serde_as(as = "Option<TimestampMilliSeconds<String, Flexible>>")]
    pub effective_date: Option<DateTime<Utc>>,
}
