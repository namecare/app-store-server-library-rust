use serde::{Deserialize, Serialize};

/// The success or error information the App Store server records when it attempts to send an App Store server notification to your server.
///
/// [sendAttemptResult](https://developer.apple.com/documentation/appstoreserverapi/sendattemptresult)
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum SendAttemptResult {
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "TIMED_OUT")]
    TimedOut,
    #[serde(rename = "TLS_ISSUE")]
    TlsIssue,
    #[serde(rename = "CIRCULAR_REDIRECT")]
    CircularRedirect,
    #[serde(rename = "NO_RESPONSE")]
    NoResponse,
    #[serde(rename = "SOCKET_ISSUE")]
    SocketIssue,
    #[serde(rename = "UNSUPPORTED_CHARSET")]
    UnsupportedCharset,
    #[serde(rename = "INVALID_RESPONSE")]
    InvalidResponse,
    #[serde(rename = "PREMATURE_CLOSE")]
    PrematureClose,
    #[serde(rename = "UNSUCCESSFUL_HTTP_RESPONSE_CODE")]
    UnsuccessfulHttpResponseCode,
    #[serde(rename = "OTHER")]
    Other,
}
