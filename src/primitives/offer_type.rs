use serde::{Deserialize, Serialize};

/// The type of subscription offer.
///
/// [offerType](https://developer.apple.com/documentation/appstoreserverapi/offertype)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum OfferType {
    IntroductoryOffer = 1,
    PromotionalOffer = 2,
    SubscriptionOfferCode = 3,
}
