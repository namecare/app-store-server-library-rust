use app_store_server_library::models::advanced_commerce_descriptors::AdvancedCommerceDescriptors;
use app_store_server_library::models::advanced_commerce_effective::AdvancedCommerceEffective;
use app_store_server_library::models::advanced_commerce_offer::AdvancedCommerceOffer;
use app_store_server_library::models::advanced_commerce_offer_period::AdvancedCommerceOfferPeriod;
use app_store_server_library::models::advanced_commerce_offer_reason::AdvancedCommerceOfferReason;
use app_store_server_library::models::advanced_commerce_one_time_charge_create_request::AdvancedCommerceOneTimeChargeCreateRequest;
use app_store_server_library::models::advanced_commerce_one_time_charge_item::AdvancedCommerceOneTimeChargeItem;
use app_store_server_library::models::advanced_commerce_period::AdvancedCommercePeriod;
use app_store_server_library::models::advanced_commerce_price_increase_info_status::AdvancedCommercePriceIncreaseInfoStatus;
use app_store_server_library::models::advanced_commerce_reason::AdvancedCommerceReason;
use app_store_server_library::models::advanced_commerce_refund_reason::AdvancedCommerceRefundReason;
use app_store_server_library::models::advanced_commerce_refund_type::AdvancedCommerceRefundType;
use app_store_server_library::models::advanced_commerce_renewal_info::AdvancedCommerceRenewalInfo;
use app_store_server_library::models::advanced_commerce_renewal_item::AdvancedCommerceRenewalItem;
use app_store_server_library::models::advanced_commerce_request_info::AdvancedCommerceRequestInfo;
use app_store_server_library::models::advanced_commerce_request_refund_item::AdvancedCommerceRequestRefundItem;
use app_store_server_library::models::advanced_commerce_request_refund_request::AdvancedCommerceRequestRefundRequest;
use app_store_server_library::models::advanced_commerce_request_refund_response::AdvancedCommerceRequestRefundResponse;
use app_store_server_library::models::advanced_commerce_subscription_cancel_request::AdvancedCommerceSubscriptionCancelRequest;
use app_store_server_library::models::advanced_commerce_subscription_cancel_response::AdvancedCommerceSubscriptionCancelResponse;
use app_store_server_library::models::advanced_commerce_subscription_change_metadata_descriptors::AdvancedCommerceSubscriptionChangeMetadataDescriptors;
use app_store_server_library::models::advanced_commerce_subscription_change_metadata_item::AdvancedCommerceSubscriptionChangeMetadataItem;
use app_store_server_library::models::advanced_commerce_subscription_change_metadata_request::AdvancedCommerceSubscriptionChangeMetadataRequest;
use app_store_server_library::models::advanced_commerce_subscription_change_metadata_response::AdvancedCommerceSubscriptionChangeMetadataResponse;
use app_store_server_library::models::advanced_commerce_subscription_create_item::AdvancedCommerceSubscriptionCreateItem;
use app_store_server_library::models::advanced_commerce_subscription_create_request::AdvancedCommerceSubscriptionCreateRequest;
use app_store_server_library::models::advanced_commerce_subscription_migrate_descriptors::AdvancedCommerceSubscriptionMigrateDescriptors;
use app_store_server_library::models::advanced_commerce_subscription_migrate_item::AdvancedCommerceSubscriptionMigrateItem;
use app_store_server_library::models::advanced_commerce_subscription_migrate_renewal_item::AdvancedCommerceSubscriptionMigrateRenewalItem;
use app_store_server_library::models::advanced_commerce_subscription_migrate_request::AdvancedCommerceSubscriptionMigrateRequest;
use app_store_server_library::models::advanced_commerce_subscription_migrate_response::AdvancedCommerceSubscriptionMigrateResponse;
use app_store_server_library::models::advanced_commerce_subscription_modify_add_item::AdvancedCommerceSubscriptionModifyAddItem;
use app_store_server_library::models::advanced_commerce_subscription_modify_change_item::AdvancedCommerceSubscriptionModifyChangeItem;
use app_store_server_library::models::advanced_commerce_subscription_modify_descriptors::AdvancedCommerceSubscriptionModifyDescriptors;
use app_store_server_library::models::advanced_commerce_subscription_modify_in_app_request::AdvancedCommerceSubscriptionModifyInAppRequest;
use app_store_server_library::models::advanced_commerce_subscription_modify_period_change::AdvancedCommerceSubscriptionModifyPeriodChange;
use app_store_server_library::models::advanced_commerce_subscription_modify_remove_item::AdvancedCommerceSubscriptionModifyRemoveItem;
use app_store_server_library::models::advanced_commerce_subscription_price_change_item::AdvancedCommerceSubscriptionPriceChangeItem;
use app_store_server_library::models::advanced_commerce_subscription_price_change_request::AdvancedCommerceSubscriptionPriceChangeRequest;
use app_store_server_library::models::advanced_commerce_subscription_price_change_response::AdvancedCommerceSubscriptionPriceChangeResponse;
use app_store_server_library::models::advanced_commerce_subscription_reactivate_in_app_request::AdvancedCommerceSubscriptionReactivateInAppRequest;
use app_store_server_library::models::advanced_commerce_subscription_reactivate_item::AdvancedCommerceSubscriptionReactivateItem;
use app_store_server_library::models::advanced_commerce_subscription_revoke_request::AdvancedCommerceSubscriptionRevokeRequest;
use app_store_server_library::models::advanced_commerce_subscription_revoke_response::AdvancedCommerceSubscriptionRevokeResponse;
use app_store_server_library::models::advanced_commerce_transaction_info::AdvancedCommerceTransactionInfo;
use app_store_server_library::models::advanced_commerce_transaction_item::AdvancedCommerceTransactionItem;
use app_store_server_library::models::billing_plan_type::BillingPlanType;
use app_store_server_library::models::helper_validation_utils::{
    validate_description, validate_display_name, validate_items, validate_period_count, validate_sku, ValidationError,
};
use app_store_server_library::models::renewal_billing_plan_type::RenewalBillingPlanType;
use app_store_server_library::models::transaction_commitment_info::TransactionCommitmentInfo;
use uuid::Uuid;

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!("tests/resources/models/{}", name))
        .unwrap_or_else(|e| panic!("failed to read fixture {}: {}", name, e))
}

