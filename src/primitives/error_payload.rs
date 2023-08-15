use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Hash)]
pub struct ErrorPayload {
    pub error_code: Option<i64>,
    pub error_message: Option<String>,
}