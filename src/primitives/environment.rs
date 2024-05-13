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
    LocalTesting, // Used for unit testing
    #[serde(other)]
    Unknown
}

impl Environment {
    pub fn base_url(&self) -> String {
        match self {
            Environment::Production => "https://api.storekit.itunes.apple.com".to_string(),
            Environment::Sandbox => "https://api.storekit-sandbox.itunes.apple.com".to_string(),
            Environment::LocalTesting => "https://local-testing-base-url".to_string(),
            _ => "https://api.storekit-sandbox.itunes.apple.com".to_string(),
        }
    }
}