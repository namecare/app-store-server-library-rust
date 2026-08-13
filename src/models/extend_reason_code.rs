use serde::{Deserialize, Serialize};

/// The code that represents the reason for the subscription-renewal-date extension.
///
/// [extendReasonCode](https://developer.apple.com/documentation/appstoreserverapi/extendreasoncode)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(from = "i64", into = "i64")]
pub enum ExtendReasonCode {
    Undeclared,
    CustomerSatisfaction,
    Other,
    ServiceIssueOrOutage,

    /// A value the App Store sent that this version of the
    /// library does not support, preserved as received.
    NotSupported(i64),
}

impl From<i64> for ExtendReasonCode {
    fn from(value: i64) -> Self {
        match value {
            0 => ExtendReasonCode::Undeclared,
            1 => ExtendReasonCode::CustomerSatisfaction,
            2 => ExtendReasonCode::Other,
            3 => ExtendReasonCode::ServiceIssueOrOutage,
            other => ExtendReasonCode::NotSupported(other),
        }
    }
}

impl From<ExtendReasonCode> for i64 {
    fn from(value: ExtendReasonCode) -> Self {
        match value {
            ExtendReasonCode::Undeclared => 0,
            ExtendReasonCode::CustomerSatisfaction => 1,
            ExtendReasonCode::Other => 2,
            ExtendReasonCode::ServiceIssueOrOutage => 3,
            ExtendReasonCode::NotSupported(other) => other,
        }
    }
}
