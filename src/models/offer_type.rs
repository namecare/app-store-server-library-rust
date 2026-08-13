use serde::{Deserialize, Serialize};

/// The type of offer.
///
/// [offerType](https://developer.apple.com/documentation/appstoreserverapi/offertype)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(from = "i64", into = "i64")]
pub enum OfferType {
    IntroductoryOffer,
    PromotionalOffer,
    OfferCode,
    WinBackOffer,

    /// A value the App Store sent that this version of the
    /// library does not support, preserved as received.
    NotSupported(i64),
}

impl From<i64> for OfferType {
    fn from(value: i64) -> Self {
        match value {
            1 => OfferType::IntroductoryOffer,
            2 => OfferType::PromotionalOffer,
            3 => OfferType::OfferCode,
            4 => OfferType::WinBackOffer,
            other => OfferType::NotSupported(other),
        }
    }
}

impl From<OfferType> for i64 {
    fn from(value: OfferType) -> Self {
        match value {
            OfferType::IntroductoryOffer => 1,
            OfferType::PromotionalOffer => 2,
            OfferType::OfferCode => 3,
            OfferType::WinBackOffer => 4,
            OfferType::NotSupported(other) => other,
        }
    }
}
