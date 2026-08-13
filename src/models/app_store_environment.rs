use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum Environment {
    #[serde(rename = "Sandbox")]
    Sandbox,
    #[serde(rename = "Production")]
    Production,
    #[serde(rename = "Xcode")]
    Xcode,
    #[serde(rename = "LocalTesting")]
    LocalTesting, // Used for unit testing,

    /// A value the App Store sent that this version of the
    /// library does not support, preserved as received.
    #[serde(untagged)]
    NotSupported(String),
}

impl Environment {
    pub fn base_url(&self) -> String {
        match self {
            Environment::Production => "https://api.storekit.apple.com".to_string(),
            Environment::Sandbox => "https://api.storekit-sandbox.apple.com".to_string(),
            Environment::LocalTesting => "https://local-testing-base-url".to_string(),
            _ => "https://api.storekit-sandbox.apple.com".to_string(),
        }
    }
}
