use serde::{Deserialize, Serialize};

/// The object you provide to a performance test request that contains the test's transaction identifier.
///
/// [PerformanceTestRequest](https://developer.apple.com/documentation/retentionmessaging/performancetestrequest)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PerformanceTestRequest {
    /// The original transaction identifier of an In-App Purchase to use as the purchase for this test.
    pub original_transaction_id: String,
}