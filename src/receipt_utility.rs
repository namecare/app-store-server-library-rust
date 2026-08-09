use asn1_rs::{Any, BerParser, Class, FromBer, Integer, SequenceIterator, Tag};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use regex::Regex;
use std::borrow::Cow;

// ASN.1 Type IDs for receipt attributes
const IN_APP_TYPE_ID: i64 = 17;
const TRANSACTION_IDENTIFIER_TYPE_ID: i64 = 1703;
const ORIGINAL_TRANSACTION_IDENTIFIER_TYPE_ID: i64 = 1705;

/// Maximum recursion depth when flattening a constructed BER OCTET STRING.
const MAX_OCTET_STRING_DEPTH: usize = 32;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ReceiptUtilityError {
    #[error("DecodeError: [{0}]")]
    DecodeError(String),

    #[error("InternalBase64DecodeError: [{0}]")]
    InternalBase64DecodeError(#[from] base64::DecodeError),

    #[error("InternalRegexError: [{0}]")]
    InternalRegexError(#[from] regex::Error),
}

/// Extracts a transaction id from an encoded App Receipt. Throws if the receipt does not match the expected format.
/// # Notes
/// *NO validation* is performed on the receipt, and any data returned should only be used to call the App Store Server API.
/// # Arguments
/// * `app_receipt`: The unmodified app receipt
/// # Returns
/// * `Option<String>`: A transaction id from the array of in-app purchases, none if the receipt contains no in-app purchases
pub fn extract_transaction_id_from_app_receipt(app_receipt: &str) -> Result<Option<String>, ReceiptUtilityError> {
    let app_receipt_bytes = STANDARD.decode(app_receipt)?;

    // ContentInfo ::= SEQUENCE { contentType OBJECT IDENTIFIER, content [0] EXPLICIT ANY }
    let (_, container) = Any::from_ber(&app_receipt_bytes)
        .map_err(|e| ReceiptUtilityError::DecodeError(format!("Malformed receipt: {}", e)))?;

    if container.tag() != Tag::Sequence {
        return Err(ReceiptUtilityError::DecodeError(format!(
            "Expected SEQUENCE, got {}",
            container.tag()
        )));
    }

    let mut nodes = SequenceIterator::<Any, BerParser>::new(container.data);

    // contentType OBJECT IDENTIFIER (signedData)
    let Some(Ok(oid)) = nodes.next() else {
        return Ok(None);
    };
    if oid.tag() != Tag::Oid {
        return Err(ReceiptUtilityError::DecodeError(format!(
            "Expected OID, got {}",
            oid.tag()
        )));
    }

    // content [0] EXPLICIT SignedData
    let Some(Ok(tagged)) = nodes.next() else {
        return Ok(None);
    };
    let Some(signed_data) = explicitly_tagged(&tagged, 0) else {
        return Ok(None);
    };
    if signed_data.tag() != Tag::Sequence {
        return Ok(None);
    }

    // SignedData ::= SEQUENCE { version, digestAlgorithms, contentInfo, ... }
    let mut nodes = SequenceIterator::<Any, BerParser>::new(signed_data.data);
    let _version = nodes.next();
    let _digest_algorithms = nodes.next();
    let Some(Ok(content_info)) = nodes.next() else {
        return Ok(None);
    };
    if content_info.tag() != Tag::Sequence {
        return Ok(None);
    }

    // ContentInfo ::= SEQUENCE { contentType OBJECT IDENTIFIER, content [0] EXPLICIT OCTET STRING }
    let mut nodes = SequenceIterator::<Any, BerParser>::new(content_info.data);
    let _content_type = nodes.next();
    let Some(Ok(tagged)) = nodes.next() else {
        return Ok(None);
    };
    let Some(content) = explicitly_tagged(&tagged, 0) else {
        return Ok(None);
    };
    let Some(content) = ber_octet_string(&content, MAX_OCTET_STRING_DEPTH) else {
        return Ok(None);
    };

    Ok(extract_transaction_id_from_app_receipt_inner(&content))
}

/// Iterates the attributes of a receipt payload, which is a SET of
/// SEQUENCE { type INTEGER, version INTEGER, value OCTET STRING }, and returns the
/// first non-none result the `processor` produces for a matching type id.
fn find_attribute_in_set<F>(receipt_content: &[u8], target_type_ids: &[i64], processor: F) -> Option<String>
where
    F: Fn(&[u8]) -> Option<String>,
{
    let (_, set) = Any::from_ber(receipt_content).ok()?;
    if set.tag() != Tag::Set {
        return None;
    }

    let mut result = None;

    for node in SequenceIterator::<Any, BerParser>::new(set.data) {
        let Ok(node) = node else { break };
        if node.tag() != Tag::Sequence {
            continue;
        }

        let mut nodes = SequenceIterator::<Any, BerParser>::new(node.data);
        let (Some(Ok(type_encoded)), Some(Ok(_version)), Some(Ok(value_encoded))) =
            (nodes.next(), nodes.next(), nodes.next())
        else {
            continue;
        };

        let Ok(attribute_type) = Integer::try_from(type_encoded).and_then(|i| i.as_i64()) else {
            continue;
        };
        let Some(value) = ber_octet_string(&value_encoded, MAX_OCTET_STRING_DEPTH) else {
            continue;
        };

        if target_type_ids.contains(&attribute_type) {
            // Matches Swift: the last matching attribute wins, rather than the first.
            if let Some(processed) = processor(&value) {
                result = Some(processed);
            }
        }
    }

    result
}

fn extract_transaction_id_from_app_receipt_inner(app_receipt_content: &[u8]) -> Option<String> {
    find_attribute_in_set(app_receipt_content, &[IN_APP_TYPE_ID], extract_transaction_id_from_in_app_receipt)
}

fn extract_transaction_id_from_in_app_receipt(in_app_receipt_content: &[u8]) -> Option<String> {
    find_attribute_in_set(
        in_app_receipt_content,
        &[TRANSACTION_IDENTIFIER_TYPE_ID, ORIGINAL_TRANSACTION_IDENTIFIER_TYPE_ID],
        |value| {
            let (_, node) = Any::from_ber(value).ok()?;
            if node.tag() != Tag::Utf8String {
                return None;
            }
            node.as_any_string()
                .ok()
                .map(str::to_string)
        },
    )
}

/// Extracts a transaction id from an encoded transactional receipt. Throws if the receipt does not match the expected format.
/// # Notes
/// *NO validation* is performed on the receipt, and any data returned should only be used to call the App Store Server API.
/// # Arguments
/// * `transaction_receipt`: The unmodified transactionReceipt
/// # Returns
/// * `Option<String>`: A transaction id, or none if no transactionId is found in the receipt
pub fn extract_transaction_id_from_transaction_receipt(
    transaction_receipt: &str,
) -> Result<Option<String>, ReceiptUtilityError> {
    let transaction_receipt_bytes = STANDARD.decode(transaction_receipt)?;

    if let Ok(decoded_top_level_str) = String::from_utf8(transaction_receipt_bytes) {
        let purchase_info_regex_str = r#""purchase-info"\s+=\s+"([a-zA-Z0-9+/=]+)";"#;
        let purchase_info_regex = Regex::new(purchase_info_regex_str)?;

        if let Some(purchase_info_match) = purchase_info_regex.captures(&decoded_top_level_str) {
            if let Some(encoded_transaction_id) = purchase_info_match.get(1) {
                if let Ok(decoded_inner_level) = STANDARD.decode(encoded_transaction_id.as_str()) {
                    if let Ok(decoded_inner_level_str) = String::from_utf8(decoded_inner_level) {
                        let transaction_id_regex_str = r#""transaction-id"\s+=\s+"([a-zA-Z0-9+/=]+)";"#;
                        let transaction_id_regex = Regex::new(transaction_id_regex_str)?;

                        if let Some(transaction_id_match) = transaction_id_regex.captures(&decoded_inner_level_str) {
                            if let Some(encoded_transaction_id) = transaction_id_match.get(1) {
                                return Ok(Some(
                                    encoded_transaction_id
                                        .as_str()
                                        .to_string(),
                                ));
                            }
                        };
                    }
                }
            }
        }
    }

    Ok(None)
}

/// Reads a BER OCTET STRING, flattening the constructed form.
///
/// BER permits an OCTET STRING to be encoded as a composition of many individual,
/// recursively encoded (primitive or constructed) OCTET STRINGs. Apple's app receipts
/// use this form (`0x24 0x80 ...`), so the fragments have to be concatenated to
/// recover the content.
fn ber_octet_string<'a>(node: &Any<'a>, depth: usize) -> Option<Cow<'a, [u8]>> {
    if node.tag() != Tag::OctetString {
        return None;
    }

    if node.header.is_primitive() {
        return Some(Cow::Borrowed(node.data));
    }

    if depth == 0 {
        return None;
    }

    let mut nodes = SequenceIterator::<Any, BerParser>::new(node.data);
    let first = match nodes.next() {
        // An empty constructed OCTET STRING is a valid, empty string.
        None => return Some(Cow::Borrowed(&[])),
        Some(node) => ber_octet_string(&node.ok()?, depth - 1)?,
    };

    // Common case: a single fragment, so the inner (already flattened) view is
    // returned as-is rather than copied.
    let second = match nodes.next() {
        None => return Some(first),
        Some(node) => ber_octet_string(&node.ok()?, depth - 1)?,
    };

    let mut flattened = first.into_owned();
    flattened.extend_from_slice(&second);
    for node in nodes {
        flattened.extend_from_slice(&ber_octet_string(&node.ok()?, depth - 1)?);
    }

    Some(Cow::Owned(flattened))
}

