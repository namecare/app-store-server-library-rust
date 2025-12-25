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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_consented: Option<bool>,

    /// An integer that indicates the percentage, in milliunits, of the In-App Purchase the customer consumed.
    ///
    /// [consumptionPercentage](https://developer.apple.com/documentation/appstoreserverapi/consumptionpercentage)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_percentage: Option<u32>,

    /// A value that indicates whether the app successfully delivered an in-app purchase that works properly.
    ///
    /// [deliveryStatus](https://developer.apple.com/documentation/appstoreserverapi/deliverystatus)
    pub delivery_status: Option<DeliveryStatus>,

    /// A value that indicates your preference, based on your operational logic, as to whether Apple should grant the refund.
    ///
    /// [refundPreference](https://developer.apple.com/documentation/appstoreserverapi/refundpreference)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_preference: Option<RefundPreference>,

    /// A Boolean value that indicates whether you provided, prior to its purchase, a free sample or trial of the content, or information about its functionality.
    ///
    /// [sampleContentProvided](https://developer.apple.com/documentation/appstoreserverapi/samplecontentprovided)
    pub sample_content_provided: bool,
}
