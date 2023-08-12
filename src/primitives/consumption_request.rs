use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::primitives::account_tenure::AccountTenure;
use crate::primitives::consumption_status::ConsumptionStatus;
use crate::primitives::delivery_status::DeliveryStatus;
use crate::primitives::lifetime_dollars_purchased::LifetimeDollarsPurchased;
use crate::primitives::lifetime_dollars_refunded::LifetimeDollarsRefunded;
use crate::primitives::platform::Platform;
use crate::primitives::play_time::PlayTime;
use crate::primitives::user_status::UserStatus;

/// The request body containing consumption information.
///
/// [ConsumptionRequest](https://developer.apple.com/documentation/appstoreserverapi/consumptionrequest)
#[derive(Debug, Deserialize, Serialize, Hash)]
pub struct ConsumptionRequest {
    /// A Boolean value that indicates whether the customer consented to provide consumption data to the App Store.
    ///
    /// [customerConsented](https://developer.apple.com/documentation/appstoreserverapi/customerconsented)
    #[serde(rename = "customerConsented")]
    pub customer_consented: Option<bool>,

    /// A value that indicates the extent to which the customer consumed the in-app purchase.
    ///
    /// [consumptionStatus](https://developer.apple.com/documentation/appstoreserverapi/consumptionstatus)
    #[serde(rename = "consumptionStatus")]
    pub consumption_status: Option<ConsumptionStatus>,

    /// A value that indicates the platform on which the customer consumed the in-app purchase.
    ///
    /// [platform](https://developer.apple.com/documentation/appstoreserverapi/platform)
    pub platform: Option<Platform>,

    /// A Boolean value that indicates whether you provided, prior to its purchase, a free sample or trial of the content, or information about its functionality.
    ///
    /// [sampleContentProvided](https://developer.apple.com/documentation/appstoreserverapi/samplecontentprovided)
    #[serde(rename = "sampleContentProvided")]
    pub sample_content_provided: Option<bool>,

    /// A value that indicates whether the app successfully delivered an in-app purchase that works properly.
    ///
    /// [deliveryStatus](https://developer.apple.com/documentation/appstoreserverapi/deliverystatus)
    #[serde(rename = "deliveryStatus")]
    pub delivery_status: Option<DeliveryStatus>,

    /// The UUID that an app optionally generates to map a customer’s in-app purchase with its resulting App Store transaction.
    ///
    /// [appAccountToken](https://developer.apple.com/documentation/appstoreserverapi/appaccounttoken)
    #[serde(rename = "appAccountToken")]
    pub app_account_token: Option<Uuid>,

    /// The age of the customer’s account.
    ///
    /// [accountTenure](https://developer.apple.com/documentation/appstoreserverapi/accounttenure)
    #[serde(rename = "accountTenure")]
    pub account_tenure: Option<AccountTenure>,

    /// A value that indicates the amount of time that the customer used the app.
    ///
    /// [playTime](https://developer.apple.com/documentation/appstoreserverapi/playtime)
    #[serde(rename = "playTime")]
    pub play_time: Option<PlayTime>,

    /// A value that indicates the total amount, in USD, of refunds the customer has received, in your app, across all platforms.
    ///
    /// [lifetimeDollarsRefunded](https://developer.apple.com/documentation/appstoreserverapi/lifetimedollarsrefunded)
    #[serde(rename = "lifetimeDollarsRefunded")]
    pub lifetime_dollars_refunded: Option<LifetimeDollarsRefunded>,

    /// A value that indicates the total amount, in USD, of in-app purchases the customer has made in your app, across all platforms.
    ///
    /// [lifetimeDollarsPurchased](https://developer.apple.com/documentation/appstoreserverapi/lifetimedollarspurchased)
    #[serde(rename = "lifetimeDollarsPurchased")]
    pub lifetime_dollars_purchased: Option<LifetimeDollarsPurchased>,

    /// The status of the customer’s account.
    ///
    /// [userStatus](https://developer.apple.com/documentation/appstoreserverapi/userstatus)
    #[serde(rename = "userStatus")]
    pub user_status: Option<UserStatus>,
}