fn assert_codable_round_trips<T>(value: &T)
where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let json = serde_json::to_string(value).expect("serialize");
    let parsed: T = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(
        &parsed, value,
        "value changed across a serialize/deserialize round trip"
    );
}

/// Asserts a raw wire string maps to the expected variant and back again.
fn assert_enum_raw_value<T>(raw: &str, expected: T)
where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let quoted = format!("\"{}\"", raw);
    let parsed: T =
        serde_json::from_str(&quoted).unwrap_or_else(|e| panic!("{} should deserialize to a variant: {}", raw, e));
    assert_eq!(parsed, expected, "{} mapped to the wrong variant", raw);
    assert_eq!(
        serde_json::to_string(&expected).expect("serialize"),
        quoted,
        "{:?} should serialize back to {}",
        expected,
        quoted
    );
    assert_codable_round_trips(&expected);
}

// ---------------------------------------------------------------------------
// Enum raw values
// ---------------------------------------------------------------------------

#[test]
fn advanced_commerce_period() {
    assert_enum_raw_value("P1W", AdvancedCommercePeriod::P1W);
    assert_enum_raw_value("P1M", AdvancedCommercePeriod::P1M);
    assert_enum_raw_value("P2M", AdvancedCommercePeriod::P2M);
    assert_enum_raw_value("P3M", AdvancedCommercePeriod::P3M);
    assert_enum_raw_value("P6M", AdvancedCommercePeriod::P6M);
    assert_enum_raw_value("P1Y", AdvancedCommercePeriod::P1Y);

    assert_eq!(
        serde_json::from_str::<AdvancedCommercePeriod>("\"INVALID\"").expect("decodes leniently"),
        AdvancedCommercePeriod::NotSupported("INVALID".to_string())
    );
}

#[test]
fn advanced_commerce_reason() {
    assert_enum_raw_value("UPGRADE", AdvancedCommerceReason::Upgrade);
    assert_enum_raw_value("DOWNGRADE", AdvancedCommerceReason::Downgrade);
    assert_enum_raw_value("APPLY_OFFER", AdvancedCommerceReason::ApplyOffer);

    assert_eq!(
        serde_json::from_str::<AdvancedCommerceReason>("\"INVALID\"").expect("decodes leniently"),
        AdvancedCommerceReason::NotSupported("INVALID".to_string())
    );
}

#[test]
fn advanced_commerce_refund_reason() {
    assert_enum_raw_value(
        "UNINTENDED_PURCHASE",
        AdvancedCommerceRefundReason::UnintendedPurchase,
    );
    assert_enum_raw_value(
        "FULFILLMENT_ISSUE",
        AdvancedCommerceRefundReason::FulfillmentIssue,
    );
    assert_enum_raw_value(
        "UNSATISFIED_WITH_PURCHASE",
        AdvancedCommerceRefundReason::UnsatisfiedWithPurchase,
    );
    assert_enum_raw_value("LEGAL", AdvancedCommerceRefundReason::Legal);
    assert_enum_raw_value("OTHER", AdvancedCommerceRefundReason::Other);
    assert_enum_raw_value(
        "MODIFY_ITEMS_REFUND",
        AdvancedCommerceRefundReason::ModifyItemsRefund,
    );
    assert_enum_raw_value(
        "SIMULATE_REFUND_DECLINE",
        AdvancedCommerceRefundReason::SimulateRefundDecline,
    );

    assert_eq!(
        serde_json::from_str::<AdvancedCommerceRefundReason>("\"INVALID\"").expect("decodes leniently"),
        AdvancedCommerceRefundReason::NotSupported("INVALID".to_string())
    );
}

#[test]
fn advanced_commerce_refund_type() {
    assert_enum_raw_value("FULL", AdvancedCommerceRefundType::Full);
    assert_enum_raw_value("PRORATED", AdvancedCommerceRefundType::Prorated);
    assert_enum_raw_value("CUSTOM", AdvancedCommerceRefundType::Custom);

    assert_eq!(
        serde_json::from_str::<AdvancedCommerceRefundType>("\"INVALID\"").expect("decodes leniently"),
        AdvancedCommerceRefundType::NotSupported("INVALID".to_string())
    );
}

