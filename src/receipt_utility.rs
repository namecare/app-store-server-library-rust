use asn1_rs::{Any, Class, Error, Explicit, FromBer, Integer, OctetString, Oid, Sequence, Set, TaggedValue, Utf8String};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use regex::Regex;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ReceiptUtilityError {
    #[error("InternalBase64DecodeError: [{0}]")]
    InternalBase64DecodeError(#[from] base64::DecodeError),

    #[error("InternalASN1DecodeError: [{0}]")]
    InternalASN1DecodeError(#[from] asn1_rs::Error),

    #[error("InternalASN1Error: [{0}]")]
    InternalASN1Error(#[from] asn1_rs::Err<asn1_rs::Error>),

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

    let (_, transaction_id) = Sequence::from_ber_and_then(app_receipt_bytes.as_slice(), |i| {
        // Skip the first object identifier
        let (i, _) = Oid::from_ber(i)?;
        let (i, value) =
            TaggedValue::<Sequence, Error, Explicit, { Class::CONTEXT_SPECIFIC }, 0>::from_ber(
                i,
            )?;

        let seq = value.into_inner();
        seq.and_then(|ii| {
            let (ii, _) = Any::from_ber(&ii)?; // Skip
            let (ii, _) = Any::from_ber(&ii)?; // Skip

            let r: (&[u8], Option<String>) = Sequence::from_ber_and_then(&ii, |iii| {
                let (iii, _) = Oid::from_ber(iii)?; // Skip

                let (iii, value) = TaggedValue::<
                    OctetString,
                    Error,
                    Explicit,
                    { Class::CONTEXT_SPECIFIC },
                    0,
                >::from_ber(&iii)?;

                let content = value.into_inner();
                let transaction_id = extract_transaction_id_from_app_receipt_inner(content.as_ref())?;

                return Ok((iii, transaction_id));
            })?;

            let (ii, _) = Any::from_ber(&ii)?; // Skip
            let (ii, _) = Any::from_ber(&ii)?; // Skip
            let (_, _) = Any::from_ber(&ii)?; // Skip

            return Ok((i, r.1));
        })
    })?;

    Ok(transaction_id)
}

fn extract_transaction_id_from_app_receipt_inner(app_receipt_content: &[u8]) -> Result<Option<String>, asn1_rs::Err<Error>> {
    const IN_APP_TYPE_ID: u64 = 17u64;

    let (_, octet_string) = OctetString::from_ber(app_receipt_content)?;
    let (_, set) = Set::from_ber(octet_string.as_ref())?;

    for (_, item) in set.ber_iter::<Sequence, Error>().enumerate() {
        if let Ok(seq) = item {
            let (ii, t) = Integer::from_ber(&seq.content)?;
            let (ii, _) = Integer::from_ber(&ii)?;

            let t = t.as_u64()?;

            if t == IN_APP_TYPE_ID {
                return extract_transaction_id_from_in_app_receipt(ii);
            }
        }
    }

    return Ok(None);
}

fn extract_transaction_id_from_in_app_receipt(app_receipt_content: &[u8]) -> Result<Option<String>, asn1_rs::Err<Error>> {
    const TRANSACTION_IDENTIFIER_TYPE_ID: u64 = 1703u64;
    const ORIGINAL_TRANSACTION_IDENTIFIER_TYPE_ID: u64 = 1705u64;

    let (_, octet_string) = OctetString::from_ber(app_receipt_content)?;
    let (_, set) = Set::from_ber(octet_string.as_ref())?;

    for (_, item) in set.ber_iter::<Sequence, Error>().enumerate() {
        if let Ok(seq) = item {
            let (ii, t) = Integer::from_ber(&seq.content)?;
            let (ii, _) = Integer::from_ber(&ii)?;

            let t = t.as_u64()?;

            if t == TRANSACTION_IDENTIFIER_TYPE_ID || t == ORIGINAL_TRANSACTION_IDENTIFIER_TYPE_ID {
                let (_, octet_string) = OctetString::from_ber(&ii)?;
                let (_, transaction_id) = Utf8String::from_ber(octet_string.as_ref())?;
                return Ok(Some(transaction_id.string()));
            }
        }
    }

    return Ok(None);
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
                        let transaction_id_regex_str =
                            r#""transaction-id"\s+=\s+"([a-zA-Z0-9+/=]+)";"#;
                        let transaction_id_regex = Regex::new(transaction_id_regex_str)?;

                        if let Some(transaction_id_match) =
                            transaction_id_regex.captures(&decoded_inner_level_str)
                        {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    const APP_RECEIPT_EXPECTED_TRANSACTION_ID: &str = "0";
    const TRANSACTION_RECEIPT_EXPECTED_TRANSACTION_ID: &str = "33993399";

    #[test]
    fn test_xcode_app_receipt_extraction_with_no_transactions() {
        let receipt = fs::read_to_string("resources/xcode/xcode-app-receipt-empty")
            .expect("Failed to read file");
        let extracted_transaction_id = extract_transaction_id_from_app_receipt(&receipt);

        assert!(extracted_transaction_id.expect("Expect Result").is_none());
    }

    #[test]
    fn test_xcode_app_receipt_extraction_with_transactions() {
        let receipt = fs::read_to_string("resources/xcode/xcode-app-receipt-with-transaction")
            .expect("Failed to read file");
        let extracted_transaction_id = extract_transaction_id_from_app_receipt(&receipt);

        assert_eq!(
            Some(APP_RECEIPT_EXPECTED_TRANSACTION_ID),
            extracted_transaction_id.expect("REASON").as_deref()
        );
    }

    #[test]
    fn test_transaction_receipt_extraction() {
        let receipt = fs::read_to_string("resources/mock_signed_data/legacyTransaction")
            .expect("Failed to read file");
        let extracted_transaction_id = extract_transaction_id_from_transaction_receipt(&receipt);

        assert_eq!(
            Some(TRANSACTION_RECEIPT_EXPECTED_TRANSACTION_ID),
            extracted_transaction_id.expect("REASON").as_deref()
        );
    }
}
