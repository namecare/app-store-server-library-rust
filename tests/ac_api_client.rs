mod common;
use common::{MockTransport, RequestVerifier};
use app_store_server_library::api_client::api::advanced_commerce_api::AdvancedCommerceAPIClient;
use app_store_server_library::api_client::error::ConfigurationError;
use app_store_server_library::primitives::environment::Environment;
use app_store_server_library::primitives::advanced_commerce::subscription_cancel_request::SubscriptionCancelRequest;
use app_store_server_library::primitives::advanced_commerce::subscription_revoke_request::SubscriptionRevokeRequest;
use app_store_server_library::primitives::advanced_commerce::request_refund_request::RequestRefundRequest;
use app_store_server_library::primitives::advanced_commerce::subscription_change_metadata_request::SubscriptionChangeMetadataRequest;
use app_store_server_library::primitives::advanced_commerce::subscription_price_change_request::SubscriptionPriceChangeRequest;
use app_store_server_library::primitives::advanced_commerce::subscription_migrate_request::SubscriptionMigrateRequest;
use app_store_server_library::primitives::advanced_commerce::request_info::RequestInfo;
use app_store_server_library::primitives::advanced_commerce::refund_reason::RefundReason;
use app_store_server_library::primitives::advanced_commerce::refund_type::RefundType;
use http::{Method, StatusCode};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use uuid::Uuid;

#[tokio::test]
async fn test_cancel_subscription() {
    let client = advanced_commerce_api_client_with_body_from_file(
        "tests/resources/models/subscriptionCancelResponse.json",
        StatusCode::OK,
        Some(Box::new(|req, body| {
            assert_eq!(&Method::POST, req.method());
            assert_eq!(
                "https://local-testing-base-url/advancedCommerce/v1/subscription/cancel/test_transaction_id",
                req.uri().to_string()
            );

            let decoded_json: HashMap<&str, Value> = serde_json::from_slice(body).unwrap();
            assert!(decoded_json.contains_key("requestInfo"));
        })),
    );

    let request = SubscriptionCancelRequest::new(Uuid::new_v4())
        .with_storefront("test_storefront".to_string());

    let response = client
        .cancel_subscription("test_transaction_id", &request)
        .await
        .unwrap();

    assert!(!response.signed_transaction_info.is_empty());
    assert!(!response.signed_renewal_info.is_empty());
}

#[tokio::test]
async fn test_revoke_subscription() {
    let client = advanced_commerce_api_client_with_body_from_file(
        "tests/resources/models/subscriptionRevokeResponse.json",
        StatusCode::OK,
        Some(Box::new(|req, body| {
            assert_eq!(&Method::POST, req.method());
            assert_eq!(
                "https://local-testing-base-url/advancedCommerce/v1/subscription/revoke/test_transaction_id",
                req.uri().to_string()
            );

            let decoded_json: HashMap<&str, Value> = serde_json::from_slice(body).unwrap();
            assert!(decoded_json.contains_key("requestInfo"));
            assert!(decoded_json.contains_key("refundReason"));
            assert!(decoded_json.contains_key("refundType"));
        })),
    );

    let request = SubscriptionRevokeRequest::new(
        Uuid::new_v4(),
        RefundReason::UnsatisfiedWithPurchase,
        "LOW_ENGAGEMENT".to_string(),
        RefundType::Full,
    );

    let response = client
        .revoke_subscription("test_transaction_id", &request)
        .await
        .unwrap();

    assert!(!response.signed_transaction_info.is_empty());
    assert!(!response.signed_renewal_info.is_empty());
}

#[tokio::test]
async fn test_request_transaction_refund() {
    let client = advanced_commerce_api_client_with_body_from_file(
        "tests/resources/models/requestRefundResponse.json",
        StatusCode::OK,
        Some(Box::new(|req, body| {
            assert_eq!(&Method::POST, req.method());
            assert_eq!(
                "https://local-testing-base-url/advancedCommerce/v1/transaction/requestRefund/test_transaction_id",
                req.uri().to_string()
            );

            let decoded_json: HashMap<&str, Value> = serde_json::from_slice(body).unwrap();
            assert!(decoded_json.contains_key("requestInfo"));
            assert!(decoded_json.contains_key("items"));
        })),
    );

    let request = RequestRefundRequest {
        request_info: RequestInfo::new(Uuid::new_v4()),
        currency: None,
        items: vec![],
        refund_risking_preference: false,
        storefront: None,
    };

    let response = client
        .request_transaction_refund("test_transaction_id", &request)
        .await
        .unwrap();

    assert!(!response.signed_transaction_info.is_empty());
}

