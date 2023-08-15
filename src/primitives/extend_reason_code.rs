use serde::{Deserialize, Serialize};

/// The code that represents the reason for the subscription-renewal-date extension.
///
/// [extendReasonCode](https://developer.apple.com/documentation/appstoreserverapi/extendreasoncode)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum ExtendReasonCode {
    Undeclared = 0,
    CustomerSatisfaction = 1,
    Other = 2,
    ServiceIssueOrOutage = 3,
}
