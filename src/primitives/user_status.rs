use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum UserStatus {
    Undeclared = 0,
    Active = 1,
    Suspended = 2,
    Terminated = 3,
    LimitedAccess = 4,
}
