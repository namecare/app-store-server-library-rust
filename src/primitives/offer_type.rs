use serde_repr::{Serialize_repr, Deserialize_repr};

/// The type of subscription offer.
///
/// [offerType](https://developer.apple.com/documentation/appstoreserverapi/offertype)
#[derive(Debug, Clone, Deserialize_repr, Serialize_repr, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum OfferType {
    IntroductoryOffer = 1,
    PromotionalOffer = 2,
    SubscriptionOfferCode = 3,
}
