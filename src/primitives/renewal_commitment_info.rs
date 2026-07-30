use crate::primitives::auto_renew_status::AutoRenewStatus;
use crate::primitives::renewal_billing_plan_type::RenewalBillingPlanType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::formats::Flexible;
use serde_with::TimestampMilliSeconds;

/// Information about a subscription renewal commitment.
///
/// [RenewalCommitmentInfo](https://developer.apple.com/documentation/appstoreserverapi/renewalcommitmentinfo)
#[serde_with::serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, Hash)]
#[serde(rename_all = "camelCase")]
pub struct RenewalCommitmentInfo {
    /// The product identifier of the subscription to which the user commits.
    ///
    /// [commitmentAutoRenewProductId](https://developer.apple.com/documentation/appstoreserverapi/commitmentautorenewproductid)
    pub commitment_auto_renew_product_id: Option<String>,

    /// The renewal status of the subscription.
    ///
    /// [commitmentAutoRenewStatus](https://developer.apple.com/documentation/appstoreserverapi/commitmentautorenewstatus)
    pub commitment_auto_renew_status: Option<AutoRenewStatus>,

    /// The billing plan type for the renewal commitment.
    ///
    /// [commitmentRenewalBillingPlanType](https://developer.apple.com/documentation/appstoreserverapi/commitmentrenewalbillingplantype)
    pub commitment_renewal_billing_plan_type: Option<RenewalBillingPlanType>,

    /// The UNIX time, in milliseconds, when the renewal commitment expires.
    ///
    /// [commitmentRenewalDate](https://developer.apple.com/documentation/appstoreserverapi/commitmentrenewaldate)
    #[serde_as(as = "Option<TimestampMilliSeconds<String, Flexible>>")]
    pub commitment_renewal_date: Option<DateTime<Utc>>,

    /// The price of the renewal commitment.
    ///
    /// [commitmentRenewalPrice](https://developer.apple.com/documentation/appstoreserverapi/commitmentrenewalprice)
    pub commitment_renewal_price: Option<i64>,
}