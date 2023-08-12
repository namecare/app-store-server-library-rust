use serde::{Deserialize, Serialize};

/// A response that indicates the current status of a request to extend the subscription renewal date to all eligible subscribers.
///
/// [MassExtendRenewalDateStatusResponse](https://developer.apple.com/documentation/appstoreserverapi/massextendrenewaldatestatusresponse)
#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct MassExtendRenewalDateStatusResponse {
    /// A string that contains a unique identifier you provide to track each subscription-renewal-date extension request.
    ///
    /// [requestIdentifier](https://developer.apple.com/documentation/appstoreserverapi/requestidentifier)
    #[serde(rename = "requestIdentifier")]
    pub request_identifier: String,

    /// A Boolean value that indicates whether the App Store completed the request to extend a subscription renewal date to active subscribers.
    ///
    /// [complete](https://developer.apple.com/documentation/appstoreserverapi/complete)
    pub complete: bool,

    /// The UNIX time, in milliseconds, that the App Store completes a request to extend a subscription renewal date for eligible subscribers.
    ///
    /// [completeDate](https://developer.apple.com/documentation/appstoreserverapi/completedate)
    #[serde(rename = "completeDate")]
    pub complete_date: chrono::NaiveDateTime,

    /// The count of subscriptions that successfully receive a subscription-renewal-date extension.
    ///
    /// [succeededCount](https://developer.apple.com/documentation/appstoreserverapi/succeededcount)
    #[serde(rename = "succeededCount")]
    pub succeeded_count: i64,

    /// The count of subscriptions that fail to receive a subscription-renewal-date extension.
    ///
    /// [failedCount](https://developer.apple.com/documentation/appstoreserverapi/failedcount)
    #[serde(rename = "failedCount")]
    pub failed_count: i64,
}