/// Reads an explicitly tagged context-specific [n] node and returns its inner node.
fn explicitly_tagged<'a>(node: &Any<'a>, tag_number: u32) -> Option<Any<'a>> {
    if node.class() != Class::ContextSpecific || node.tag() != Tag(tag_number) {
        return None;
    }

    Any::from_ber(node.data)
        .ok()
        .map(|(_, inner)| inner)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ber_octet_string_primitive() {
        let data = [0x04, 0x05, 0x48, 0x65, 0x6c, 0x6c, 0x6f]; // OCTET STRING "Hello"
        let (_, node) = Any::from_ber(&data).unwrap();
        assert_eq!(
            ber_octet_string(&node, MAX_OCTET_STRING_DEPTH).as_deref(),
            Some(&b"Hello"[..])
        );
    }

    #[test]
    fn test_ber_octet_string_constructed_indefinite() {
        let data = [
            0x24, 0x80, // constructed OCTET STRING, indefinite length
            0x04, 0x03, 0x48, 0x65, 0x6c, // "Hel"
            0x04, 0x02, 0x6c, 0x6f, // "lo"
            0x00, 0x00, // end-of-contents
        ];
        let (_, node) = Any::from_ber(&data).unwrap();
        assert_eq!(
            ber_octet_string(&node, MAX_OCTET_STRING_DEPTH).as_deref(),
            Some(&b"Hello"[..])
        );
    }

    #[test]
    fn test_ber_octet_string_constructed_single_fragment() {
        let data = [
            0x24, 0x80, // constructed OCTET STRING, indefinite length
            0x04, 0x05, 0x48, 0x65, 0x6c, 0x6c, 0x6f, // "Hello"
            0x00, 0x00, // end-of-contents
        ];
        let (_, node) = Any::from_ber(&data).unwrap();
        assert_eq!(
            ber_octet_string(&node, MAX_OCTET_STRING_DEPTH).as_deref(),
            Some(&b"Hello"[..])
        );
    }

    #[test]
    fn test_ber_octet_string_nested_constructed() {
        let data = [
            0x24, 0x80, // constructed OCTET STRING, indefinite length
            0x24, 0x06, // nested constructed OCTET STRING
            0x04, 0x02, 0x48, 0x65, // "He"
            0x04, 0x00, // ""
            0x04, 0x03, 0x6c, 0x6c, 0x6f, // "llo"
            0x00, 0x00, // end-of-contents
        ];
        let (_, node) = Any::from_ber(&data).unwrap();
        assert_eq!(
            ber_octet_string(&node, MAX_OCTET_STRING_DEPTH).as_deref(),
            Some(&b"Hello"[..])
        );
    }

    #[test]
    fn test_ber_octet_string_empty_constructed() {
        let data = [0x24, 0x80, 0x00, 0x00];
        let (_, node) = Any::from_ber(&data).unwrap();
        assert_eq!(
            ber_octet_string(&node, MAX_OCTET_STRING_DEPTH).as_deref(),
            Some(&b""[..])
        );
    }

    #[test]
    fn test_ber_octet_string_wrong_tag() {
        let data = [0x0C, 0x05, 0x48, 0x65, 0x6c, 0x6c, 0x6f]; // UTF8String
        let (_, node) = Any::from_ber(&data).unwrap();
        assert!(ber_octet_string(&node, MAX_OCTET_STRING_DEPTH).is_none());
    }

    #[test]
    fn test_ber_octet_string_depth_limit() {
        let data = [
            0x24, 0x04, // constructed OCTET STRING
            0x24, 0x02, // nested constructed OCTET STRING
            0x04, 0x00, // ""
        ];
        let (_, node) = Any::from_ber(&data).unwrap();
        assert!(ber_octet_string(&node, 1).is_none());
        assert!(ber_octet_string(&node, 2).is_some());
    }

    #[test]
    fn test_explicitly_tagged() {
        let data = [
            0xA0, 0x03, // [0] EXPLICIT
            0x02, 0x01, 0x05, // INTEGER 5
        ];
        let (_, node) = Any::from_ber(&data).unwrap();
        let inner = explicitly_tagged(&node, 0).expect("Expected inner node");
        assert_eq!(inner.tag(), Tag::Integer);
    }

    #[test]
    fn test_explicitly_tagged_wrong_number() {
        let data = [0xA0, 0x03, 0x02, 0x01, 0x05];
        let (_, node) = Any::from_ber(&data).unwrap();
        assert!(explicitly_tagged(&node, 1).is_none());
    }

    #[test]
    fn test_explicitly_tagged_wrong_class() {
        let data = [0x30, 0x03, 0x02, 0x01, 0x05]; // SEQUENCE, universal class
        let (_, node) = Any::from_ber(&data).unwrap();
        assert!(explicitly_tagged(&node, 0).is_none());
    }

    #[test]
    fn test_extract_from_app_receipt_invalid_base64() {
        let result = extract_transaction_id_from_app_receipt("not base64!!!");
        assert!(matches!(
            result,
            Err(ReceiptUtilityError::InternalBase64DecodeError(_))
        ));
    }

    #[test]
    fn test_extract_from_app_receipt_not_asn1() {
        // Valid base64, but not a valid BER structure.
        let result = extract_transaction_id_from_app_receipt(&STANDARD.encode([0xFF, 0xFF, 0xFF]));
        assert!(matches!(result, Err(ReceiptUtilityError::DecodeError(_))));
    }

    #[test]
    fn test_extract_from_app_receipt_truncated() {
        // A SEQUENCE header claiming more content than is present must not panic.
        let result = extract_transaction_id_from_app_receipt(&STANDARD.encode([0x30, 0x7F, 0x06, 0x01]));
        assert!(result.is_err());
    }
}
