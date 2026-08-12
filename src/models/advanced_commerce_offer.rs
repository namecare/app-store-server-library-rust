use serde::{Deserialize, Serialize};

use crate::models::advanced_commerce_offer_period::AdvancedCommerceOfferPeriod;
use crate::models::advanced_commerce_offer_reason::AdvancedCommerceOfferReason;

/// A discount offer for an auto-renewable subscription.
///
/// [AdvancedCommerceOffer](https://developer.apple.com/documentation/advancedcommerceapi/offer)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedCommerceOffer {
    /// The period of the offer.
    ///
    /// [AdvancedCommercePeriod](https://developer.apple.com/documentation/advancedcommerceapi/period)
    pub period: AdvancedCommerceOfferPeriod,

    /// The number of periods the offer is active.
    /// Minimum: 1, Maximum: 12
    pub period_count: i32,

    /// The offer price, in milliunits.
    ///
    /// [Price](https://developer.apple.com/documentation/advancedcommerceapi/price)
    pub price: i64,

    /// The reason for the offer.
    ///
    /// [AdvancedCommerceReason](https://developer.apple.com/documentation/advancedcommerceapi/reason)
    pub reason: AdvancedCommerceOfferReason,
}

impl AdvancedCommerceOffer {
    pub fn new(
        period: AdvancedCommerceOfferPeriod,
        period_count: i32,
        price: i64,
        reason: AdvancedCommerceOfferReason,
    ) -> Self {
        Self {
            period,
            period_count,
            price,
            reason,
        }
    }
}
