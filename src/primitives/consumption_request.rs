use crate::primitives::serde_ext::{de_string_as_optional_uuid, ser_optional_uuid_as_string};
use crate::primitives::account_tenure::AccountTenure;
use crate::primitives::consumption_status::ConsumptionStatus;
use crate::primitives::delivery_status::DeliveryStatus;
use crate::primitives::lifetime_dollars_purchased::LifetimeDollarsPurchased;
use crate::primitives::lifetime_dollars_refunded::LifetimeDollarsRefunded;
use crate::primitives::platform::Platform;
use crate::primitives::play_time::PlayTime;
use crate::primitives::refund_preference::RefundPreference;
use crate::primitives::user_status::UserStatus;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The request body containing consumption information.
///
/// [ConsumptionRequest](https://developer.apple.com/documentation/appstoreserverapi/consumptionrequest)
#[derive(Debug, Clone, Deserialize, Serialize, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ConsumptionRequest {
    /// A Boolean value that indicates whether the customer consented to provide consumption data to the App Store.
    ///
    /// [customerConsented](https://developer.apple.com/documentation/appstoreserverapi/customerconsented)
    pub customer_consented: Option<bool>,

    /// A value that indicates the extent to which the customer consumed the in-app purchase.
    ///
    /// [consumptionStatus](https://developer.apple.com/documentation/appstoreserverapi/consumptionstatus)
    pub consumption_status: Option<ConsumptionStatus>,

    /// A value that indicates the platform on which the customer consumed the in-app purchase.
    ///
    /// [platform](https://developer.apple.com/documentation/appstoreserverapi/platform)
    pub platform: Option<Platform>,

    /// A Boolean value that indicates whether you provided, prior to its purchase, a free sample or trial of the content, or information about its functionality.
    ///
    /// [sampleContentProvided](https://developer.apple.com/documentation/appstoreserverapi/samplecontentprovided)
    pub sample_content_provided: Option<bool>,

    /// A value that indicates whether the app successfully delivered an in-app purchase that works properly.
    ///
    /// [deliveryStatus](https://developer.apple.com/documentation/appstoreserverapi/deliverystatus)
    pub delivery_status: Option<DeliveryStatus>,

    /// The UUID that an app optionally generates to map a customer's in-app purchase with its resulting App Store transaction.
    ///
    /// [appAccountToken](https://developer.apple.com/documentation/appstoreserverapi/appaccounttoken)
    #[serde(
        deserialize_with = "de_string_as_optional_uuid",
        serialize_with = "ser_optional_uuid_as_string"
    )]
    pub app_account_token: Option<Uuid>,

    /// The age of the customer’s account.
    ///
    /// [accountTenure](https://developer.apple.com/documentation/appstoreserverapi/accounttenure)
    pub account_tenure: Option<AccountTenure>,

    /// A value that indicates the amount of time that the customer used the app.
    ///
    /// [playTime](https://developer.apple.com/documentation/appstoreserverapi/playtime)
    pub play_time: Option<PlayTime>,

    /// A value that indicates the total amount, in USD, of refunds the customer has received, in your app, across all platforms.
    ///
    /// [lifetimeDollarsRefunded](https://developer.apple.com/documentation/appstoreserverapi/lifetimedollarsrefunded)
    pub lifetime_dollars_refunded: Option<LifetimeDollarsRefunded>,

    /// A value that indicates the total amount, in USD, of in-app purchases the customer has made in your app, across all platforms.
    ///
    /// [lifetimeDollarsPurchased](https://developer.apple.com/documentation/appstoreserverapi/lifetimedollarspurchased)
    pub lifetime_dollars_purchased: Option<LifetimeDollarsPurchased>,

    /// The status of the customer’s account.
    ///
    /// [userStatus](https://developer.apple.com/documentation/appstoreserverapi/userstatus)
    pub user_status: Option<UserStatus>,

    /// A value that indicates your preference, based on your operational logic, as to whether Apple should grant the refund.
    ///
    /// [refundPreference](https://developer.apple.com/documentation/appstoreserverapi/refundpreference)
    pub refund_preference: Option<RefundPreference>,
}