#[tokio::test]
async fn test_change_subscription_metadata() {
    let client = advanced_commerce_api_client_with_body_from_file(
        "tests/resources/models/subscriptionChangeMetadataResponse.json",
        StatusCode::OK,
        Some(Box::new(|req, body| {
            assert_eq!(&Method::POST, req.method());
            assert_eq!(
                "https://local-testing-base-url/advancedCommerce/v1/subscription/changeMetadata/test_transaction_id",
                req.uri().to_string()
            );

            let decoded_json: HashMap<&str, Value> = serde_json::from_slice(body).unwrap();
            assert!(decoded_json.contains_key("requestInfo"));
            assert!(decoded_json.contains_key("items"));
        })),
    );

    let request = SubscriptionChangeMetadataRequest::new(Uuid::new_v4())
        .with_items(vec![]);

    let response = client
        .change_subscription_metadata("test_transaction_id", &request)
        .await
        .unwrap();

    assert!(!response.signed_transaction_info.is_empty());
    assert!(!response.signed_renewal_info.is_empty());
}

#[tokio::test]
async fn test_change_subscription_price() {
    let client = advanced_commerce_api_client_with_body_from_file(
        "tests/resources/models/subscriptionPriceChangeResponse.json",
        StatusCode::OK,
        Some(Box::new(|req, body| {
            assert_eq!(&Method::POST, req.method());
            assert_eq!(
                "https://local-testing-base-url/advancedCommerce/v1/subscription/changePrice/test_transaction_id",
                req.uri().to_string()
            );

            let decoded_json: HashMap<&str, Value> = serde_json::from_slice(body).unwrap();
            assert!(decoded_json.contains_key("requestInfo"));
            assert!(decoded_json.contains_key("items"));
        })),
    );

    let request = SubscriptionPriceChangeRequest::new(
        "test_storefront".to_string(),
        vec![],
        Uuid::new_v4(),
    );

    let response = client
        .change_subscription_price("test_transaction_id", &request)
        .await
        .unwrap();

    assert!(!response.signed_transaction_info.is_empty());
    assert!(!response.signed_renewal_info.is_empty());
}

#[tokio::test]
async fn test_migrate_subscription() {
    let client = advanced_commerce_api_client_with_body_from_file(
        "tests/resources/models/subscriptionMigrateResponse.json",
        StatusCode::OK,
        Some(Box::new(|req, body| {
            assert_eq!(&Method::POST, req.method());
            assert_eq!(
                "https://local-testing-base-url/advancedCommerce/v1/subscription/migrate/test_transaction_id",
                req.uri().to_string()
            );

            let decoded_json: HashMap<&str, Value> = serde_json::from_slice(body).unwrap();
            assert!(decoded_json.contains_key("requestInfo"));
            assert!(decoded_json.contains_key("items"));
        })),
    );

    let request = SubscriptionMigrateRequest::new(
        Uuid::new_v4(),
        vec![],
        "target_product".to_string(),
        "tax_code".to_string(),
    );

    let response = client
        .migrate_subscription("test_transaction_id", &request)
        .await
        .unwrap();

    assert!(!response.signed_transaction_info.is_empty());
    assert!(!response.signed_renewal_info.is_empty());
}

#[test]
fn test_xcode_environment_is_not_supported() {
    let mock_transport = MockTransport::new(
        String::new(),
        StatusCode::OK,
        None
    );

    let result = AdvancedCommerceAPIClient::new(
        vec![],
        "test_key_id",
        "test_issuer_id",
        "com.test.app",
        Environment::Xcode,
        mock_transport,
    );

    assert!(result.is_err());
    match result {
        Err(ConfigurationError::InvalidEnvironment(msg)) => {
            assert!(msg.contains("Xcode environment is not supported"));
        }
        _ => panic!("Expected InvalidEnvironment error"),
    }
}

#[test]
fn test_sandbox_environment_is_accepted() {
    let mock_transport = MockTransport::new(
        String::new(),
        StatusCode::OK,
        None
    );

    let result = AdvancedCommerceAPIClient::new(
        vec![],
        "test_key_id",
        "test_issuer_id",
        "com.test.app",
        Environment::Sandbox,
        mock_transport,
    );

    assert!(result.is_ok());
}

#[test]
fn test_production_environment_is_accepted() {
    let mock_transport = MockTransport::new(
        String::new(),
        StatusCode::OK,
        None
    );

    let result = AdvancedCommerceAPIClient::new(
        vec![],
        "test_key_id",
        "test_issuer_id",
        "com.test.app",
        Environment::Production,
        mock_transport,
    );

    assert!(result.is_ok());
}

fn advanced_commerce_api_client_with_body_from_file(
    path: &str,
    status: StatusCode,
    request_verifier: Option<RequestVerifier>,
) -> AdvancedCommerceAPIClient<MockTransport> {
    let body = fs::read_to_string(path).expect("Failed to read file");
    advanced_commerce_api_client(body, status, request_verifier)
}

fn advanced_commerce_api_client(
    body: String,
    status: StatusCode,
    request_verifier: Option<RequestVerifier>,
) -> AdvancedCommerceAPIClient<MockTransport> {
    let key = fs::read("tests/resources/certs/testSigningKey.p8").expect("Failed to read file");

    let mock_transport = MockTransport::new(body, status, request_verifier);

    AdvancedCommerceAPIClient::new(
        key,
        "keyId",
        "issuerId",
        "com.example",
        Environment::LocalTesting,
        mock_transport,
    )
    .expect("Error creating advanced commerce client")
}