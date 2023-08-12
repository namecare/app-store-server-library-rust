use serde::{Deserialize, Serialize};

/// An error or result that the App Store server receives when attempting to send an App Store server notification to your server.
///
/// [firstSendAttemptResult](https://developer.apple.com/documentation/appstoreserverapi/firstsendattemptresult)
#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum FirstSendAttemptResult {
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
    UnsuportedCharset,
    #[serde(rename = "INVALID_RESPONSE")]
    InvalidResponse,
    #[serde(rename = "PREMATURE_CLOSE")]
    PrematureClose,
    #[serde(rename = "UNSUCCESSFUL_HTTP_RESPONSE_CODE")]
    UnsuccessfulHttpResponseCode,
    #[serde(rename = "OTHER")]
    Other,
}
