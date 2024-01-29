use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Hash)]
pub struct ErrorPayload {
    #[serde(rename = "errorCode")]
    pub error_code: Option<i64>,

    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
}
