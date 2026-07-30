use crate::models::delivery_status::DeliveryStatus;
use crate::models::refund_preference::RefundPreference;
use serde::{Deserialize, Serialize};

/// The request body containing consumption information.
///
/// [ConsumptionRequest](https://developer.apple.com/documentation/appstoreserverapi/consumptionrequest)
#[derive(Debug, Clone, Deserialize, Serialize, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ConsumptionRequest {
    /// A Boolean value that indicates whether the customer consented to provide consumption data to the App Store.
    ///
    /// [customerConsented](https://developer.apple.com/documentation/appstoreserverapi/customerconsented)
    pub customer_consented: bool,

    /// An integer that indicates the percentage, in milliunits, of the In-App Purchase the customer consumed.
    ///
    /// [consumptionPercentage](https://developer.apple.com/documentation/appstoreserverapi/consumptionpercentage)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_percentage: Option<u32>,

    /// A value that indicates whether the app successfully delivered an in-app purchase that works properly.
    ///
    /// [deliveryStatus](https://developer.apple.com/documentation/appstoreserverapi/deliverystatus)
    pub delivery_status: DeliveryStatus,

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