#[test]
fn advanced_commerce_offer_period() {
    assert_enum_raw_value("P3D", AdvancedCommerceOfferPeriod::P3d);
    assert_enum_raw_value("P1W", AdvancedCommerceOfferPeriod::P1w);
    assert_enum_raw_value("P2W", AdvancedCommerceOfferPeriod::P2w);
    assert_enum_raw_value("P1M", AdvancedCommerceOfferPeriod::P1m);
    assert_enum_raw_value("P2M", AdvancedCommerceOfferPeriod::P2m);
    assert_enum_raw_value("P3M", AdvancedCommerceOfferPeriod::P3m);
    assert_enum_raw_value("P6M", AdvancedCommerceOfferPeriod::P6m);
    assert_enum_raw_value("P9M", AdvancedCommerceOfferPeriod::P9m);
    assert_enum_raw_value("P1Y", AdvancedCommerceOfferPeriod::P1y);

    assert_eq!(
        serde_json::from_str::<AdvancedCommerceOfferPeriod>("\"INVALID\"").expect("decodes leniently"),
        AdvancedCommerceOfferPeriod::NotSupported("INVALID".to_string())
    );
}

#[test]
fn advanced_commerce_offer_reason() {
    assert_enum_raw_value("ACQUISITION", AdvancedCommerceOfferReason::Acquisition);
    assert_enum_raw_value("WIN_BACK", AdvancedCommerceOfferReason::WinBack);
    assert_enum_raw_value("RETENTION", AdvancedCommerceOfferReason::Retention);

    assert_eq!(
        serde_json::from_str::<AdvancedCommerceOfferReason>("\"INVALID\"").expect("decodes leniently"),
        AdvancedCommerceOfferReason::NotSupported("INVALID".to_string())
    );
}

#[test]
fn advanced_commerce_effective() {
    assert_enum_raw_value("IMMEDIATELY", AdvancedCommerceEffective::Immediately);
    assert_enum_raw_value("NEXT_BILL_CYCLE", AdvancedCommerceEffective::NextBillCycle);

    assert_eq!(
        serde_json::from_str::<AdvancedCommerceEffective>("\"INVALID\"").expect("decodes leniently"),
        AdvancedCommerceEffective::NotSupported("INVALID".to_string())
    );
}

// ---------------------------------------------------------------------------
// Validation utilities
// ---------------------------------------------------------------------------

#[test]
fn validation_utils_description() {
    assert_eq!(
        validate_description("a valid description").unwrap(),
        "a valid description"
    );

    // 45 characters is the documented maximum.
    let at_limit = "a".repeat(45);
    assert_eq!(validate_description(&at_limit).unwrap(), at_limit);

    let too_long = "a".repeat(46);
    assert!(validate_description(&too_long).is_err());
}

#[test]
fn validation_utils_display_name() {
    assert_eq!(
        validate_display_name("a display name").unwrap(),
        "a display name"
    );

    // 30 characters is the documented maximum.
    let at_limit = "a".repeat(30);
    assert_eq!(validate_display_name(&at_limit).unwrap(), at_limit);

    let too_long = "a".repeat(31);
    assert!(validate_display_name(&too_long).is_err());
}

#[test]
fn validation_utils_sku() {
    assert_eq!(validate_sku("a.valid.sku").unwrap(), "a.valid.sku");

    // 128 characters is the documented maximum.
    let at_limit = "a".repeat(128);
    assert_eq!(validate_sku(&at_limit).unwrap(), at_limit);

    let too_long = "a".repeat(129);
    assert!(validate_sku(&too_long).is_err());
}

#[test]
fn validation_utils_period_count() {
    assert_eq!(validate_period_count(1).unwrap(), 1);
    assert_eq!(validate_period_count(6).unwrap(), 6);
    assert_eq!(validate_period_count(12).unwrap(), 12);

    for bad in [0, 13] {
        assert!(
            matches!(
                validate_period_count(bad),
                Err(ValidationError::InvalidPeriodCount(_))
            ),
            "period count {} should be rejected",
            bad
        );
    }
}

#[test]
fn validation_utils_items() {
    let valid_list = vec![AdvancedCommerceOneTimeChargeItem::new(
        "sku1".to_string(),
        "desc".to_string(),
        "name".to_string(),
        1000,
    )];
    assert_eq!(validate_items(valid_list.clone()).unwrap(), valid_list);

    let empty_list: Vec<AdvancedCommerceOneTimeChargeItem> = vec![];
    assert!(validate_items(empty_list).is_err());
}

// ---------------------------------------------------------------------------
// JSON deserialization
// ---------------------------------------------------------------------------

#[test]
fn advanced_commerce_descriptors() {
    let parsed: AdvancedCommerceDescriptors =
        serde_json::from_str(&fixture("advancedCommerceDescriptors.json")).unwrap();
    assert_eq!(parsed.description, "description");
    assert_eq!(parsed.display_name, "display name");

    assert_codable_round_trips(&parsed);
}

#[test]
fn advanced_commerce_one_time_charge_item() {
    let item: AdvancedCommerceOneTimeChargeItem =
        serde_json::from_str(&fixture("advancedCommerceOneTimeChargeItem.json")).unwrap();
    assert_eq!(item.description, "description");
    assert_eq!(item.display_name, "display name");
    assert_eq!(item.sku, "sku");
    assert_eq!(item.price, 15000);

    assert_codable_round_trips(&item);
}

