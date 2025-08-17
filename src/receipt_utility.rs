use crate::asn1::asn1_basics::*;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use regex::Regex;

// ASN.1 Type IDs for receipt attributes
const IN_APP_TYPE_ID: u64 = 17;
const TRANSACTION_IDENTIFIER_TYPE_ID: u64 = 1703;
const ORIGINAL_TRANSACTION_IDENTIFIER_TYPE_ID: u64 = 1705;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ReceiptUtilityError {
    #[error("DecodeError: [{0}]")]
    DecodeError(String),

    #[error("InternalBase64DecodeError: [{0}]")]
    InternalBase64DecodeError(#[from] base64::DecodeError),

    #[error("InternalASN1DecodeError: [{0}]")]
    InternalASN1DecodeError(#[from] ASN1Error),

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

    // Parse the outer PKCS7 structure using custom BER parser
    let mut offset = 0;

    // Read SEQUENCE
    let (content_offset, _) = read_sequence(&app_receipt_bytes, offset)?;
    offset = content_offset;

    // Skip OID
    let _ = read_oid(&app_receipt_bytes, offset)?;
    offset = skip(&app_receipt_bytes, offset)?;

    // Read context-specific [0]
    let (content_offset, _) = read_context_specific_0(&app_receipt_bytes, offset)?;
    offset = content_offset;

    // Read inner SEQUENCE
    let (content_offset, _) = read_sequence(&app_receipt_bytes, offset)?;
    offset = content_offset;

    // Skip two items
    offset = skip(&app_receipt_bytes, offset)?;
    offset = skip(&app_receipt_bytes, offset)?;

    // Read receipt content SEQUENCE
    let (content_offset, _) = read_sequence(&app_receipt_bytes, offset)?;
    offset = content_offset;

    // Skip OID
    offset = skip(&app_receipt_bytes, offset)?;

    // Read context-specific [0] with receipt data
    let (content_offset, _) = read_context_specific_0(&app_receipt_bytes, offset)?;
    offset = content_offset;

    // Read indefinite-length content starting with 0x24 0x80 or direct OCTET STRING
    let (tag, length, content_offset) = read_tlv(&app_receipt_bytes, offset)?;
    if tag == TAG_CONTEXT_SPECIFIC_CONSTRUCTED_4 {
        // BIT STRING with indefinite length
        offset = content_offset;

        // Read OCTET STRING
        let (content_offset, length) = read_octet_string(&app_receipt_bytes, offset)?;

        // Read the receipt data - if indefinite length, look for end marker
        let receipt_data = get_content(&app_receipt_bytes, content_offset, length)?;

        extract_transaction_id_from_app_receipt_inner(receipt_data)
    } else if tag == TAG_OCTET_STRING {
        // Direct OCTET STRING
        let receipt_data = get_content(&app_receipt_bytes, content_offset, length)?;
        extract_transaction_id_from_app_receipt_inner(receipt_data)
    } else {
        Err(ReceiptUtilityError::DecodeError(format!(
            "Unexpected tag: 0x{:02x}",
            tag
        )))
    }
}

/// Helper function to unwrap content if it's wrapped in an OCTET STRING
fn unwrap_octet_string(data: &[u8]) -> &[u8] {
    if let Ok((tag, length, content_offset)) = read_tlv(data, 0) {
        if tag == TAG_OCTET_STRING {
            return &data[content_offset..content_offset + length];
        }
    }
    data
}

/// Helper function to parse an ASN.1 attribute (SEQUENCE with type, version, value)
fn parse_attribute(data: &[u8], offset: usize) -> Result<(u64, usize), ReceiptUtilityError> {
    // Read type integer
    let type_int = read_integer(data, offset)?;
    // Skip type integer
    let after_type_offset = skip(data, offset)?;
    // Skip version integer
    let after_version_offset = skip(data, after_type_offset)?;
    Ok((type_int, after_version_offset))
}

/// Helper function to find an attribute with a specific type ID in a SET
fn find_attribute_in_set<F>(
    set_data: &[u8],
    target_type_ids: &[u64],
    processor: F,
) -> Result<Option<String>, ReceiptUtilityError>
where
    F: Fn(&[u8], usize) -> Result<Option<String>, ReceiptUtilityError>,
{
    let mut offset = 0;

    // Parse as SET
    let (content_offset, set_length) = read_set(set_data, offset)?;
    offset = content_offset;
    let set_end = if set_length == usize::MAX {
        set_data.len()
    } else {
        content_offset + set_length
    };

    while offset < set_end {
        let (tag, seq_length, content_offset) = read_tlv(set_data, offset)?;
        if tag == TAG_SEQUENCE {
            let (type_int, after_version_offset) = parse_attribute(set_data, content_offset)?;

            if target_type_ids.contains(&type_int) {
                if let Some(result) = processor(set_data, after_version_offset)? {
                    return Ok(Some(result));
                }
            }

            // Move to next item
            offset = if seq_length == usize::MAX {
                find_end_of_contents(set_data, content_offset)?
            } else {
                content_offset + seq_length
            };
        } else if tag == 0x00 && set_length == usize::MAX {
            // End-of-contents marker
            break;
        } else {
            // Skip this item
            offset = skip(set_data, offset)?;
        }
    }

    Ok(None)
}

fn extract_transaction_id_from_app_receipt_inner(
    app_receipt_content: &[u8],
) -> Result<Option<String>, ReceiptUtilityError> {
    // Unwrap if wrapped in OCTET STRING
    let content_to_parse = unwrap_octet_string(app_receipt_content);

    find_attribute_in_set(content_to_parse, &[IN_APP_TYPE_ID], |data, offset| {
        // Read OCTET STRING containing in-app data
        if let Ok((content_offset, length)) = read_octet_string(data, offset) {
            let in_app_data = &data[content_offset..content_offset + length];
            extract_transaction_id_from_in_app_receipt(in_app_data)
        } else {
            Ok(None)
        }
    })
}

fn extract_transaction_id_from_in_app_receipt(
    app_receipt_content: &[u8],
) -> Result<Option<String>, ReceiptUtilityError> {
    // Unwrap if wrapped in OCTET STRING
    let set_data = unwrap_octet_string(app_receipt_content);

    find_attribute_in_set(
        set_data,
        &[TRANSACTION_IDENTIFIER_TYPE_ID, ORIGINAL_TRANSACTION_IDENTIFIER_TYPE_ID],
        |data, offset| {
            // Read OCTET STRING containing the transaction ID
            if let Ok((content_offset, length)) = read_octet_string(data, offset) {
                let octet_data = &data[content_offset..content_offset + length];
                // Parse UTF8String from the OCTET STRING
                if let Ok(utf8_str) = read_utf8_string(octet_data, 0) {
                    return Ok(Some(utf8_str));
                }
            }
            Ok(None)
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
                                return Ok(Some(encoded_transaction_id.as_str().to_string()));
                            }
                        };
                    }
                }
            }
        }
    }

    Ok(None)
}
