use serde::{Deserialize, Serialize};

/// The payment mode you configure for an introductory offer, promotional offer, or offer code on an auto-renewable subscription.
///
/// [offerDiscountType](https://developer.apple.com/documentation/appstoreserverapi/offerdiscounttype)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OfferDiscountType {
    #[serde(rename = "FREE_TRIAL")]
    FreeTrial,
    #[serde(rename = "PAY_AS_YOU_GO")]
    PayAsYouGo,
    #[serde(rename = "PAY_UP_FRONT")]
    PayUpFront,
}