#[test]
fn advanced_commerce_subscription_create_item() {
    let item: AdvancedCommerceSubscriptionCreateItem =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionCreateItem.json")).unwrap();
    assert_eq!(item.description, "description");
    assert_eq!(item.display_name, "display name");
    assert_eq!(item.sku, "sku");
    assert_eq!(item.price, 20000);

    assert_codable_round_trips(&item);
}

#[test]
fn advanced_commerce_request_refund_item() {
    let item: AdvancedCommerceRequestRefundItem =
        serde_json::from_str(&fixture("advancedCommerceRequestRefundItem.json")).unwrap();
    assert_eq!(item.sku, "sku");
    assert_eq!(item.refund_reason, AdvancedCommerceRefundReason::Legal);
    assert_eq!(item.refund_type, AdvancedCommerceRefundType::Full);
    assert!(item.revoke);
    assert_eq!(item.refund_amount, Some(5000));

    assert_codable_round_trips(&item);
}

#[test]
fn advanced_commerce_offer() {
    let offer: AdvancedCommerceOffer = serde_json::from_str(&fixture("advancedCommerceOffer.json")).unwrap();
    assert_eq!(offer.period, AdvancedCommerceOfferPeriod::P1w);
    assert_eq!(offer.period_count, 3);
    assert_eq!(offer.price, 5000);
    assert_eq!(offer.reason, AdvancedCommerceOfferReason::WinBack);

    assert_codable_round_trips(&offer);
}

#[test]
fn advanced_commerce_one_time_charge_create_request() {
    let request: AdvancedCommerceOneTimeChargeCreateRequest =
        serde_json::from_str(&fixture("advancedCommerceOneTimeChargeCreateRequest.json")).unwrap();
    assert_eq!(request.currency, "USD");
    assert_eq!(request.item.sku, "sku");
    assert_eq!(request.tax_code, "taxCode");
    assert_eq!(
        request
            .request_info
            .request_reference_id,
        Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap()
    );
    assert_eq!(request.storefront.as_deref(), Some("USA"));

    assert_codable_round_trips(&request);
}

#[test]
fn advanced_commerce_subscription_create_request() {
    let request: AdvancedCommerceSubscriptionCreateRequest =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionCreateRequest.json")).unwrap();
    assert_eq!(request.currency, "USD");
    assert_eq!(request.descriptors.description, "description");
    assert_eq!(request.items.len(), 2);
    assert_eq!(request.period, AdvancedCommercePeriod::P1M);
    assert_eq!(request.tax_code, "taxCode");
    assert_eq!(request.storefront.as_deref(), Some("USA"));
    assert_eq!(
        request
            .previous_transaction_id
            .as_deref(),
        Some("transactionId")
    );

    assert_codable_round_trips(&request);
}

#[test]
fn advanced_commerce_request_refund_request() {
    let request: AdvancedCommerceRequestRefundRequest =
        serde_json::from_str(&fixture("advancedCommerceRequestRefundRequest.json")).unwrap();
    assert_eq!(request.items.len(), 2);
    assert!(request.refund_risking_preference);
    assert_eq!(
        request
            .request_info
            .request_reference_id,
        Uuid::parse_str("550e8400-e29b-41d4-a716-446655440002").unwrap()
    );
    assert_eq!(request.currency.as_deref(), Some("USD"));
    assert_eq!(request.storefront.as_deref(), Some("USA"));

    assert_codable_round_trips(&request);
}

#[test]
fn advanced_commerce_subscription_cancel_request() {
    let request: AdvancedCommerceSubscriptionCancelRequest =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionCancelRequest.json")).unwrap();
    assert_eq!(
        request
            .request_info
            .request_reference_id,
        Uuid::parse_str("550e8400-e29b-41d4-a716-446655440003").unwrap()
    );
    assert_eq!(request.storefront.as_deref(), Some("USA"));

    assert_codable_round_trips(&request);
}

#[test]
fn advanced_commerce_subscription_revoke_request() {
    let request: AdvancedCommerceSubscriptionRevokeRequest =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionRevokeRequest.json")).unwrap();
    assert_eq!(
        request
            .request_info
            .request_reference_id,
        Uuid::parse_str("550e8400-e29b-41d4-a716-446655440004").unwrap()
    );
    assert!(request.refund_risking_preference);
    assert_eq!(request.refund_reason, AdvancedCommerceRefundReason::Legal);
    assert_eq!(request.refund_type, AdvancedCommerceRefundType::Full);
    assert_eq!(request.storefront.as_deref(), Some("USA"));

    assert_codable_round_trips(&request);
}

#[test]
fn advanced_commerce_subscription_price_change_request() {
    let request: AdvancedCommerceSubscriptionPriceChangeRequest = serde_json::from_str(&fixture(
        "advancedCommerceSubscriptionPriceChangeRequest.json",
    ))
    .unwrap();
    assert_eq!(request.items.len(), 1);
    assert_eq!(
        request
            .request_info
            .request_reference_id,
        Uuid::parse_str("550e8400-e29b-41d4-a716-446655440005").unwrap()
    );
    assert_eq!(request.currency.as_deref(), Some("USD"));

    assert_codable_round_trips(&request);
}

