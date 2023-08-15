use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::primitives::auto_renew_status::AutoRenewStatus;
use crate::primitives::environment::Environment;
use crate::primitives::expiration_intent::ExpirationIntent;
use crate::primitives::offer_type::OfferType;
use crate::primitives::price_increase_status::PriceIncreaseStatus;

/// A decoded payload containing subscription renewal information for an auto-renewable subscription.
///
/// [JWSRenewalInfoDecodedPayload](https://developer.apple.com/documentation/appstoreserverapi/jwsrenewalinfodecodedpayload)
#[derive(Debug, Clone, Deserialize, Serialize, Hash)]
pub struct JWSRenewalInfoDecodedPayload {
    /// The reason the subscription expired.
    ///
    /// [expirationIntent](https://developer.apple.com/documentation/appstoreserverapi/expirationintent)
    pub expiration_intent: Option<ExpirationIntent>,

    /// The original transaction identifier of a purchase.
    ///
    /// [originalTransactionId](https://developer.apple.com/documentation/appstoreserverapi/originaltransactionid)
    pub original_transaction_id: Option<String>,

    /// The product identifier of the product that will renew at the next billing period.
    ///
    /// [autoRenewProductId](https://developer.apple.com/documentation/appstoreserverapi/autorenewproductid)
    pub auto_renew_product_id: Option<String>,

    /// The unique identifier for the product, that you create in App Store Connect.
    ///
    /// [productId](https://developer.apple.com/documentation/appstoreserverapi/productid)
    pub product_id: Option<String>,

    /// The renewal status of the auto-renewable subscription.
    ///
    /// [autoRenewStatus](https://developer.apple.com/documentation/appstoreserverapi/autorenewstatus)
    pub auto_renew_status: Option<AutoRenewStatus>,

    /// A Boolean value that indicates whether the App Store is attempting to automatically renew an expired subscription.
    ///
    /// [isInBillingRetryPeriod](https://developer.apple.com/documentation/appstoreserverapi/isinbillingretryperiod)
    pub is_in_billing_retry_period: Option<bool>,

    /// The status that indicates whether the auto-renewable subscription is subject to a price increase.
    ///
    /// [priceIncreaseStatus](https://developer.apple.com/documentation/appstoreserverapi/priceincreasestatus)
    pub price_increase_status: Option<PriceIncreaseStatus>,

    /// The time when the billing grace period for subscription renewals expires.
    ///
    /// [gracePeriodExpiresDate](https://developer.apple.com/documentation/appstoreserverapi/graceperiodexpiresdate)
    pub grace_period_expires_date: Option<DateTime<Utc>>,

    /// The type of the subscription offer.
    ///
    /// [offerType](https://developer.apple.com/documentation/appstoreserverapi/offertype)
    pub offer_type: Option<OfferType>,

    /// The identifier that contains the promo code or the promotional offer identifier.
    ///
    /// [offerIdentifier](https://developer.apple.com/documentation/appstoreserverapi/offeridentifier)
    pub offer_identifier: Option<String>,

    /// The UNIX time, in milliseconds, that the App Store signed the JSON Web Signature data.
    ///
    /// [signedDate](https://developer.apple.com/documentation/appstoreserverapi/signeddate)
    pub signed_date: Option<DateTime<Utc>>,

    /// The server environment, either sandbox or production.
    ///
    /// [environment](https://developer.apple.com/documentation/appstoreserverapi/environment)
    pub environment: Option<Environment>,

    /// The earliest start date of a subscription in a series of auto-renewable subscription purchases that ignores all lapses of paid service shorter than 60 days.
    ///
    /// [recentSubscriptionStartDate](https://developer.apple.com/documentation/appstoreserverapi/recentsubscriptionstartdate)
    pub recent_subscription_start_date: Option<DateTime<Utc>>,

    /// The UNIX time, in milliseconds, when the most recent auto-renewable subscription purchase expires.
    ///
    /// [renewalDate](https://developer.apple.com/documentation/appstoreserverapi/renewaldate)
    pub renewal_date: Option<DateTime<Utc>>,
}