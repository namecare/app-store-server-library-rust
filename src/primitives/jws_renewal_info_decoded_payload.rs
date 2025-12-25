use crate::primitives::advanced_commerce_price_increase_info::AdvancedCommercePriceIncreaseInfo;
use crate::primitives::auto_renew_status::AutoRenewStatus;
use crate::primitives::environment::Environment;
use crate::primitives::expiration_intent::ExpirationIntent;
use crate::primitives::offer_discount_type::OfferDiscountType;
use crate::primitives::offer_type::OfferType;
use crate::primitives::price_increase_status::PriceIncreaseStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::formats::Flexible;
use serde_with::TimestampMilliSeconds;
use uuid::Uuid;
use crate::primitives::advanced_commerce_renewal_info::AdvancedCommerceRenewalInfo;

/// A decoded payload containing subscription renewal information for an auto-renewable subscription.
///
/// [JWSRenewalInfoDecodedPayload](https://developer.apple.com/documentation/appstoreserverapi/jwsrenewalinfodecodedpayload)
#[serde_with::serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, Hash)]
#[serde(rename_all = "camelCase")]
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
    #[serde_as(as = "Option<TimestampMilliSeconds<String, Flexible>>")]
    pub grace_period_expires_date: Option<DateTime<Utc>>,

    /// The type of subscription offer.
    ///
    /// [offerType](https://developer.apple.com/documentation/appstoreserverapi/offertype)
    pub offer_type: Option<OfferType>,

    /// The offer code or the promotional offer identifier.
    ///
    /// [offerIdentifier](https://developer.apple.com/documentation/appstoreserverapi/offeridentifier)
    pub offer_identifier: Option<String>,

    /// The UNIX time, in milliseconds, that the App Store signed the JSON Web Signature data.
    ///
    /// [signedDate](https://developer.apple.com/documentation/appstoreserverapi/signeddate)
    #[serde_as(as = "Option<TimestampMilliSeconds<String, Flexible>>")]
    pub signed_date: Option<DateTime<Utc>>,

    /// The server environment, either sandbox or production.
    ///
    /// [environment](https://developer.apple.com/documentation/appstoreserverapi/environment)
    pub environment: Option<Environment>,

    /// The earliest start date of a subscription in a series of auto-renewable subscription purchases that ignores all lapses of paid service shorter than 60 days.
    ///
    /// [recentSubscriptionStartDate](https://developer.apple.com/documentation/appstoreserverapi/recentsubscriptionstartdate)
    #[serde_as(as = "Option<TimestampMilliSeconds<String, Flexible>>")]
    pub recent_subscription_start_date: Option<DateTime<Utc>>,

    /// The UNIX time, in milliseconds, when the most recent auto-renewable subscription purchase expires.
    ///
    /// [renewalDate](https://developer.apple.com/documentation/appstoreserverapi/renewaldate)
    #[serde_as(as = "Option<TimestampMilliSeconds<String, Flexible>>")]
    pub renewal_date: Option<DateTime<Utc>>,

    ///The currency code for the renewalPrice of the subscription.
    ///
    ///[currency](https://developer.apple.com/documentation/appstoreserverapi/currency)
    pub currency: Option<String>,

    ///The renewal price, in milliunits, of the auto-renewable subscription that renews at the next billing period.
    ///
    ///[renewalPrice](https://developer.apple.com/documentation/appstoreserverapi/renewalprice)
    pub renewal_price: Option<i64>,

    ///The payment mode you configure for the offer.
    ///
    ///[offerDiscountType](https://developer.apple.com/documentation/appstoreserverapi/offerdiscounttype)
    pub offer_discount_type: Option<OfferDiscountType>,

    ///An array of win-back offer identifiers that a customer is eligible to redeem, which sorts the identifiers to present the better offers first.
    ///
    ///[eligibleWinBackOfferIds](https://developer.apple.com/documentation/appstoreserverapi/eligiblewinbackofferids)
    pub eligible_win_back_offer_ids: Option<Vec<String>>,

    /// The UUID that an app optionally generates to map a customer’s in-app purchase with its resulting App Store transaction.
    ///
    /// [appAccountToken](https://developer.apple.com/documentation/appstoreserverapi/appAccountToken)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_account_token: Option<Uuid>,

    /// The unique identifier of the app download transaction.
    ///
    /// [appTransactionId](https://developer.apple.com/documentation/appstoreserverapi/appTransactionId)
    pub app_transaction_id: Option<String>,

    /// The duration of the offer.
    ///
    /// [offerPeriod](https://developer.apple.com/documentation/appstoreserverapi/offerPeriod)
    pub offer_period: Option<String>,

    /// Renewal information that is present only for Advanced Commerce SKUs.
    ///
    /// [advancedCommerceRenewalInfo](https://developer.apple.com/documentation/appstoreserverapi/advancedcommercerenewalinfo)
    pub advanced_commerce_info: Option<AdvancedCommerceRenewalInfo>,

    /// Information about the Advanced Commerce price increase for the subscription.
    ///
    /// [advancedCommercePriceIncreaseInfo](https://developer.apple.com/documentation/appstoreserverapi/advancedcommercepriceincrease)
    pub advanced_commerce_price_increase_info: Option<AdvancedCommercePriceIncreaseInfo>,
}
