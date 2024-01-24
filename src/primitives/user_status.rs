use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum UserStatus {
    Undeclared = 0,
    Active = 1,
    Suspended = 2,
    Terminated = 3,
    LimitedAccess = 4,
}
