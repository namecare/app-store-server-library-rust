//! Advanced Commerce model tests, ported from Apple's Swift library.
//!
//! Source of truth:
//! `app-store-server-library-swift/Tests/AppStoreServerLibraryTests/AdvancedCommerceModelsTests.swift`
//!
//! Swift test names map to snake_case here so the correspondence stays auditable.
//! `testBillingPlanType`, `testRenewalBillingPlanType`, and
//! `testTransactionCommitmentInfoBillingPeriodNumberValidation` live in
//! `tests/phase_1_core_api_tests.rs` instead, and are not duplicated here.

use app_store_server_library::primitives::advanced_commerce::descriptors::Descriptors;
use app_store_server_library::primitives::advanced_commerce::effective::Effective;
use app_store_server_library::primitives::advanced_commerce::in_app_request_operation::InAppRequestOperation;
use app_store_server_library::primitives::advanced_commerce::in_app_request_version::InAppRequestVersion;
use app_store_server_library::primitives::advanced_commerce::offer::Offer;
use app_store_server_library::primitives::advanced_commerce::offer_period::OfferPeriod;
use app_store_server_library::primitives::advanced_commerce::offer_reason::OfferReason;
use app_store_server_library::primitives::advanced_commerce::one_time_charge_create_request::OneTimeChargeCreateRequest;
use app_store_server_library::primitives::advanced_commerce::one_time_charge_item::OneTimeChargeItem;
use app_store_server_library::primitives::advanced_commerce::period::Period;
use app_store_server_library::primitives::advanced_commerce::reason::Reason;
use app_store_server_library::primitives::advanced_commerce::refund_reason::RefundReason;
use app_store_server_library::primitives::advanced_commerce::refund_type::RefundType;
use app_store_server_library::primitives::advanced_commerce::request_info::RequestInfo;
use app_store_server_library::primitives::advanced_commerce::request_refund_item::RequestRefundItem;
use app_store_server_library::primitives::advanced_commerce::request_refund_request::RequestRefundRequest;
use app_store_server_library::primitives::advanced_commerce::request_refund_response::RequestRefundResponse;
use app_store_server_library::primitives::advanced_commerce::subscription_cancel_request::SubscriptionCancelRequest;
use app_store_server_library::primitives::advanced_commerce::subscription_cancel_response::SubscriptionCancelResponse;
use app_store_server_library::primitives::advanced_commerce::subscription_change_metadata_descriptors::SubscriptionChangeMetadataDescriptors;
use app_store_server_library::primitives::advanced_commerce::subscription_change_metadata_item::SubscriptionChangeMetadataItem;
use app_store_server_library::primitives::advanced_commerce::subscription_change_metadata_request::SubscriptionChangeMetadataRequest;
use app_store_server_library::primitives::advanced_commerce::subscription_change_metadata_response::SubscriptionChangeMetadataResponse;
use app_store_server_library::primitives::advanced_commerce::subscription_create_item::SubscriptionCreateItem;
use app_store_server_library::primitives::advanced_commerce::subscription_create_request::SubscriptionCreateRequest;
use app_store_server_library::primitives::advanced_commerce::subscription_migrate_descriptors::SubscriptionMigrateDescriptors;
use app_store_server_library::primitives::advanced_commerce::subscription_migrate_item::SubscriptionMigrateItem;
use app_store_server_library::primitives::advanced_commerce::subscription_migrate_renewal_item::SubscriptionMigrateRenewalItem;
use app_store_server_library::primitives::advanced_commerce::subscription_migrate_request::SubscriptionMigrateRequest;
use app_store_server_library::primitives::advanced_commerce::subscription_migrate_response::SubscriptionMigrateResponse;
use app_store_server_library::primitives::advanced_commerce::subscription_modify_add_item::SubscriptionModifyAddItem;
use app_store_server_library::primitives::advanced_commerce::subscription_modify_change_item::SubscriptionModifyChangeItem;
use app_store_server_library::primitives::advanced_commerce::subscription_modify_descriptors::SubscriptionModifyDescriptors;
use app_store_server_library::primitives::advanced_commerce::subscription_modify_in_app_request::SubscriptionModifyInAppRequest;
use app_store_server_library::primitives::advanced_commerce::subscription_modify_period_change::SubscriptionModifyPeriodChange;
use app_store_server_library::primitives::advanced_commerce::subscription_modify_remove_item::SubscriptionModifyRemoveItem;
use app_store_server_library::primitives::advanced_commerce::subscription_price_change_item::SubscriptionPriceChangeItem;
use app_store_server_library::primitives::advanced_commerce::subscription_price_change_request::SubscriptionPriceChangeRequest;
use app_store_server_library::primitives::advanced_commerce::subscription_price_change_response::SubscriptionPriceChangeResponse;
use app_store_server_library::primitives::advanced_commerce::subscription_reactivate_in_app_request::SubscriptionReactivateInAppRequest;
use app_store_server_library::primitives::advanced_commerce::subscription_reactivate_item::SubscriptionReactivateItem;
use app_store_server_library::primitives::advanced_commerce::subscription_revoke_request::SubscriptionRevokeRequest;
use app_store_server_library::primitives::advanced_commerce::subscription_revoke_response::SubscriptionRevokeResponse;
use app_store_server_library::primitives::advanced_commerce::validation_utils::{
    validate_description, validate_display_name, validate_period_count, validate_sku,
    ValidationError,
};
use app_store_server_library::primitives::advanced_commerce_price_increase_info::AdvancedCommercePriceIncreaseInfoStatus;
use uuid::Uuid;

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!("tests/resources/models/{}", name))
        .unwrap_or_else(|e| panic!("failed to read fixture {}: {}", name, e))
}

