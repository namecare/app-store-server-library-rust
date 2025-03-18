use crate::primitives::environment::Environment;
use crate::primitives::in_app_ownership_type::InAppOwnershipType;
use crate::primitives::offer_discount_type::OfferDiscountType;
use crate::primitives::offer_type::OfferType;
use crate::primitives::product_type::ProductType;
use crate::primitives::revocation_reason::RevocationReason;
use crate::primitives::transaction_reason::TransactionReason;
use chrono::{DateTime, Utc};
use serde_with::formats::Flexible;
use serde_with::TimestampMilliSeconds;
use uuid::Uuid;

/// A decoded payload containing transaction information.
///
/// [JWSTransactionDecodedPayload](https://developer.apple.com/documentation/appstoreserverapi/jwstransactiondecodedpayload)
#[serde_with::serde_as]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Hash)]
pub struct JWSTransactionDecodedPayload {
    /// The original transaction identifier of a purchase.
    ///
    /// [originalTransactionId](https://developer.apple.com/documentation/appstoreserverapi/originaltransactionid)
    #[serde(rename = "originalTransactionId")]
    pub original_transaction_id: Option<String>,

    /// The unique identifier for a transaction such as an in-app purchase, restored in-app purchase, or subscription renewal.
    ///
    /// [transactionId](https://developer.apple.com/documentation/appstoreserverapi/transactionid)
    #[serde(rename = "transactionId")]
    pub transaction_id: Option<String>,

    /// The unique identifier of subscription-purchase events across devices, including renewals.
    ///
    /// [webOrderLineItemId](https://developer.apple.com/documentation/appstoreserverapi/weborderlineitemid)
    #[serde(rename = "webOrderLineItemId")]
    pub web_order_line_item_id: Option<String>,

    /// The bundle identifier of an app.
    ///
    /// [bundle_id](https://developer.apple.com/documentation/appstoreserverapi/bundleid)
    #[serde(rename = "bundleId")]
    pub bundle_id: Option<String>,

    /// The unique identifier for the product, that you create in App Store Connect.
    ///
    /// [productId](https://developer.apple.com/documentation/appstoreserverapi/productid)
    #[serde(rename = "productId")]
    pub product_id: Option<String>,

    /// The identifier of the subscription group that the subscription belongs to.
    ///
    /// [subscriptionGroupIdentifier](https://developer.apple.com/documentation/appstoreserverapi/subscriptiongroupidentifier)
    #[serde(rename = "subscriptionGroupIdentifier")]
    pub subscription_group_identifier: Option<String>,

    /// The time that the App Store charged the user’s account for an in-app purchase, a restored in-app purchase, a subscription, or a subscription renewal after a lapse.
    ///
    /// [purchaseDate](https://developer.apple.com/documentation/appstoreserverapi/purchasedate)
    #[serde(rename = "purchaseDate")]
    #[serde_as(as = "Option<TimestampMilliSeconds<String, Flexible>>")]
    pub purchase_date: Option<DateTime<Utc>>,

    /// The purchase date of the transaction associated with the original transaction identifier.
    ///
    /// [originalPurchaseDate](https://developer.apple.com/documentation/appstoreserverapi/originalpurchasedate)
    #[serde(rename = "originalPurchaseDate")]
    #[serde_as(as = "Option<TimestampMilliSeconds<String, Flexible>>")]
    pub original_purchase_date: Option<DateTime<Utc>>,

    /// The UNIX time, in milliseconds, an auto-renewable subscription expires or renews.
    ///
    /// [expiresDate](https://developer.apple.com/documentation/appstoreserverapi/expiresdate)
    #[serde(rename = "expiresDate")]
    #[serde_as(as = "Option<TimestampMilliSeconds<String, Flexible>>")]
    pub expires_date: Option<DateTime<Utc>>,

    /// The number of consumable products purchased.
    ///
    /// [quantity](https://developer.apple.com/documentation/appstoreserverapi/quantity)
    #[serde(rename = "quantity")]
    pub quantity: Option<i32>,

    /// The type of the in-app purchase.
    ///
    /// [type](https://developer.apple.com/documentation/appstoreserverapi/type)
    #[serde(rename = "type")]
    pub r#type: Option<ProductType>,

    /// The UUID that an app optionally generates to map a customer’s in-app purchase with its resulting App Store transaction.
    ///
    /// [appAccountToken](https://developer.apple.com/documentation/appstoreserverapi/appaccounttoken)
    #[serde(rename = "appAccountToken")]
    pub app_account_token: Option<Uuid>,

