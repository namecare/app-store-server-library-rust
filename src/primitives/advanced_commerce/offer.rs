use crate::primitives::advanced_commerce::offer_period::OfferPeriod;
use crate::primitives::advanced_commerce::offer_reason::OfferReason;
use serde::{Deserialize, Serialize};

/// A discount offer for an auto-renewable subscription.
///
/// [Offer](https://developer.apple.com/documentation/advancedcommerceapi/offer)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Offer {
    /// The period of the offer.
    ///
    /// [Period](https://developer.apple.com/documentation/advancedcommerceapi/period)
    pub period: OfferPeriod,
    
    /// The number of periods the offer is active.
    /// Minimum: 1, Maximum: 12
    pub period_count: i32,
    
    /// The offer price, in milliunits.
    ///
    /// [Price](https://developer.apple.com/documentation/advancedcommerceapi/price)
    pub price: i64,
    
    /// The reason for the offer.
    ///
    /// [Reason](https://developer.apple.com/documentation/advancedcommerceapi/reason)
    pub reason: OfferReason,
}

impl Offer {
    pub fn new(period: OfferPeriod, period_count: i32, price: i64, reason: OfferReason) -> Self {
        Self {
            period,
            period_count,
            price,
            reason,
        }
    }
}