#[test]
fn advanced_commerce_request_refund_response() {
    let response: AdvancedCommerceRequestRefundResponse =
        serde_json::from_str(&fixture("advancedCommerceRequestRefundResponse.json")).unwrap();
    assert_eq!(response.signed_renewal_info, None);
    assert_eq!(
        response.signed_transaction_info,
        "signed_transaction_info_value"
    );

    assert_codable_round_trips(&response);
}

#[test]
fn advanced_commerce_subscription_cancel_response() {
    let response: AdvancedCommerceSubscriptionCancelResponse =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionCancelResponse.json")).unwrap();
    assert_eq!(response.signed_renewal_info, "signed_renewal_info");
    assert_eq!(response.signed_transaction_info, "signed_transaction_info");

    assert_codable_round_trips(&response);
}

#[test]
fn advanced_commerce_subscription_revoke_response() {
    let response: AdvancedCommerceSubscriptionRevokeResponse =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionRevokeResponse.json")).unwrap();
    assert_eq!(response.signed_renewal_info, "signed_renewal_info");
    assert_eq!(response.signed_transaction_info, "signed_transaction_info");

    assert_codable_round_trips(&response);
}

#[test]
fn advanced_commerce_subscription_price_change_response() {
    let response: AdvancedCommerceSubscriptionPriceChangeResponse = serde_json::from_str(&fixture(
        "advancedCommerceSubscriptionPriceChangeResponse.json",
    ))
    .unwrap();
    assert_eq!(response.signed_renewal_info, "signed_renewal_info");
    assert_eq!(response.signed_transaction_info, "signed_transaction_info");

    assert_codable_round_trips(&response);
}

#[test]
fn advanced_commerce_subscription_change_metadata_response() {
    let response: AdvancedCommerceSubscriptionChangeMetadataResponse = serde_json::from_str(&fixture(
        "advancedCommerceSubscriptionChangeMetadataResponse.json",
    ))
    .unwrap();
    assert_eq!(response.signed_renewal_info, "signed_renewal_info");
    assert_eq!(response.signed_transaction_info, "signed_transaction_info");

    assert_codable_round_trips(&response);
}

#[test]
fn advanced_commerce_subscription_migrate_request() {
    let request: AdvancedCommerceSubscriptionMigrateRequest =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionMigrateRequest.json")).unwrap();
    assert_eq!(request.descriptors.description, "description");
    assert_eq!(request.descriptors.display_name, "display name");
    assert_eq!(request.items.len(), 1);
    assert_eq!(request.tax_code, "taxCode");
    assert_eq!(request.target_product_id, "targetProductId");

    assert_codable_round_trips(&request);
}

#[test]
fn advanced_commerce_subscription_modify_in_app_request() {
    let request: AdvancedCommerceSubscriptionModifyInAppRequest = serde_json::from_str(&fixture(
        "advancedCommerceSubscriptionModifyInAppRequest.json",
    ))
    .unwrap();
    assert_eq!(request.currency.as_deref(), Some("USD"));
    assert!(request.descriptors.is_some());
    assert_eq!(request.tax_code.as_deref(), Some("taxCode"));
    assert_eq!(request.transaction_id, "transactionId");
    assert!(request.retain_billing_cycle);

    assert_codable_round_trips(&request);
}

#[test]
fn advanced_commerce_subscription_reactivate_in_app_request() {
    let request: AdvancedCommerceSubscriptionReactivateInAppRequest = serde_json::from_str(&fixture(
        "advancedCommerceSubscriptionReactivateInAppRequest.json",
    ))
    .unwrap();
    assert!(request.items.is_some());
    assert_eq!(request.transaction_id, "transactionId");

    assert_codable_round_trips(&request);
}

#[test]
fn advanced_commerce_subscription_change_metadata_request() {
    let request: AdvancedCommerceSubscriptionChangeMetadataRequest = serde_json::from_str(&fixture(
        "advancedCommerceSubscriptionChangeMetadataRequest.json",
    ))
    .unwrap();
    assert!(request.items.is_some());
    assert_eq!(
        request
            .request_info
            .request_reference_id,
        Uuid::parse_str("550e8400-e29b-41d4-a716-446655440009").unwrap()
    );

    assert_codable_round_trips(&request);
}

#[test]
fn advanced_commerce_subscription_migrate_descriptors() {
    let descriptors: AdvancedCommerceSubscriptionMigrateDescriptors = serde_json::from_str(&fixture(
        "advancedCommerceSubscriptionMigrateDescriptors.json",
    ))
    .unwrap();
    assert_eq!(descriptors.description, "description");
    assert_eq!(descriptors.display_name, "displayName");

    assert_codable_round_trips(&descriptors);
}

#[test]
fn advanced_commerce_subscription_modify_descriptors() {
    let descriptors: AdvancedCommerceSubscriptionModifyDescriptors = serde_json::from_str(&fixture(
        "advancedCommerceSubscriptionModifyDescriptors.json",
    ))
    .unwrap();
    assert_eq!(descriptors.description.as_deref(), Some("description"));
    assert_eq!(descriptors.display_name.as_deref(), Some("displayName"));
    assert_eq!(
        descriptors.effective,
        AdvancedCommerceEffective::Immediately
    );

    assert_codable_round_trips(&descriptors);
}