    /// A string that describes whether the transaction was purchased by the user, or is available to them through Family Sharing.
    ///
    /// [inAppOwnershipType](https://developer.apple.com/documentation/appstoreserverapi/inappownershiptype)
    #[serde(rename = "inAppOwnershipType")]
    pub in_app_ownership_type: Option<InAppOwnershipType>,

    /// The UNIX time, in milliseconds, that the App Store signed the JSON Web Signature data.
    ///
    /// [signedDate](https://developer.apple.com/documentation/appstoreserverapi/signeddate)
    #[serde(rename = "signedDate")]
    #[serde_as(as = "Option<TimestampMilliSeconds<String, Flexible>>")]
    pub signed_date: Option<DateTime<Utc>>,

    /// The reason that the App Store refunded the transaction or revoked it from family sharing.
    ///
    /// [revocationReason](https://developer.apple.com/documentation/appstoreserverapi/revocationreason)
    #[serde(rename = "revocationReason")]
    pub revocation_reason: Option<RevocationReason>,

    /// The UNIX time, in milliseconds, that Apple Support refunded a transaction.
    ///
    /// [revocationDate](https://developer.apple.com/documentation/appstoreserverapi/revocationdate)
    #[serde(rename = "revocationDate")]
    #[serde_as(as = "Option<TimestampMilliSeconds<String, Flexible>>")]
    pub revocation_date: Option<DateTime<Utc>>,

    /// The Boolean value that indicates whether the user upgraded to another subscription.
    ///
    /// [isUpgraded](https://developer.apple.com/documentation/appstoreserverapi/isupgraded)
    #[serde(rename = "isUpgraded")]
    pub is_upgraded: Option<bool>,

    /// A value that represents the promotional offer type.
    ///
    /// [offerType](https://developer.apple.com/documentation/appstoreserverapi/offertype)
    #[serde(rename = "offerType")]
    pub offer_type: Option<OfferType>,

    /// The identifier that contains the promo code or the promotional offer identifier.
    ///
    /// [offerIdentifier](https://developer.apple.com/documentation/appstoreserverapi/offeridentifier)
    #[serde(rename = "offerIdentifier")]
    pub offer_identifier: Option<String>,

    /// The server environment, either sandbox or production.
    ///
    /// [environment](https://developer.apple.com/documentation/appstoreserverapi/environment)
    pub environment: Option<Environment>,

    /// The three-letter code that represents the country or region associated with the App Store storefront for the purchase.
    ///
    /// [storefront](https://developer.apple.com/documentation/appstoreserverapi/storefront)
    pub storefront: Option<String>,

    /// An Apple-defined value that uniquely identifies the App Store storefront associated with the purchase.
    ///
    /// [storefrontId](https://developer.apple.com/documentation/appstoreserverapi/storefrontid)
    #[serde(rename = "storefrontId")]
    pub storefront_id: Option<String>,

    /// The reason for the purchase transaction, which indicates whether it’s a customer’s purchase or a renewal for an auto-renewable subscription that the system initiates.
    ///
    /// [transactionReason](https://developer.apple.com/documentation/appstoreserverapi/transactionreason)
    #[serde(rename = "transactionReason")]
    pub transaction_reason: Option<TransactionReason>,

    /// The three-letter ISO 4217 currency code for the price of the product.
    ///
    /// [currency](https://developer.apple.com/documentation/appstoreserverapi/currency)
    pub currency: Option<String>,

    /// The price, in milliunits, of the in-app purchase or subscription offer that you configured in App Store Connect.
    ///
    /// [price](https://developer.apple.com/documentation/appstoreserverapi/price)
    pub price: Option<i64>,

    /// The payment mode you configure for an introductory offer, promotional offer, or offer code on an auto-renewable subscription.
    ///
    /// [offerDiscountType](https://developer.apple.com/documentation/appstoreserverapi/offerdiscounttype)
    #[serde(rename = "offerDiscountType")]
    pub offer_discount_type: Option<OfferDiscountType>,

    /// The unique identifier of the app download transaction.
    ///
    /// [appTransactionId](https://developer.apple.com/documentation/appstoreserverapi/appTransactionId)
    #[serde(rename = "appTransactionId")]
    pub app_transaction_id: Option<String>,

    /// The duration of the offer.
    ///
    /// [offerPeriod](https://developer.apple.com/documentation/appstoreserverapi/offerPeriod)
    #[serde(rename = "offerPeriod")]
    pub offer_period: Option<String>,
}
