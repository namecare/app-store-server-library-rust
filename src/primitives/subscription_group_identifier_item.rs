use crate::primitives::last_transactions_item::LastTransactionsItem;
use serde::{Deserialize, Serialize};

/// Information for auto-renewable subscriptions, including signed transaction information and signed renewal information, for one subscription group.
///
/// [SubscriptionGroupIdentifierItem](https://developer.apple.com/documentation/appstoreserverapi/subscriptiongroupidentifieritem)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq)]
pub struct SubscriptionGroupIdentifierItem {
    /// The identifier of the subscription group that the subscription belongs to.
    ///
    /// [subscriptionGroupIdentifier](https://developer.apple.com/documentation/appstoreserverapi/subscriptiongroupidentifier)
    #[serde(rename = "subscriptionGroupIdentifier")]
    pub subscription_group_identifier: Option<String>,

    /// An array of the most recent App Store-signed transaction information and App Store-signed renewal information for all auto-renewable subscriptions in the subscription group.
    #[serde(rename = "lastTransactions")]
    pub last_transactions: Option<Vec<LastTransactionsItem>>,
}
