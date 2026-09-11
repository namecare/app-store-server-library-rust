use app_store_server_library::primitives::advanced_commerce::offer::Offer;
use app_store_server_library::primitives::advanced_commerce::offer_period::OfferPeriod;
use app_store_server_library::primitives::advanced_commerce::offer_reason::OfferReason;
use app_store_server_library::primitives::advanced_commerce::period::Period;
use app_store_server_library::primitives::jws_renewal_info_decoded_payload::JWSRenewalInfoDecodedPayload;
use app_store_server_library::primitives::jws_transaction_decoded_payload::JWSTransactionDecodedPayload;
use chrono::{DateTime, Utc};

fn transaction_with_items(items: &str) -> JWSTransactionDecodedPayload {
    let json = format!(
        r#"{{
            "originalTransactionId": "2000000000000001",
            "transactionId": "2000000000000002",
            "bundleId": "com.example.app",
            "productId": "com.example.generic",
            "type": "Auto-Renewable Subscription",
            "environment": "Production",
            "currency": "USD",
            "advancedCommerceInfo": {{
                "descriptors": {{ "description": "Premium", "displayName": "Premium" }},
                "estimatedTax": 0,
                "items": {items},
                "period": "P1M",
                "requestReferenceId": "3a7e7d1c-1d7a-4e0e-9a5f-0a1b2c3d4e5f",
                "taxCode": "C003-00-2",
                "taxExclusivePrice": 8000,
                "taxRate": "0"
            }}
        }}"#
    );
    serde_json::from_str(&json).expect("a transaction payload with advanced commerce info")
}

/// Apple documents every `advancedCommerceTransactionItem` property as optional; a
/// line item bought without an offer, never refunded and never revoked carries only
/// its SKU, descriptors and price.
#[test]
fn transaction_item_without_offer_refunds_or_revocation_decodes() {
    let payload = transaction_with_items(
        r#"[{ "SKU": "premium.monthly", "description": "Premium", "displayName": "Premium", "price": 8000 }]"#,
    );
    let info = payload.advanced_commerce_info.expect("advanced commerce info");
    assert_eq!(info.period, Some(Period::P1M));
    let items = info.items.expect("items");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].sku.as_deref(), Some("premium.monthly"));
    assert_eq!(items[0].price, Some(8000));
    assert_eq!(items[0].offer, None);
    assert_eq!(items[0].refunds, None);
    assert_eq!(items[0].revocation_date, None);
}

#[test]
fn transaction_item_with_offer_and_revocation_decodes() {
    let payload = transaction_with_items(
        r#"[{
            "SKU": "premium.monthly",
            "description": "Premium",
            "displayName": "Premium",
            "offer": { "period": "P1M", "periodCount": 1, "price": 1000, "reason": "ACQUISITION" },
            "price": 8000,
            "refunds": [{ "refundAmount": 8000, "refundDate": 1698148900000, "refundReason": "OTHER", "refundType": "FULL" }],
            "revocationDate": 1698148900000
        }]"#,
    );
    let info = payload.advanced_commerce_info.expect("advanced commerce info");
    let items = info.items.expect("items");
    assert_eq!(
        items[0].offer,
        Some(Offer::new(OfferPeriod::P1m, 1, 1000, OfferReason::Acquisition))
    );
    assert_eq!(items[0].refunds.as_ref().map(Vec::len), Some(1));
    assert_eq!(
        items[0].revocation_date,
        Some(DateTime::<Utc>::from_timestamp_millis(1698148900000).unwrap())
    );
}

#[test]
fn period_uses_apples_iso_8601_codes_on_the_wire() {
    for (period, wire) in [
        (Period::P1W, "\"P1W\""),
        (Period::P1M, "\"P1M\""),
        (Period::P2M, "\"P2M\""),
        (Period::P3M, "\"P3M\""),
        (Period::P6M, "\"P6M\""),
        (Period::P1Y, "\"P1Y\""),
    ] {
        assert_eq!(serde_json::to_string(&period).unwrap(), wire);
        assert_eq!(serde_json::from_str::<Period>(wire).unwrap(), period);
    }
}

#[test]
fn advanced_commerce_info_with_no_properties_decodes() {
    let payload: JWSTransactionDecodedPayload = serde_json::from_str(
        r#"{ "originalTransactionId": "2000000000000001", "advancedCommerceInfo": {} }"#,
    )
    .expect("an empty advanced commerce object");
    let info = payload.advanced_commerce_info.expect("advanced commerce info");
    assert_eq!(info.items, None);
    assert_eq!(info.period, None);
}

#[test]
fn renewal_item_without_offer_decodes() {
    let payload: JWSRenewalInfoDecodedPayload = serde_json::from_str(
        r#"{
            "originalTransactionId": "2000000000000001",
            "autoRenewStatus": 1,
            "environment": "Production",
            "advancedCommerceInfo": {
                "consistencyToken": "token",
                "descriptors": { "description": "Premium", "displayName": "Premium" },
                "items": [{ "SKU": "premium.monthly", "description": "Premium", "displayName": "Premium", "price": 8000 }],
                "period": "P1M",
                "requestReferenceId": "3a7e7d1c-1d7a-4e0e-9a5f-0a1b2c3d4e5f",
                "taxCode": "C003-00-2"
            }
        }"#,
    )
    .expect("a renewal payload with advanced commerce info");
    let info = payload.advanced_commerce_info.expect("advanced commerce info");
    let items = info.items.expect("items");
    assert_eq!(items[0].offer, None);
    assert_eq!(items[0].price, Some(8000));
}