#[test]
fn advanced_commerce_subscription_change_metadata_descriptors() {
    let descriptors: AdvancedCommerceSubscriptionChangeMetadataDescriptors = serde_json::from_str(&fixture(
        "advancedCommerceSubscriptionChangeMetadataDescriptors.json",
    ))
    .unwrap();
    assert_eq!(descriptors.description.as_deref(), Some("description"));
    assert_eq!(descriptors.display_name.as_deref(), Some("displayName"));
    assert_eq!(
        descriptors.effective,
        AdvancedCommerceEffective::Immediately
    );

    assert_codable_round_trips(&descriptors);
}

#[test]
fn advanced_commerce_subscription_migrate_item() {
    let item: AdvancedCommerceSubscriptionMigrateItem =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionMigrateItem.json")).unwrap();
    assert_eq!(item.description, "description");
    assert_eq!(item.display_name, "displayName");
    assert_eq!(item.sku, "sku");

    assert_codable_round_trips(&item);
}

#[test]
fn advanced_commerce_subscription_migrate_renewal_item() {
    let item: AdvancedCommerceSubscriptionMigrateRenewalItem = serde_json::from_str(&fixture(
        "advancedCommerceSubscriptionMigrateRenewalItem.json",
    ))
    .unwrap();
    assert_eq!(item.description, "description");
    assert_eq!(item.display_name, "displayName");
    assert_eq!(item.sku, "sku");

    assert_codable_round_trips(&item);
}

#[test]
fn advanced_commerce_subscription_modify_add_item() {
    let item: AdvancedCommerceSubscriptionModifyAddItem =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionModifyAddItem.json")).unwrap();
    assert_eq!(item.description, "description");
    assert_eq!(item.display_name, "displayName");
    assert_eq!(item.sku, "sku");
    assert_eq!(item.price, 12000);

    assert_codable_round_trips(&item);
}

#[test]
fn advanced_commerce_subscription_modify_change_item() {
    let item: AdvancedCommerceSubscriptionModifyChangeItem = serde_json::from_str(&fixture(
        "advancedCommerceSubscriptionModifyChangeItem.json",
    ))
    .unwrap();
    assert_eq!(item.description, "description");
    assert_eq!(item.display_name, "displayName");
    assert_eq!(item.sku, "sku");
    assert_eq!(item.current_sku, "currentSku");
    assert_eq!(item.price, 13000);
    assert_eq!(item.effective, AdvancedCommerceEffective::Immediately);
    assert_eq!(item.reason, AdvancedCommerceReason::Upgrade);

    assert_codable_round_trips(&item);
}

#[test]
fn advanced_commerce_subscription_modify_remove_item() {
    let item: AdvancedCommerceSubscriptionModifyRemoveItem = serde_json::from_str(&fixture(
        "advancedCommerceSubscriptionModifyRemoveItem.json",
    ))
    .unwrap();
    assert_eq!(item.sku, "sku");

    assert_codable_round_trips(&item);
}

#[test]
fn advanced_commerce_subscription_modify_period_change() {
    let period_change: AdvancedCommerceSubscriptionModifyPeriodChange = serde_json::from_str(&fixture(
        "advancedCommerceSubscriptionModifyPeriodChange.json",
    ))
    .unwrap();
    assert_eq!(period_change.period, AdvancedCommercePeriod::P3M);
    assert_eq!(
        period_change.effective,
        AdvancedCommerceEffective::Immediately
    );

    assert_codable_round_trips(&period_change);
}

#[test]
fn advanced_commerce_subscription_price_change_item() {
    let item: AdvancedCommerceSubscriptionPriceChangeItem =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionPriceChangeItem.json")).unwrap();
    assert_eq!(item.sku, "sku");
    assert_eq!(item.price, 16000);
    assert_eq!(
        item.dependent_skus
            .as_ref()
            .and_then(|v| v.first()),
        Some(&"dependentSKU".to_string())
    );

    assert_codable_round_trips(&item);
}

#[test]
fn advanced_commerce_subscription_price_change_item_dependent_sku_validation() {
    let valid_sku = "A".repeat(128);
    let too_long_sku = "A".repeat(129);

    let item = AdvancedCommerceSubscriptionPriceChangeItem::new("sku".to_string(), 1000, Some(vec![valid_sku.clone()]))
        .unwrap();
    assert_eq!(
        item.dependent_skus
            .as_ref()
            .and_then(|v| v.first()),
        Some(&valid_sku)
    );

    assert!(matches!(
        AdvancedCommerceSubscriptionPriceChangeItem::new("sku".to_string(), 1000, Some(vec![too_long_sku])),
        Err(ValidationError::SkuTooLong(129))
    ));

    let nil_list_item = AdvancedCommerceSubscriptionPriceChangeItem::new("sku".to_string(), 1000, None).unwrap();
    assert_eq!(nil_list_item.dependent_skus, None);
}

#[test]
fn advanced_commerce_subscription_reactivate_item() {
    let item: AdvancedCommerceSubscriptionReactivateItem =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionReactivateItem.json")).unwrap();
    assert_eq!(item.sku, "sku");

    assert_codable_round_trips(&item);
}

