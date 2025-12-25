use serde::{Deserialize, Serialize};

/// The response times measured during a performance test.
///
/// [PerformanceTestResponseTimes](https://developer.apple.com/documentation/retentionmessaging/performancetestresponsetimes)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct PerformanceTestResponseTimes {
    /// Average response time in milliseconds.
    pub average: i64,

    /// The 50th percentile response time in milliseconds.
    pub p50: i64,

    /// The 90th percentile response time in milliseconds.
    pub p90: i64,

    /// The 95th percentile response time in milliseconds.
    pub p95: i64,

    /// The 99th percentile response time in milliseconds.
    pub p99: i64,
}