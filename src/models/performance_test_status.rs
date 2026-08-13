use serde::{Deserialize, Serialize};

/// The status of a performance test.
///
/// [PerformanceTestStatus](https://developer.apple.com/documentation/retentionmessaging/performanceteststatus)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum PerformanceTestStatus {
    /// The test is still pending.
    #[serde(rename = "PENDING")]
    Pending,
    /// The test passed.
    #[serde(rename = "PASS")]
    Pass,
    /// The test failed.
    #[serde(rename = "FAIL")]
    Fail,

    /// A value the App Store sent that this version of the
    /// library does not support, preserved as received.
    #[serde(untagged)]
    NotSupported(String),
}
