use serde::{Deserialize, Serialize};

/// Values that represent Apple platforms.
///
/// [PurchasePlatform](https://developer.apple.com/documentation/storekit/appstore/platform)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum PurchasePlatform {
    #[serde(rename = "iOS")]
    IOS,
    #[serde(rename = "macOS")]
    MacOs,
    #[serde(rename = "tvOS")]
    TvOs,
    #[serde(rename = "visionOS")]
    VisionOs,
    #[serde(rename = "watchOS")]
    WatchOs,
}