/// Rust stand-in for Swift's `TestingUtility.confirmCodableInternallyConsistent`:
/// a serialize -> deserialize -> compare-equal round trip.
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
    let parsed: T = serde_json::from_str(&quoted)
        .unwrap_or_else(|e| panic!("{} should deserialize to a variant: {}", raw, e));
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
    assert_enum_raw_value("P1W", Period::P1W);
    assert_enum_raw_value("P1M", Period::P1M);
    assert_enum_raw_value("P2M", Period::P2M);
    assert_enum_raw_value("P3M", Period::P3M);
    assert_enum_raw_value("P6M", Period::P6M);
    assert_enum_raw_value("P1Y", Period::P1Y);

    // Swift: XCTAssertNil(AdvancedCommercePeriod(rawValue: "INVALID"))
    assert!(serde_json::from_str::<Period>("\"INVALID\"").is_err());
    // A guard against a SCREAMING_SNAKE_CASE regression, which would emit "P1_M".
    assert!(serde_json::from_str::<Period>("\"P1_M\"").is_err());
}

#[test]
fn advanced_commerce_reason() {
    assert_enum_raw_value("UPGRADE", Reason::Upgrade);
    assert_enum_raw_value("DOWNGRADE", Reason::Downgrade);
    assert_enum_raw_value("APPLY_OFFER", Reason::ApplyOffer);

    assert!(serde_json::from_str::<Reason>("\"INVALID\"").is_err());
}

#[test]
fn advanced_commerce_refund_reason() {
    assert_enum_raw_value("UNINTENDED_PURCHASE", RefundReason::UnintendedPurchase);
    assert_enum_raw_value("FULFILLMENT_ISSUE", RefundReason::FulfillmentIssue);
    assert_enum_raw_value(
        "UNSATISFIED_WITH_PURCHASE",
        RefundReason::UnsatisfiedWithPurchase,
    );
    assert_enum_raw_value("LEGAL", RefundReason::Legal);
    assert_enum_raw_value("OTHER", RefundReason::Other);
    assert_enum_raw_value("MODIFY_ITEMS_REFUND", RefundReason::ModifyItemsRefund);
    assert_enum_raw_value("SIMULATE_REFUND_DECLINE", RefundReason::SimulateRefundDecline);

    assert!(serde_json::from_str::<RefundReason>("\"INVALID\"").is_err());
}

#[test]
fn advanced_commerce_refund_type() {
    assert_enum_raw_value("FULL", RefundType::Full);
    assert_enum_raw_value("PRORATED", RefundType::Prorated);
    assert_enum_raw_value("CUSTOM", RefundType::Custom);

    assert!(serde_json::from_str::<RefundType>("\"INVALID\"").is_err());
}

