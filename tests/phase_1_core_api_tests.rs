use app_store_server_library::primitives::billing_plan_type::BillingPlanType;
use app_store_server_library::primitives::jws_renewal_info_decoded_payload::JWSRenewalInfoDecodedPayload;
use app_store_server_library::primitives::jws_transaction_decoded_payload::JWSTransactionDecodedPayload;
use app_store_server_library::primitives::renewal_billing_plan_type::RenewalBillingPlanType;
use app_store_server_library::primitives::transaction_commitment_info::TransactionCommitmentInfo;
use std::fs;

#[test]
fn test_billing_plan_type_serialization() {
    let json = r#""BILLED_UPFRONT""#;
    let result: Result<BillingPlanType, _> = serde_json::from_str(json);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), BillingPlanType::BilledUpfront);
}

#[test]
fn test_billing_plan_type_monthly() {
    let json = r#""MONTHLY""#;
    let result: Result<BillingPlanType, _> = serde_json::from_str(json);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), BillingPlanType::Monthly);
}

#[test]
fn test_renewal_billing_plan_type_serialization() {
    let json = r#""MONTHLY""#;
    let result: Result<RenewalBillingPlanType, _> = serde_json::from_str(json);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), RenewalBillingPlanType::Monthly);
}

// These mirror Swift's testTransactionCommitmentInfoBillingPeriodNumberValidation,
// which accepts 1, 12, and nil, and rejects 0, 13, and -1. Note that Swift does NOT
// validate totalBillingPeriods at all, so neither do we.

fn commitment_with_billing_period(billing_period_number: Option<i32>) -> TransactionCommitmentInfo {
    TransactionCommitmentInfo {
        billing_period_number,
        commitment_expires_date: None,
        commitment_price: None,
        total_billing_periods: None,
    }
}

#[test]
fn test_commitment_info_accepts_both_range_boundaries() {
    assert!(commitment_with_billing_period(Some(1)).validate().is_ok());
    assert!(commitment_with_billing_period(Some(12)).validate().is_ok());
}

#[test]
fn test_commitment_info_accepts_absent_billing_period() {
    assert!(commitment_with_billing_period(None).validate().is_ok());
}

#[test]
fn test_commitment_info_rejects_values_outside_range() {
    for bad in [0, 13, -1] {
        assert!(
            commitment_with_billing_period(Some(bad)).validate().is_err(),
            "billingPeriodNumber {} should be rejected",
            bad
        );
    }
}

#[test]
fn test_commitment_info_does_not_validate_total_billing_periods() {
    // Apple's library imposes no bound here, so a large value must still be accepted.
    let info = TransactionCommitmentInfo {
        billing_period_number: Some(3),
        commitment_expires_date: None,
        commitment_price: Some(119880),
        total_billing_periods: Some(600),
    };
    assert!(info.validate().is_ok());
}

#[test]
fn test_jws_transaction_decoded_payload_with_commitment_info() {
    let fixture = fs::read_to_string("tests/resources/models/signedTransaction.json")
        .expect("Failed to read fixture");
    let result: Result<JWSTransactionDecodedPayload, _> = serde_json::from_str(&fixture);
    assert!(result.is_ok());

    let payload = result.unwrap();
    assert_eq!(payload.billing_plan_type, Some(BillingPlanType::Monthly));
    assert!(payload.commitment_info.is_some());

    let commitment = payload.commitment_info.unwrap();
    assert_eq!(commitment.billing_period_number, Some(3));
    assert_eq!(commitment.total_billing_periods, Some(12));
    assert_eq!(commitment.commitment_price, Some(119880));
    assert!(commitment.commitment_expires_date.is_some());
}

#[test]
fn test_jws_renewal_info_decoded_payload_with_commitment_info() {
    let fixture = fs::read_to_string("tests/resources/models/signedRenewalInfo.json")
        .expect("Failed to read fixture");
    let result: Result<JWSRenewalInfoDecodedPayload, _> = serde_json::from_str(&fixture);
    assert!(result.is_ok());

    let payload = result.unwrap();
    assert_eq!(
        payload.renewal_billing_plan_type,
        Some(RenewalBillingPlanType::Monthly)
    );
    assert!(payload.commitment_info.is_some());

    let commitment = payload.commitment_info.unwrap();
    assert_eq!(
        commitment.commitment_auto_renew_product_id,
        Some("com.example.product.commitment".to_string())
    );
    assert_eq!(
        commitment.commitment_renewal_billing_plan_type,
        Some(RenewalBillingPlanType::Monthly)
    );
    assert_eq!(commitment.commitment_renewal_price, Some(9990));
    assert!(commitment.commitment_renewal_date.is_some());
}

#[test]
fn test_jws_transaction_decoded_payload_backwards_compatible_without_commitment() {
    let json = r#"{
        "transactionId": "12345",
        "originalTransactionId": "orig123",
        "bundleId": "com.example",
        "productId": "product1",
        "purchaseDate": 1698148900000,
        "environment": "Production",
        "type": "Auto-Renewable Subscription"
    }"#;
    let result: Result<JWSTransactionDecodedPayload, _> = serde_json::from_str(json);
    assert!(result.is_ok());

    let payload = result.unwrap();
    assert!(payload.billing_plan_type.is_none());
    assert!(payload.commitment_info.is_none());
}