#[test]
fn advanced_commerce_subscription_change_metadata_item() {
    let item: AdvancedCommerceSubscriptionChangeMetadataItem = serde_json::from_str(&fixture(
        "advancedCommerceSubscriptionChangeMetadataItem.json",
    ))
    .unwrap();
    assert_eq!(item.description.as_deref(), Some("description"));
    assert_eq!(item.display_name.as_deref(), Some("displayName"));
    assert_eq!(item.sku.as_deref(), Some("sku"));
    assert_eq!(item.current_sku, "currentSku");
    assert_eq!(item.effective, AdvancedCommerceEffective::NextBillCycle);

    assert_codable_round_trips(&item);
}

#[test]
fn advanced_commerce_request_info() {
    let request_info: AdvancedCommerceRequestInfo =
        serde_json::from_str(&fixture("advancedCommerceRequestInfo.json")).unwrap();
    assert_eq!(
        request_info.request_reference_id,
        Uuid::parse_str("550e8400-e29b-41d4-a716-446655440010").unwrap()
    );
    assert_eq!(
        request_info.app_account_token,
        Some(Uuid::parse_str("660e8400-e29b-41d4-a716-446655440011").unwrap())
    );
    assert_eq!(
        request_info
            .consistency_token
            .as_deref(),
        Some("consistency_token_value")
    );

    assert_codable_round_trips(&request_info);
}

#[test]
fn advanced_commerce_subscription_migrate_response() {
    let response: AdvancedCommerceSubscriptionMigrateResponse =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionMigrateResponse.json")).unwrap();
    assert_eq!(response.signed_renewal_info, "signed_renewal_info_value");
    assert_eq!(
        response.signed_transaction_info,
        "signed_transaction_info_value"
    );

    assert_codable_round_trips(&response);
}

// ---------------------------------------------------------------------------
// Operation / version defaults
//
// Swift models `operation` and `version` as computed constants that are encoded
// but never decoded, so its fixtures omit both keys. The Rust request types
// default them on decode and must still put them on the wire.
// ---------------------------------------------------------------------------

#[test]
fn one_time_charge_create_request_deserialization_sets_operation_and_version() {
    let request: AdvancedCommerceOneTimeChargeCreateRequest =
        serde_json::from_str(&fixture("advancedCommerceOneTimeChargeCreateRequest.json")).unwrap();

    let node = serde_json::to_value(&request).unwrap();
    assert_eq!(node["operation"], "CREATE_ONE_TIME_CHARGE");
    assert_eq!(node["version"], "1");
}

#[test]
fn subscription_create_request_deserialization_sets_operation_and_version() {
    let request: AdvancedCommerceSubscriptionCreateRequest =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionCreateRequest.json")).unwrap();

    let node = serde_json::to_value(&request).unwrap();
    assert_eq!(node["operation"], "CREATE_SUBSCRIPTION");
    assert_eq!(node["version"], "1");
}

#[test]
fn subscription_modify_in_app_request_deserialization_sets_operation_and_version() {
    let request: AdvancedCommerceSubscriptionModifyInAppRequest = serde_json::from_str(&fixture(
        "advancedCommerceSubscriptionModifyInAppRequest.json",
    ))
    .unwrap();

    let node = serde_json::to_value(&request).unwrap();
    assert_eq!(node["operation"], "MODIFY_SUBSCRIPTION");
    assert_eq!(node["version"], "1");
}

#[test]
fn subscription_reactivate_in_app_request_deserialization_sets_operation_and_version() {
    let request: AdvancedCommerceSubscriptionReactivateInAppRequest = serde_json::from_str(&fixture(
        "advancedCommerceSubscriptionReactivateInAppRequest.json",
    ))
    .unwrap();

    let node = serde_json::to_value(&request).unwrap();
    assert_eq!(node["operation"], "REACTIVATE_SUBSCRIPTION");
    assert_eq!(node["version"], "1");
}

#[test]
fn advanced_commerce_price_increase_info_status() {
    assert_enum_raw_value(
        "SCHEDULED",
        AdvancedCommercePriceIncreaseInfoStatus::Scheduled,
    );
    assert_enum_raw_value("PENDING", AdvancedCommercePriceIncreaseInfoStatus::Pending);
    assert_enum_raw_value(
        "ACCEPTED",
        AdvancedCommercePriceIncreaseInfoStatus::Accepted,
    );

    assert_eq!(
        serde_json::from_str::<AdvancedCommercePriceIncreaseInfoStatus>("\"INVALID\"").expect("decodes leniently"),
        AdvancedCommercePriceIncreaseInfoStatus::NotSupported("INVALID".to_string())
    );
}

// ---------------------------------------------------------------------------
// Billing plan types / commitment info
// ---------------------------------------------------------------------------

#[test]
fn billing_plan_type() {
    assert_enum_raw_value("BILLED_UPFRONT", BillingPlanType::BilledUpfront);
    assert_enum_raw_value("MONTHLY", BillingPlanType::Monthly);

    assert_eq!(
        serde_json::from_str::<BillingPlanType>("\"INVALID\"").expect("decodes leniently"),
        BillingPlanType::NotSupported("INVALID".to_string())
    );
}

