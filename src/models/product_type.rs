use serde::{Deserialize, Serialize};

/// The type of in-app purchase products you can offer in your app.
///
/// [ProductType](https://developer.apple.com/documentation/appstoreserverapi/type)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum ProductType {
    #[serde(rename = "Auto-Renewable Subscription")]
    AutoRenewableSubscription,
    #[serde(rename = "Non-Consumable")]
    NonConsumable,
    #[serde(rename = "Consumable")]
    Consumable,
    #[serde(rename = "Non-Renewing Subscription")]
    NonRenewingSubscription,
}
