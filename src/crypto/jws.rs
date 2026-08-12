//! JWS (RFC 7515) compact-serialization handling.
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use serde::de::DeserializeOwned;
use serde::Deserialize;

const EXPECTED_SEGMENTS: usize = 3;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum JwsError {
    #[error("InvalidFormat")]
    InvalidFormat,

    #[error("InvalidBase64: [{0}]")]
    InvalidBase64(String),

    #[error("InvalidJson: [{0}]")]
    InvalidJson(String),
}

/// The decoded JOSE header.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct JwtHeader {
    #[serde(default)]
    pub alg: Option<String>,
    #[serde(default)]
    pub x5c: Option<Vec<String>>,
}

/// Splits a compact JWS into exactly three segments.
pub fn split(token: &str) -> Result<[&str; 3], JwsError> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != EXPECTED_SEGMENTS {
        return Err(JwsError::InvalidFormat);
    }
    Ok([parts[0], parts[1], parts[2]])
}

/// Decodes the JOSE header of a compact JWS.
pub fn decode_header(token: &str) -> Result<JwtHeader, JwsError> {
    let [header, _, _] = split(token)?;
    let bytes = b64url_decode(header)?;
    serde_json::from_slice(&bytes).map_err(|e| JwsError::InvalidJson(e.to_string()))
}

/// The bytes covered by the signature: `header.payload`.
pub fn signing_input(token: &str) -> Result<String, JwsError> {
    let [header, payload, _] = split(token)?;
    Ok(format!("{header}.{payload}"))
}

/// The decoded payload segment.
pub fn decode_payload_bytes(token: &str) -> Result<Vec<u8>, JwsError> {
    let [_, payload, _] = split(token)?;
    b64url_decode(payload)
}

/// Decodes and JSON-deserializes the payload segment of a compact JWS.
pub fn decode_payload<T: DeserializeOwned>(token: &str) -> Result<T, JwsError> {
    let bytes = decode_payload_bytes(token)?;
    serde_json::from_slice(&bytes).map_err(|e| JwsError::InvalidJson(e.to_string()))
}

/// The decoded signature segment.
pub fn decode_signature_bytes(token: &str) -> Result<Vec<u8>, JwsError> {
    let [_, _, signature] = split(token)?;
    b64url_decode(signature)
}

/// base64url, unpadded (RFC 7515 §2).
pub fn b64url_encode(data: &[u8]) -> String {
    URL_SAFE_NO_PAD.encode(data)
}

/// base64url decode, tolerating both padded and unpadded input.
pub fn b64url_decode(data: &str) -> Result<Vec<u8>, JwsError> {
    let trimmed = data.trim_end_matches('=');
    URL_SAFE_NO_PAD
        .decode(trimmed)
        .map_err(|e| JwsError::InvalidBase64(e.to_string()))
}

/// Assembles a compact JWS from already-encoded header and payload segments
/// plus a raw signature.
pub fn encode_compact(encoded_header: &str, encoded_payload: &str, signature: &[u8]) -> String {
    format!(
        "{encoded_header}.{encoded_payload}.{}",
        b64url_encode(signature)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // A minimal ES256 JWS with a two-element x5c. Signature is not verified here.
    const SAMPLE: &str = "eyJhbGciOiJFUzI1NiIsIng1YyI6WyJhYWEiLCJiYmIiXX0.eyJmb28iOiJiYXIifQ.c2ln";

    #[test]
    fn decodes_header_alg_and_x5c() {
        let header = decode_header(SAMPLE).expect("valid header");
        assert_eq!(header.alg.as_deref(), Some("ES256"));
        assert_eq!(header.x5c, Some(vec!["aaa".to_string(), "bbb".to_string()]));
    }

    #[test]
    fn header_without_x5c_decodes_with_none() {
        // {"alg":"ES256"}
        let jws_str = "eyJhbGciOiJFUzI1NiJ9.eyJmb28iOiJiYXIifQ.c2ln";
        let header = decode_header(jws_str).expect("valid header");
        assert_eq!(header.alg.as_deref(), Some("ES256"));
        assert_eq!(header.x5c, None);
    }

    #[test]
    fn rejects_wrong_segment_count() {
        assert!(matches!(
            decode_header("only.two"),
            Err(JwsError::InvalidFormat)
        ));
        assert!(matches!(
            decode_header("a.b.c.d"),
            Err(JwsError::InvalidFormat)
        ));
        assert!(matches!(decode_header(""), Err(JwsError::InvalidFormat)));
    }

    #[test]
    fn rejects_non_base64_header() {
        assert!(decode_header("!!!.eyJmb28iOiJiYXIifQ.c2ln").is_err());
    }

    #[test]
    fn rejects_header_that_is_not_json() {
        // "notjson" base64url-encoded
        assert!(decode_header("bm90anNvbg.eyJmb28iOiJiYXIifQ.c2ln").is_err());
    }

    #[test]
    fn signing_input_is_first_two_segments() {
        assert_eq!(
            signing_input(SAMPLE).unwrap(),
            "eyJhbGciOiJFUzI1NiIsIng1YyI6WyJhYWEiLCJiYmIiXX0.eyJmb28iOiJiYXIifQ"
        );
    }

    #[test]
    fn decodes_payload_bytes() {
        let payload = decode_payload_bytes(SAMPLE).unwrap();
        assert_eq!(payload, br#"{"foo":"bar"}"#);
    }

    #[test]
    fn b64url_round_trips_without_padding() {
        // 0xFB 0xFF exercises the -_ alphabet; length 2 would need padding.
        let encoded = b64url_encode(&[0xFB, 0xFF]);
        assert!(!encoded.contains('='), "must be unpadded, got {encoded}");
        assert!(!encoded.contains('+') && !encoded.contains('/'));
        assert_eq!(b64url_decode(&encoded).unwrap(), vec![0xFB, 0xFF]);
    }

    #[test]
    fn b64url_decode_accepts_unpadded_input() {
        assert_eq!(b64url_decode("YWJj").unwrap(), b"abc");
        assert_eq!(b64url_decode("YQ").unwrap(), b"a");
    }

    #[test]
    fn encode_compact_joins_three_segments() {
        let token = encode_compact("aGRy", "cGF5", &[0x01, 0x02]);
        assert_eq!(token, "aGRy.cGF5.AQI");
        assert_eq!(token.split('.').count(), 3);
    }
}