#[test]
fn advanced_commerce_offer_period() {
    assert_enum_raw_value("P3D", OfferPeriod::P3d);
    assert_enum_raw_value("P1W", OfferPeriod::P1w);
    assert_enum_raw_value("P2W", OfferPeriod::P2w);
    assert_enum_raw_value("P1M", OfferPeriod::P1m);
    assert_enum_raw_value("P2M", OfferPeriod::P2m);
    assert_enum_raw_value("P3M", OfferPeriod::P3m);
    assert_enum_raw_value("P6M", OfferPeriod::P6m);
    assert_enum_raw_value("P9M", OfferPeriod::P9m);
    assert_enum_raw_value("P1Y", OfferPeriod::P1y);

    assert!(serde_json::from_str::<OfferPeriod>("\"INVALID\"").is_err());
}

#[test]
fn advanced_commerce_offer_reason() {
    assert_enum_raw_value("ACQUISITION", OfferReason::Acquisition);
    assert_enum_raw_value("WIN_BACK", OfferReason::WinBack);
    assert_enum_raw_value("RETENTION", OfferReason::Retention);

    assert!(serde_json::from_str::<OfferReason>("\"INVALID\"").is_err());
}

#[test]
fn advanced_commerce_effective() {
    assert_enum_raw_value("IMMEDIATELY", Effective::Immediately);
    assert_enum_raw_value("NEXT_BILL_CYCLE", Effective::NextBillCycle);

    assert!(serde_json::from_str::<Effective>("\"INVALID\"").is_err());
}

#[test]
fn advanced_commerce_price_increase_info_status() {
    assert_enum_raw_value(
        "SCHEDULED",
        AdvancedCommercePriceIncreaseInfoStatus::Scheduled,
    );
    assert_enum_raw_value("PENDING", AdvancedCommercePriceIncreaseInfoStatus::Pending);
    assert_enum_raw_value("ACCEPTED", AdvancedCommercePriceIncreaseInfoStatus::Accepted);

    assert!(
        serde_json::from_str::<AdvancedCommercePriceIncreaseInfoStatus>("\"INVALID\"").is_err()
    );
}

// ---------------------------------------------------------------------------
// Validation utilities
// ---------------------------------------------------------------------------

#[test]
fn validation_utils_description() {
    assert_eq!(validate_description("a valid description").unwrap(), "a valid description");

    // 45 characters is the documented maximum.
    let at_limit = "a".repeat(45);
    assert_eq!(validate_description(&at_limit).unwrap(), at_limit);

    let too_long = "a".repeat(46);
    assert!(validate_description(&too_long).is_err());
}

