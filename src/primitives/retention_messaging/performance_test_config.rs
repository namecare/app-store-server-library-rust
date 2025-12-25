use serde::{Deserialize, Serialize};

/// The performance test configuration object.
///
/// [PerformanceTestConfig](https://developer.apple.com/documentation/retentionmessaging/performancetestconfig)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PerformanceTestConfig {
    /// The maximum number of concurrent requests the API allows.
    pub max_concurrent_requests: i64,

    /// The response time threshold in milliseconds.
    pub response_time_threshold: i64,

    /// The success rate threshold percentage.
    pub success_rate_threshold: i32,

    /// The total duration of the test in milliseconds.
    pub total_duration: i64,

    /// The total number of requests to make during the test.
    pub total_requests: i64,
}