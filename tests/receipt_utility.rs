#![cfg(feature = "receipt-utility")]

use app_store_server_library::receipt_utility::{
    extract_transaction_id_from_app_receipt, extract_transaction_id_from_transaction_receipt,
};
use std::fs;

const APP_RECEIPT_EXPECTED_TRANSACTION_ID: &str = "0";
const TRANSACTION_RECEIPT_EXPECTED_TRANSACTION_ID: &str = "33993399";

#[test]
fn test_xcode_app_receipt_extraction_with_no_transactions() {
    let receipt = fs::read_to_string("tests/resources/xcode/xcode-app-receipt-empty").expect("Failed to read file");
    let extracted_transaction_id = extract_transaction_id_from_app_receipt(&receipt);

    assert!(extracted_transaction_id
        .expect("Expect Result")
        .is_none());
}

#[test]
fn test_xcode_app_receipt_extraction_with_transactions() {
    let receipt =
        fs::read_to_string("tests/resources/xcode/xcode-app-receipt-with-transaction").expect("Failed to read file");
    let extracted_transaction_id = extract_transaction_id_from_app_receipt(&receipt);

    assert_eq!(
        Some(APP_RECEIPT_EXPECTED_TRANSACTION_ID),
        extracted_transaction_id
            .expect("Expect Result")
            .as_deref()
    );
}

#[test]
fn test_transaction_receipt_extraction() {
    let receipt = fs::read_to_string("tests/resources/mock_signed_data/legacyTransaction").expect("Failed to read file");
    let extracted_transaction_id = extract_transaction_id_from_transaction_receipt(&receipt);

    assert_eq!(
        Some(TRANSACTION_RECEIPT_EXPECTED_TRANSACTION_ID),
        extracted_transaction_id
            .expect("Expect Result")
            .as_deref()
    );
}

#[test]
fn test_extract_transaction_id_from_app_receipt() {
    let receipt = fs::read_to_string("tests/resources/xcode/xcode-app-receipt-legacy").expect("Failed to read file");
    let extracted_transaction_id = extract_transaction_id_from_app_receipt(&receipt);
    assert_eq!(
        Some("2000000909538865"),
        extracted_transaction_id
            .expect("Expect Result")
            .as_deref()
    );
}
