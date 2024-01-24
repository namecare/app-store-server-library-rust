use base64::engine::general_purpose::STANDARD;
use base64::{DecodeError, Engine};
use std::time::SystemTime;

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
#[allow(dead_code)]
pub(crate) fn system_timestamp() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

/// Converts a base64URL-encoded string to a standard base64-encoded string.
///
/// Replaces '/' with '+' and '_' with '-', and adds padding if needed.
///
/// # Examples
///
/// ```ignore
/// let encoded_string = "aGVsbG8gd29ybGQh";
/// let result = base64_url_to_base64(encoded_string);
/// assert_eq!(result, "aGVsbG8gd29ybGQh==");
/// ```
pub(crate) fn base64_url_to_base64(encoded_string: &str) -> String {
    let replaced_string = encoded_string.replace('/', "+").replace('_', "-");

    if replaced_string.len() % 4 != 0 {
        return replaced_string.clone() + &"=".repeat(4 - replaced_string.len() % 4);
    }

    replaced_string
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

impl StringExt for String {
    fn as_der_bytes(&self) -> Result<Vec<u8>, DecodeError> {
        STANDARD.decode(self)
    }
}

impl StringExt for &str {
    fn as_der_bytes(&self) -> Result<Vec<u8>, DecodeError> {
        STANDARD.decode(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_url_to_base64() {
        // Test with a base64URL-encoded string
        let encoded_string = "aGVsbG8gd29ybGQh";
        let result = base64_url_to_base64(encoded_string);
        assert_eq!(result, "aGVsbG8gd29ybGQh");

        // Test with a base64URL-encoded string requiring padding
        let encoded_string_padding = "aGVsbG8gd29ybz";
        let result_padding = base64_url_to_base64(encoded_string_padding);
        assert_eq!(result_padding, "aGVsbG8gd29ybz==");
    }
}