#[test]
fn validation_utils_display_name() {
    assert_eq!(validate_display_name("a display name").unwrap(), "a display name");

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
    // Swift bounds this to 1..=12 inclusive via minPeriod/maxPeriod.
    assert_eq!(validate_period_count(1).unwrap(), 1);
    assert_eq!(validate_period_count(12).unwrap(), 12);
    assert_eq!(validate_period_count(6).unwrap(), 6);

    for bad in [0, 13, -1] {
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

// ---------------------------------------------------------------------------
// JSON deserialization
// ---------------------------------------------------------------------------

#[test]
fn advanced_commerce_descriptors() {
    let parsed: Descriptors =
        serde_json::from_str(&fixture("advancedCommerceDescriptors.json")).unwrap();
    assert_eq!(parsed.description, "description");
    assert_eq!(parsed.display_name, "display name");

    assert_codable_round_trips(&parsed);

    // displayName must stay camelCase on the wire.
    let json = serde_json::to_string(&parsed).unwrap();
    assert!(json.contains("\"displayName\""), "got: {}", json);
}

#[test]
fn advanced_commerce_one_time_charge_item() {
    let item: OneTimeChargeItem =
        serde_json::from_str(&fixture("advancedCommerceOneTimeChargeItem.json")).unwrap();
    assert_eq!(item.description, "description");
    assert_eq!(item.display_name, "display name");
    assert_eq!(item.sku, "sku");
    assert_eq!(item.price, 15000);

    assert_codable_round_trips(&item);
}

#[test]
fn advanced_commerce_subscription_create_item() {
    let item: SubscriptionCreateItem =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionCreateItem.json")).unwrap();
    assert_eq!(item.description, "description");
    assert_eq!(item.display_name, "display name");
    assert_eq!(item.sku, "sku");
    assert_eq!(item.price, 20000);

    assert_codable_round_trips(&item);
}

#[test]
fn advanced_commerce_request_refund_item() {
    let item: RequestRefundItem =
        serde_json::from_str(&fixture("advancedCommerceRequestRefundItem.json")).unwrap();
    assert_eq!(item.sku, "sku");
    assert_eq!(item.refund_reason, RefundReason::Legal);
    assert_eq!(item.refund_type, RefundType::Full);
    assert_eq!(item.revoke, true);
    assert_eq!(item.refund_amount, Some(5000));

    assert_codable_round_trips(&item);
}

#[test]
fn advanced_commerce_offer() {
    let offer: Offer = serde_json::from_str(&fixture("advancedCommerceOffer.json")).unwrap();
    assert_eq!(offer.period, OfferPeriod::P1w);
    assert_eq!(offer.period_count, 3);
    assert_eq!(offer.price, 5000);
    assert_eq!(offer.reason, OfferReason::WinBack);

    assert_codable_round_trips(&offer);
}

#[test]
fn advanced_commerce_one_time_charge_create_request() {
    let request: OneTimeChargeCreateRequest =
        serde_json::from_str(&fixture("advancedCommerceOneTimeChargeCreateRequest.json")).unwrap();
    assert_eq!(request.currency, "USD");
    assert_eq!(request.item.sku, "sku");
    assert_eq!(request.tax_code, "taxCode");
    assert_eq!(
        request.request_info.request_reference_id,
        Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap()
    );
    assert_eq!(request.storefront.as_deref(), Some("USA"));

    assert_codable_round_trips(&request);
}

#[test]
fn advanced_commerce_subscription_create_request() {
    let request: SubscriptionCreateRequest =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionCreateRequest.json")).unwrap();
    assert_eq!(request.currency, "USD");
    assert_eq!(request.descriptors.description, "description");
    assert_eq!(request.items.len(), 2);
    assert_eq!(request.period, Period::P1M);
    assert_eq!(request.tax_code, "taxCode");
    assert_eq!(request.storefront.as_deref(), Some("USA"));
    assert_eq!(request.previous_transaction_id.as_deref(), Some("transactionId"));

    assert_codable_round_trips(&request);
}

#[test]
fn advanced_commerce_request_refund_request() {
    let request: RequestRefundRequest =
        serde_json::from_str(&fixture("advancedCommerceRequestRefundRequest.json")).unwrap();
    assert_eq!(request.items.len(), 2);
    assert_eq!(request.refund_risking_preference, true);
    assert_eq!(request.currency.as_deref(), Some("USD"));
    assert_eq!(request.storefront.as_deref(), Some("USA"));

    assert_codable_round_trips(&request);
}

#[test]
fn advanced_commerce_subscription_cancel_request() {
    let request: SubscriptionCancelRequest =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionCancelRequest.json")).unwrap();
    assert_eq!(
        request.request_info.request_reference_id,
        Uuid::parse_str("550e8400-e29b-41d4-a716-446655440003").unwrap()
    );
    assert_eq!(request.storefront.as_deref(), Some("USA"));

    assert_codable_round_trips(&request);
}

#[test]
fn advanced_commerce_subscription_revoke_request() {
    let request: SubscriptionRevokeRequest =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionRevokeRequest.json")).unwrap();
    assert_eq!(
        request.request_info.request_reference_id,
        Uuid::parse_str("550e8400-e29b-41d4-a716-446655440004").unwrap()
    );
    assert_eq!(request.refund_risking_preference, true);
    assert_eq!(request.refund_reason, RefundReason::Legal);
    assert_eq!(request.refund_type, RefundType::Full);
    assert_eq!(request.storefront.as_deref(), Some("USA"));

    assert_codable_round_trips(&request);
}

#[test]
fn advanced_commerce_subscription_price_change_request() {
    let request: SubscriptionPriceChangeRequest =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionPriceChangeRequest.json"))
            .unwrap();
    assert_eq!(request.items.len(), 1);
    assert_eq!(
        request.request_info.request_reference_id,
        Uuid::parse_str("550e8400-e29b-41d4-a716-446655440005").unwrap()
    );
    assert_eq!(request.currency.as_deref(), Some("USD"));

    assert_codable_round_trips(&request);
}

#[test]
fn advanced_commerce_request_refund_response() {
    let response: RequestRefundResponse =
        serde_json::from_str(&fixture("advancedCommerceRequestRefundResponse.json")).unwrap();
    assert_eq!(response.signed_renewal_info, None);
    assert_eq!(response.signed_transaction_info, "signed_transaction_info_value");

    assert_codable_round_trips(&response);
}

#[test]
fn advanced_commerce_subscription_cancel_response() {
    let response: SubscriptionCancelResponse =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionCancelResponse.json")).unwrap();
    assert_eq!(response.signed_renewal_info, "signed_renewal_info");
    assert_eq!(response.signed_transaction_info, "signed_transaction_info");

    assert_codable_round_trips(&response);
}

#[test]
fn advanced_commerce_subscription_revoke_response() {
    let response: SubscriptionRevokeResponse =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionRevokeResponse.json")).unwrap();
    assert_eq!(response.signed_renewal_info, "signed_renewal_info");
    assert_eq!(response.signed_transaction_info, "signed_transaction_info");

    assert_codable_round_trips(&response);
}

#[test]
fn advanced_commerce_subscription_price_change_response() {
    let response: SubscriptionPriceChangeResponse =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionPriceChangeResponse.json"))
            .unwrap();
    assert_eq!(response.signed_renewal_info, "signed_renewal_info");
    assert_eq!(response.signed_transaction_info, "signed_transaction_info");

    assert_codable_round_trips(&response);
}

#[test]
fn advanced_commerce_subscription_change_metadata_response() {
    let response: SubscriptionChangeMetadataResponse =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionChangeMetadataResponse.json"))
            .unwrap();
    assert_eq!(response.signed_renewal_info, "signed_renewal_info");
    assert_eq!(response.signed_transaction_info, "signed_transaction_info");

    assert_codable_round_trips(&response);
}

#[test]
fn advanced_commerce_subscription_migrate_request() {
    let request: SubscriptionMigrateRequest =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionMigrateRequest.json")).unwrap();
    assert!(request.descriptors.is_some());
    assert_eq!(request.items.len(), 1);
    assert_eq!(request.tax_code, "taxCode");
    assert_eq!(request.target_product_id, "targetProductId");

    assert_codable_round_trips(&request);
}

#[test]
fn advanced_commerce_subscription_modify_in_app_request() {
    let request: SubscriptionModifyInAppRequest =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionModifyInAppRequest.json"))
            .unwrap();
    assert_eq!(request.currency.as_deref(), Some("USD"));
    assert!(request.descriptors.is_some());
    assert_eq!(request.tax_code.as_deref(), Some("taxCode"));
    assert_eq!(request.transaction_id, "transactionId");
    assert_eq!(request.retain_billing_cycle, true);

    assert_codable_round_trips(&request);
}

#[test]
fn advanced_commerce_subscription_reactivate_in_app_request() {
    let request: SubscriptionReactivateInAppRequest =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionReactivateInAppRequest.json"))
            .unwrap();
    assert!(request.items.is_some());
    assert_eq!(request.transaction_id, "transactionId");

    assert_codable_round_trips(&request);
}

#[test]
fn advanced_commerce_subscription_change_metadata_request() {
    let request: SubscriptionChangeMetadataRequest =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionChangeMetadataRequest.json"))
            .unwrap();
    assert!(request.items.is_some());
    assert_eq!(
        request.request_info.request_reference_id,
        Uuid::parse_str("550e8400-e29b-41d4-a716-446655440009").unwrap()
    );

    assert_codable_round_trips(&request);
}

#[test]
fn advanced_commerce_subscription_migrate_descriptors() {
    let descriptors: SubscriptionMigrateDescriptors =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionMigrateDescriptors.json"))
            .unwrap();
    assert_eq!(descriptors.description, "description");
    assert_eq!(descriptors.display_name, "displayName");

    assert_codable_round_trips(&descriptors);
}

#[test]
fn advanced_commerce_subscription_modify_descriptors() {
    let descriptors: SubscriptionModifyDescriptors =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionModifyDescriptors.json"))
            .unwrap();
    assert_eq!(descriptors.description.as_deref(), Some("description"));
    assert_eq!(descriptors.display_name.as_deref(), Some("displayName"));
    assert_eq!(descriptors.effective, Effective::Immediately);

    assert_codable_round_trips(&descriptors);
}

#[test]
fn advanced_commerce_subscription_change_metadata_descriptors() {
    let descriptors: SubscriptionChangeMetadataDescriptors =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionChangeMetadataDescriptors.json"))
            .unwrap();
    assert_eq!(descriptors.description.as_deref(), Some("description"));
    assert_eq!(descriptors.display_name.as_deref(), Some("displayName"));
    assert_eq!(descriptors.effective, Effective::Immediately);

    assert_codable_round_trips(&descriptors);
}

#[test]
fn advanced_commerce_subscription_migrate_item() {
    let item: SubscriptionMigrateItem =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionMigrateItem.json")).unwrap();
    assert_eq!(item.description, "description");
    assert_eq!(item.display_name, "displayName");
    assert_eq!(item.sku, "sku");

    assert_codable_round_trips(&item);
}

#[test]
fn advanced_commerce_subscription_migrate_renewal_item() {
    let item: SubscriptionMigrateRenewalItem =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionMigrateRenewalItem.json"))
            .unwrap();
    assert_eq!(item.description, "description");
    assert_eq!(item.display_name, "displayName");
    assert_eq!(item.sku, "sku");

    assert_codable_round_trips(&item);
}

#[test]
fn advanced_commerce_subscription_modify_add_item() {
    let item: SubscriptionModifyAddItem =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionModifyAddItem.json")).unwrap();
    assert_eq!(item.description, "description");
    assert_eq!(item.display_name, "displayName");
    assert_eq!(item.sku, "sku");
    assert_eq!(item.price, 12000);

    // SubscriptionModifyAddItem has no Eq/Hash derive path issue, but confirm round trip.
    assert_codable_round_trips(&item);
}

#[test]
fn advanced_commerce_subscription_modify_change_item() {
    let item: SubscriptionModifyChangeItem =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionModifyChangeItem.json"))
            .unwrap();
    assert_eq!(item.description, "description");
    assert_eq!(item.display_name, "displayName");
    assert_eq!(item.sku, "sku");
    assert_eq!(item.current_sku, "currentSku");
    assert_eq!(item.price, 13000);
    assert_eq!(item.effective, Effective::Immediately);
    assert_eq!(item.reason, Reason::Upgrade);

    assert_codable_round_trips(&item);
}

#[test]
fn advanced_commerce_subscription_modify_remove_item() {
    let item: SubscriptionModifyRemoveItem =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionModifyRemoveItem.json"))
            .unwrap();
    assert_eq!(item.sku, "sku");

    assert_codable_round_trips(&item);
}

#[test]
fn advanced_commerce_subscription_modify_period_change() {
    let period_change: SubscriptionModifyPeriodChange =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionModifyPeriodChange.json"))
            .unwrap();
    assert_eq!(period_change.period, Period::P3M);
    assert_eq!(period_change.effective, Effective::Immediately);

    assert_codable_round_trips(&period_change);
}

#[test]
fn advanced_commerce_subscription_price_change_item() {
    let item: SubscriptionPriceChangeItem =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionPriceChangeItem.json"))
            .unwrap();
    assert_eq!(item.sku, "sku");
    assert_eq!(item.price, 16000);
    assert_eq!(
        item.dependent_skus.as_ref().and_then(|v| v.first()),
        Some(&"dependentSKU".to_string())
    );

    assert_codable_round_trips(&item);
}

#[test]
fn advanced_commerce_subscription_price_change_item_dependent_sku_validation() {
    let valid_sku = "A".repeat(128);
    let too_long_sku = "A".repeat(129);

    let item = SubscriptionPriceChangeItem::new("sku".to_string(), 1000, Some(vec![valid_sku.clone()]))
        .unwrap();
    assert_eq!(item.dependent_skus.as_ref().and_then(|v| v.first()), Some(&valid_sku));

    assert!(matches!(
        SubscriptionPriceChangeItem::new("sku".to_string(), 1000, Some(vec![too_long_sku])),
        Err(ValidationError::SkuTooLong(129))
    ));

    let nil_list_item =
        SubscriptionPriceChangeItem::new("sku".to_string(), 1000, None).unwrap();
    assert_eq!(nil_list_item.dependent_skus, None);
}

#[test]
fn advanced_commerce_subscription_reactivate_item() {
    let item: SubscriptionReactivateItem =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionReactivateItem.json")).unwrap();
    assert_eq!(item.sku, "sku");

    assert_codable_round_trips(&item);
}

#[test]
fn advanced_commerce_subscription_change_metadata_item() {
    let item: SubscriptionChangeMetadataItem =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionChangeMetadataItem.json"))
            .unwrap();
    assert_eq!(item.description.as_deref(), Some("description"));
    assert_eq!(item.display_name.as_deref(), Some("displayName"));
    assert_eq!(item.sku.as_deref(), Some("sku"));
    assert_eq!(item.current_sku, "currentSku");
    assert_eq!(item.effective, Effective::NextBillCycle);

    assert_codable_round_trips(&item);
}

#[test]
fn advanced_commerce_request_info() {
    let request_info: RequestInfo =
        serde_json::from_str(&fixture("advancedCommerceRequestInfo.json")).unwrap();
    assert_eq!(
        request_info.request_reference_id,
        Uuid::parse_str("550e8400-e29b-41d4-a716-446655440010").unwrap()
    );
    assert_eq!(
        request_info.app_account_token,
        Some(Uuid::parse_str("660e8400-e29b-41d4-a716-446655440011").unwrap())
    );
    assert_eq!(request_info.consistency_token.as_deref(), Some("consistency_token_value"));

    assert_codable_round_trips(&request_info);
}

#[test]
fn advanced_commerce_subscription_migrate_response() {
    let response: SubscriptionMigrateResponse =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionMigrateResponse.json")).unwrap();
    assert_eq!(response.signed_renewal_info, "signed_renewal_info_value");
    assert_eq!(response.signed_transaction_info, "signed_transaction_info_value");

    assert_codable_round_trips(&response);
}

// ---------------------------------------------------------------------------
// Operation / version defaults
//
// Apple's Swift library models `operation` and `version` as computed constants:
// they are injected on encode and NEVER decoded. Its fixtures therefore omit both
// keys. The Rust port declares them as plain required fields, so each of these
// four request types carries `#[serde(default = ...)]` to decode a key-less
// fixture while still serializing the constant.
// ---------------------------------------------------------------------------

#[test]
fn one_time_charge_create_request_deserialization_sets_operation_and_version() {
    let parsed: OneTimeChargeCreateRequest =
        serde_json::from_str(&fixture("advancedCommerceOneTimeChargeCreateRequest.json")).unwrap();
    assert_eq!(parsed.operation, InAppRequestOperation::CreateOneTimeCharge);
    assert_eq!(parsed.version, InAppRequestVersion::V1);

    // The constants must still reach the wire.
    let json = serde_json::to_string(&parsed).unwrap();
    assert!(json.contains("\"CREATE_ONE_TIME_CHARGE\""), "got: {}", json);
    assert!(json.contains("\"version\":\"1\""), "got: {}", json);
}

#[test]
fn subscription_create_request_deserialization_sets_operation_and_version() {
    let parsed: SubscriptionCreateRequest =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionCreateRequest.json")).unwrap();
    assert_eq!(parsed.operation, InAppRequestOperation::CreateSubscription);
    assert_eq!(parsed.version, InAppRequestVersion::V1);

    let json = serde_json::to_string(&parsed).unwrap();
    assert!(json.contains("\"CREATE_SUBSCRIPTION\""), "got: {}", json);
}

#[test]
fn subscription_modify_in_app_request_deserialization_sets_operation_and_version() {
    let parsed: SubscriptionModifyInAppRequest =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionModifyInAppRequest.json"))
            .unwrap();
    assert_eq!(parsed.operation, InAppRequestOperation::ModifySubscription);
    assert_eq!(parsed.version, InAppRequestVersion::V1);

    let json = serde_json::to_string(&parsed).unwrap();
    assert!(json.contains("\"MODIFY_SUBSCRIPTION\""), "got: {}", json);
}

#[test]
fn subscription_reactivate_in_app_request_deserialization_sets_operation_and_version() {
    let parsed: SubscriptionReactivateInAppRequest =
        serde_json::from_str(&fixture("advancedCommerceSubscriptionReactivateInAppRequest.json"))
            .unwrap();
    assert_eq!(parsed.operation, InAppRequestOperation::ReactivateSubscription);
    assert_eq!(parsed.version, InAppRequestVersion::V1);

    let json = serde_json::to_string(&parsed).unwrap();
    assert!(json.contains("\"REACTIVATE_SUBSCRIPTION\""), "got: {}", json);
}
