use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::formats::Flexible;
use serde_with::TimestampMilliSeconds;

/// A response that indicates the current status of a request to extend the subscription renewal date to all eligible subscribers.
///
/// [MassExtendRenewalDateStatusResponse](https://developer.apple.com/documentation/appstoreserverapi/massextendrenewaldatestatusresponse)
#[serde_with::serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct MassExtendRenewalDateStatusResponse {
    /// A string that contains a unique identifier you provide to track each subscription-renewal-date extension request.
    ///
    /// [requestIdentifier](https://developer.apple.com/documentation/appstoreserverapi/requestidentifier)
    #[serde(rename = "requestIdentifier")]
    pub request_identifier: Option<String>,

    /// A Boolean value that indicates whether the App Store completed the request to extend a subscription renewal date to active subscribers.
    ///
    /// [complete](https://developer.apple.com/documentation/appstoreserverapi/complete)
    pub complete: Option<bool>,

    /// The UNIX time, in milliseconds, that the App Store completes a request to extend a subscription renewal date for eligible subscribers.
    ///
    /// [completeDate](https://developer.apple.com/documentation/appstoreserverapi/completedate)
    #[serde(rename = "completeDate")]
    #[serde_as(as = "Option<TimestampMilliSeconds<String, Flexible>>")]
    pub complete_date: Option<DateTime<Utc>>,

    /// The count of subscriptions that successfully receive a subscription-renewal-date extension.
    ///
    /// [succeededCount](https://developer.apple.com/documentation/appstoreserverapi/succeededcount)
    #[serde(rename = "succeededCount")]
    pub succeeded_count: Option<i64>,

    /// The count of subscriptions that fail to receive a subscription-renewal-date extension.
    ///
    /// [failedCount](https://developer.apple.com/documentation/appstoreserverapi/failedcount)
    #[serde(rename = "failedCount")]
    pub failed_count: Option<i64>,
}
