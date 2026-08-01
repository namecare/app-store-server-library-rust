use base64::engine::general_purpose::STANDARD;
use base64::{DecodeError, Engine};

/// Percent-encodes a string for safe use as a URL query parameter value.
pub(crate) fn percent_encode_query_value(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.as_bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(*byte as char);
            }
            _ => {
                encoded.push('%');
                encoded.push_str(&format!("{:02X}", byte));
            }
        }
    }
    encoded
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

// `base64_url_to_base64` and its test were removed with the `jsonwebtoken`
// migration: `crate::jws` now owns base64url handling and covers it in
// `tests/jws.rs` (`b64url_round_trips_without_padding`,
// `b64url_decode_accepts_unpadded_input`).