#[test]
fn renewal_billing_plan_type() {
    assert_enum_raw_value("BILLED_UPFRONT", RenewalBillingPlanType::BilledUpfront);
    assert_enum_raw_value("MONTHLY", RenewalBillingPlanType::Monthly);

    assert_eq!(
        serde_json::from_str::<RenewalBillingPlanType>("\"INVALID\"").expect("decodes leniently"),
        RenewalBillingPlanType::NotSupported("INVALID".to_string())
    );
}

fn commitment_with_billing_period(billing_period_number: Option<i32>) -> TransactionCommitmentInfo {
    TransactionCommitmentInfo {
        billing_period_number,
        commitment_expires_date: None,
        commitment_price: None,
        total_billing_periods: None,
    }
}

#[test]
fn transaction_commitment_info_billing_period_number_validation() {
    assert!(commitment_with_billing_period(Some(1))
        .validate()
        .is_ok());
    assert!(commitment_with_billing_period(Some(12))
        .validate()
        .is_ok());
    assert!(commitment_with_billing_period(None)
        .validate()
        .is_ok());

    for bad in [0, 13, -1] {
        assert!(
            commitment_with_billing_period(Some(bad))
                .validate()
                .is_err(),
            "billingPeriodNumber {} should be rejected",
            bad
        );
    }
}

// ---------------------------------------------------------------------------
// Rust-only
//
// Everything above mirrors AdvancedCommerceModelsTests.swift test for test.
// These cover shapes the Swift suite does not.
// ---------------------------------------------------------------------------

#[test]
fn advanced_commerce_period_rejects_snake_cased_codes() {
    // SCREAMING_SNAKE_CASE would emit "P1_M"; it must not resolve to P1M.
    assert_eq!(
        serde_json::from_str::<AdvancedCommercePeriod>("\"P1_M\"").expect("decodes leniently"),
        AdvancedCommercePeriod::NotSupported("P1_M".to_string())
    );
}

#[test]
fn commitment_info_does_not_validate_total_billing_periods() {
    // Apple's library imposes no bound here, so a large value must still be accepted.
    let info = TransactionCommitmentInfo {
        billing_period_number: Some(3),
        commitment_expires_date: None,
        commitment_price: Some(119880),
        total_billing_periods: Some(600),
    };
    assert!(info.validate().is_ok());
}

// Apple documents every property of advancedCommerceTransactionInfo,
// advancedCommerceTransactionItem, advancedCommerceRenewalInfo and
// advancedCommerceRenewalItem as optional. The full signedTransaction.json and
// signedRenewalInfo.json fixtures cover the populated shape; these pin the sparse
// ones: a line item bought without an offer and never refunded or revoked, an
// item carrying an offer, and an info object with no properties at all.

#[test]
fn advanced_commerce_transaction_item_without_offer_refunds_or_revocation() {
    let item: AdvancedCommerceTransactionItem =
        serde_json::from_str(&fixture("advancedCommerceTransactionItem.json")).unwrap();
    assert_eq!(item.sku.as_deref(), Some("com.example.sku.premium"));
    assert_eq!(item.description.as_deref(), Some("Premium feature"));
    assert_eq!(item.display_name.as_deref(), Some("Premium Feature"));
    assert_eq!(item.price, Some(9990));
    assert_eq!(item.offer, None);
    assert_eq!(item.refunds, None);
    assert_eq!(item.revocation_date, None);

    assert_codable_round_trips(&item);
}

#[test]
fn advanced_commerce_transaction_item_with_offer() {
    let item: AdvancedCommerceTransactionItem =
        serde_json::from_str(&fixture("advancedCommerceTransactionItemWithOffer.json")).unwrap();
    assert_eq!(
        item.offer,
        Some(AdvancedCommerceOffer::new(
            AdvancedCommerceOfferPeriod::P1m,
            1,
            1000,
            AdvancedCommerceOfferReason::Acquisition,
        ))
    );
    assert_eq!(item.price, Some(9990));
    assert_eq!(item.refunds, None);
    assert_eq!(item.revocation_date, None);

    assert_codable_round_trips(&item);
}

#[test]
fn advanced_commerce_renewal_item_with_offer() {
    let item: AdvancedCommerceRenewalItem = serde_json::from_str(&fixture("advancedCommerceRenewalItem.json")).unwrap();
    assert_eq!(item.sku.as_deref(), Some("com.example.sku.premium"));
    assert_eq!(
        item.offer,
        Some(AdvancedCommerceOffer::new(
            AdvancedCommerceOfferPeriod::P1w,
            3,
            5000,
            AdvancedCommerceOfferReason::WinBack,
        ))
    );
    assert_eq!(item.price, Some(9990));
    assert_eq!(item.price_increase_info, None);

    assert_codable_round_trips(&item);
}

#[test]
fn advanced_commerce_info_without_properties() {
    let transaction_info: AdvancedCommerceTransactionInfo = serde_json::from_str("{}").unwrap();
    assert_eq!(transaction_info.items, None);
    assert_eq!(transaction_info.period, None);
    assert_codable_round_trips(&transaction_info);

    let renewal_info: AdvancedCommerceRenewalInfo = serde_json::from_str("{}").unwrap();
    assert_eq!(renewal_info.items, None);
    assert_eq!(renewal_info.period, None);
    assert_codable_round_trips(&renewal_info);
}
