use std::time::{SystemTime};
use base64::{DecodeError, Engine};
use base64::engine::general_purpose::STANDARD;

/// Returns the current system timestamp in seconds since the UNIX EPOCH.
///
/// The function retrieves the current system time and calculates the duration
/// since the UNIX EPOCH (January 1, 1970, UTC) in seconds. If the system time
/// is earlier than the UNIX EPOCH, a panic is triggered.
///
/// # Panics
///
/// This function may panic if the system time is earlier than the UNIX EPOCH,
/// which is an invalid state for most systems.
///
pub(crate) fn system_timestamp() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

/// A trait for extending the functionality of Rust strings.
pub trait StringExt {
    /// Converts the string into a DER-encoded byte vector.
    ///
    /// This method attempts to parse the string as a DER-encoded byte sequence
    /// and returns the result as a `Vec<u8>`. If the parsing fails, it returns
    /// a `DecodeError`.
    ///
    /// # Errors
    ///
    /// If the string cannot be successfully parsed as DER-encoded bytes, this
    /// method returns a `DecodeError` indicating the reason for the failure.
    ///
    fn as_der_bytes(&self) -> Result<Vec<u8>, DecodeError>;
}

impl StringExt for String  {
    fn as_der_bytes(&self) -> Result<Vec<u8>, DecodeError> {
        STANDARD.decode(self)
    }
}

impl StringExt for &str  {
    fn as_der_bytes(&self) -> Result<Vec<u8>, DecodeError> {
        STANDARD.decode(self)
    }
}