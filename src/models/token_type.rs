use serde::{Deserialize, Serialize};

/// The type of an external purchase custom link token.
/// The token type field is present only for custom link tokens.
/// For more information on tokens, see Receiving and decoding external purchase tokens.
///
/// [tokenType](https://developer.apple.com/documentation/appstoreservernotifications/tokentype)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum TokenType {
    /// A token type that indicates an initial acquisition.
    #[serde(rename = "ACQUISITION")]
    Acquisition,

    /// A token type that indicates usage of App Store services.
    #[serde(rename = "SERVICES")]
    Services,
}
