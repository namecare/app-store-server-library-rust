use serde_repr::{Serialize_repr, Deserialize_repr};

/// The code that represents the reason for the subscription-renewal-date extension.
///
/// [extendReasonCode](https://developer.apple.com/documentation/appstoreserverapi/extendreasoncode)
#[derive(Debug, Clone, Deserialize_repr, Serialize_repr, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum ExtendReasonCode {
    Undeclared = 0,
    CustomerSatisfaction = 1,
    Other = 2,
    ServiceIssueOrOutage = 3,
}
