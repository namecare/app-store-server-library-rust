use app_store_server_library::models::alternate_product::AlternateProduct;
use app_store_server_library::models::billing_plan_type::BillingPlanType;
use app_store_server_library::models::bullet_point::{BulletPoint, BulletPointValidationError};
use app_store_server_library::models::get_image_list_response::GetImageListResponse;
use app_store_server_library::models::header_position::HeaderPosition;
use app_store_server_library::models::image_size::ImageSize;
use app_store_server_library::models::performance_test_response::PerformanceTestResponse;
use app_store_server_library::models::performance_test_result_response::PerformanceTestResultResponse;
use app_store_server_library::models::performance_test_status::PerformanceTestStatus;
use app_store_server_library::models::realtime_url_request::RealtimeUrlRequest;
use app_store_server_library::models::realtime_url_response::RealtimeUrlResponse;
use uuid::Uuid;

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!("tests/resources/models/{}", name))
        .unwrap_or_else(|e| panic!("failed to read fixture {}: {}", name, e))
}

#[test]
fn header_position_round_trips_both_values() {
    assert_eq!(
        serde_json::from_str::<HeaderPosition>("\"ABOVE_BODY\"").unwrap(),
        HeaderPosition::AboveBody
    );
    assert_eq!(
        serde_json::from_str::<HeaderPosition>("\"ABOVE_IMAGE\"").unwrap(),
        HeaderPosition::AboveImage
    );
    assert_eq!(
        serde_json::to_string(&HeaderPosition::AboveImage).unwrap(),
        "\"ABOVE_IMAGE\""
    );
}

#[test]
fn image_size_round_trips_both_values() {
    assert_eq!(
        serde_json::from_str::<ImageSize>("\"FULL_SIZE\"").unwrap(),
        ImageSize::FullSize
    );
    assert_eq!(
        serde_json::from_str::<ImageSize>("\"BULLET_POINT\"").unwrap(),
        ImageSize::BulletPoint
    );
    assert_eq!(
        serde_json::to_string(&ImageSize::FullSize).unwrap(),
        "\"FULL_SIZE\""
    );
}

#[test]
fn bullet_point_accepts_text_at_the_limit() {
    let text = "a".repeat(66);
    let alt = "b".repeat(150);
    let bp = BulletPoint::new(text.clone(), Uuid::nil(), alt.clone())
        .expect("66-char text and 150-char alt text are both legal");
    assert_eq!(bp.text, text);
    assert_eq!(bp.alt_text, alt);
}

#[test]
fn bullet_point_rejects_text_over_66_chars() {
    let err = BulletPoint::new("a".repeat(67), Uuid::nil(), "ok".to_string()).unwrap_err();
    assert_eq!(err, BulletPointValidationError::TextTooLong);
}

#[test]
fn bullet_point_rejects_alt_text_over_150_chars() {
    let err = BulletPoint::new("ok".to_string(), Uuid::nil(), "b".repeat(151)).unwrap_err();
    assert_eq!(err, BulletPointValidationError::AltTextTooLong);
}

#[test]
fn bullet_point_serializes_with_camel_case_keys() {
    let bp = BulletPoint::new("hello".to_string(), Uuid::nil(), "alt".to_string()).unwrap();
    let json = serde_json::to_string(&bp).unwrap();
    assert!(json.contains("\"imageIdentifier\""), "got: {}", json);
    assert!(json.contains("\"altText\""), "got: {}", json);
}

#[test]
fn realtime_url_request_rejects_url_over_256_chars() {
    let long = format!("https://example.com/{}", "a".repeat(300));
    assert!(RealtimeUrlRequest::new(long).is_err());
}

#[test]
fn realtime_url_request_serializes_capital_url_key() {
    let req = RealtimeUrlRequest::new("https://example.com/realtime".to_string()).unwrap();
    let json = serde_json::to_string(&req).unwrap();
    assert!(
        json.contains("\"realtimeURL\""),
        "key must be realtimeURL with capital URL, got: {}",
        json
    );
}

