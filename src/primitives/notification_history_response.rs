use serde::{Deserialize, Serialize};
use crate::primitives::notification_history_response_item::NotificationHistoryResponseItem;

/// A response that contains the App Store Server Notifications history for your app.
///
/// [NotificationHistoryResponse](https://developer.apple.com/documentation/appstoreserverapi/notificationhistoryresponse)
#[derive(Debug, Deserialize, Serialize, Hash)]
pub struct NotificationHistoryResponse {
    /// A pagination token that you return to the endpoint on a subsequent call to receive the next set of results.
    ///
    /// [paginationToken](https://developer.apple.com/documentation/appstoreserverapi/paginationtoken)
    #[serde(rename = "paginationToken")]
    pub pagination_token: Option<String>,

    /// A Boolean value indicating whether the App Store has more transaction data.
    ///
    /// [hasMore](https://developer.apple.com/documentation/appstoreserverapi/hasmore)
    #[serde(rename = "hasMore")]
    pub has_more: Option<bool>,

    /// An array of App Store server notification history records.
    #[serde(rename = "notificationHistory")]
    pub notification_history: Option<Vec<NotificationHistoryResponseItem>>,
}
