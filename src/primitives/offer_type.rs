use serde_repr::{Deserialize_repr, Serialize_repr};

/// The type of offer.
///
/// [offerType](https://developer.apple.com/documentation/appstoreserverapi/offertype)
#[derive(Debug, Clone, Deserialize_repr, Serialize_repr, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum OfferType {
    IntroductoryOffer = 1,
    PromotionalOffer = 2,
    OfferCode = 3,
    WinBackOffer = 4,
}