#[test]
fn realtime_url_response_serializes_capital_url_key() {
    let resp = RealtimeUrlResponse {
        realtime_url: "https://example.com/realtime".to_string(),
    };
    let json = serde_json::to_string(&resp).unwrap();
    assert!(
        json.contains("\"realtimeURL\""),
        "key must be realtimeURL with capital URL, got: {}",
        json
    );
}

#[test]
fn realtime_url_response_parses_fixture() {
    let parsed: RealtimeUrlResponse = serde_json::from_str(&fixture("getRealtimeUrlResponse.json")).unwrap();
    assert_eq!(parsed.realtime_url, "https://example.com/realtime");
}

#[test]
fn performance_test_response_parses_fixture() {
    let parsed: PerformanceTestResponse = serde_json::from_str(&fixture("performanceTestResponse.json")).unwrap();
    assert_eq!(parsed.config.max_concurrent_requests, 10);
    assert_eq!(parsed.config.total_requests, 100);
    assert_eq!(parsed.config.total_duration, 60000);
    assert_eq!(parsed.config.response_time_threshold, 500);
    assert_eq!(parsed.config.success_rate_threshold, 95);
    assert_eq!(
        parsed.request_id,
        Uuid::parse_str("c4b87a1d-2e3f-4a5b-9c6d-7e8f9a0b1c2d").unwrap()
    );
}

#[test]
fn performance_test_status_round_trips_all_values() {
    assert_eq!(
        serde_json::from_str::<PerformanceTestStatus>("\"PENDING\"").unwrap(),
        PerformanceTestStatus::Pending
    );
    assert_eq!(
        serde_json::from_str::<PerformanceTestStatus>("\"PASS\"").unwrap(),
        PerformanceTestStatus::Pass
    );
    assert_eq!(
        serde_json::from_str::<PerformanceTestStatus>("\"FAIL\"").unwrap(),
        PerformanceTestStatus::Fail
    );
}

#[test]
fn performance_test_result_response_parses_fixture() {
    let parsed: PerformanceTestResultResponse =
        serde_json::from_str(&fixture("performanceTestResultResponse.json")).unwrap();
    assert_eq!(parsed.config.max_concurrent_requests, 10);
    assert_eq!(parsed.result, PerformanceTestStatus::Pass);
    assert_eq!(parsed.success_rate, 98);
    assert_eq!(parsed.num_pending, 0);
    assert_eq!(parsed.target, "https://example.com/retention");
    assert_eq!(parsed.response_times.average, 120);
    assert_eq!(parsed.response_times.p50, 100);
    assert_eq!(parsed.response_times.p90, 200);
    assert_eq!(parsed.response_times.p95, 250);
    assert_eq!(parsed.response_times.p99, 400);
    assert_eq!(parsed.failures.len(), 2);
}

#[test]
fn image_list_response_parses_image_size_from_fixture() {
    let parsed: GetImageListResponse = serde_json::from_str(&fixture("getImageListResponse.json")).unwrap();
    let items = parsed
        .image_identifiers
        .as_ref()
        .expect("fixture has imageIdentifiers");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].image_size, Some(ImageSize::FullSize));
    assert_eq!(
        items[0].image_identifier,
        Some(Uuid::parse_str("a1b2c3d4-e5f6-7890-a1b2-c3d4e5f67890").unwrap())
    );
}

#[test]
fn alternate_product_serializes_with_explicit_renames() {
    let product = AlternateProduct {
        message_identifier: Some(Uuid::nil()),
        product_id: Some("com.example.premium".to_string()),
        billing_plan_type: Some(BillingPlanType::Monthly),
    };
    let json = serde_json::to_string(&product).unwrap();
    assert!(json.contains("\"messageIdentifier\""), "got: {}", json);
    assert!(json.contains("\"productId\""), "got: {}", json);
    assert!(json.contains("\"billingPlanType\""), "got: {}", json);

    let parsed: AlternateProduct = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, product);
}
