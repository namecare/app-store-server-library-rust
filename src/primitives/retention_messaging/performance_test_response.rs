use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::primitives::retention_messaging::performance_test_config::PerformanceTestConfig;

/// The response from initiating a performance test.
///
/// [PerformanceTestResponse](https://developer.apple.com/documentation/retentionmessaging/performancetestresponse)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PerformanceTestResponse {
    /// The performance test configuration object.
    pub config: PerformanceTestConfig,

    /// The performance test request identifier.
    pub request_id: Uuid,
}