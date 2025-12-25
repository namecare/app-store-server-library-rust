use serde::{Deserialize, Serialize};
use crate::primitives::retention_messaging::failures::Failures;
use crate::primitives::retention_messaging::performance_test_config::PerformanceTestConfig;
use crate::primitives::retention_messaging::performance_test_response_times::PerformanceTestResponseTimes;
use crate::primitives::retention_messaging::performance_test_status::PerformanceTestStatus;

/// The response from the Get Performance Test Results API call.
///
/// [PerformanceTestResultResponse](https://developer.apple.com/documentation/retentionmessaging/performancetestresultresponse)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PerformanceTestResultResponse {
    /// A PerformanceTestConfig object that enumerates the test parameters.
    pub config: PerformanceTestConfig,

    /// A Failures object that represents a map of server-to-server notification failure reasons
    /// and counts that represent the number of failures encountered during the performance test.
    pub failures: Failures,

    /// An integer that describes the number of pending requests in the performance test.
    pub num_pending: i32,

    /// A PerformanceTestResponseTimes object that enumerates the response times measured during the test.
    pub response_times: PerformanceTestResponseTimes,

    /// A PerformanceTestStatus object that describes the overall result of the test.
    pub result: PerformanceTestStatus,

    /// An integer that describes the success rate percentage of the performance test.
    pub success_rate: i32,

    /// The target URL for the performance test.
    pub target: String,
}