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

pub trait StringExt {
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