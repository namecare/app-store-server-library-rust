mod common;
use std::fs;

use app_store_server_library::advanced_commerce_api_client::AdvancedCommerceApiClient;
use app_store_server_library::api_client::error::ConfigurationError;
use app_store_server_library::models::advanced_commerce_effective::AdvancedCommerceEffective;
use app_store_server_library::models::advanced_commerce_refund_reason::AdvancedCommerceRefundReason;
use app_store_server_library::models::advanced_commerce_refund_type::AdvancedCommerceRefundType;
use app_store_server_library::models::advanced_commerce_request_info::AdvancedCommerceRequestInfo;
use app_store_server_library::models::advanced_commerce_request_refund_item::AdvancedCommerceRequestRefundItem;
use app_store_server_library::models::advanced_commerce_request_refund_request::AdvancedCommerceRequestRefundRequest;
use app_store_server_library::models::advanced_commerce_subscription_cancel_request::AdvancedCommerceSubscriptionCancelRequest;
use app_store_server_library::models::advanced_commerce_subscription_change_metadata_descriptors::AdvancedCommerceSubscriptionChangeMetadataDescriptors;
use app_store_server_library::models::advanced_commerce_subscription_change_metadata_item::AdvancedCommerceSubscriptionChangeMetadataItem;
use app_store_server_library::models::advanced_commerce_subscription_change_metadata_request::AdvancedCommerceSubscriptionChangeMetadataRequest;
use app_store_server_library::models::advanced_commerce_subscription_migrate_descriptors::AdvancedCommerceSubscriptionMigrateDescriptors;
use app_store_server_library::models::advanced_commerce_subscription_migrate_item::AdvancedCommerceSubscriptionMigrateItem;
use app_store_server_library::models::advanced_commerce_subscription_migrate_request::AdvancedCommerceSubscriptionMigrateRequest;
use app_store_server_library::models::advanced_commerce_subscription_price_change_item::AdvancedCommerceSubscriptionPriceChangeItem;
use app_store_server_library::models::advanced_commerce_subscription_price_change_request::AdvancedCommerceSubscriptionPriceChangeRequest;
use app_store_server_library::models::advanced_commerce_subscription_revoke_request::AdvancedCommerceSubscriptionRevokeRequest;
use app_store_server_library::models::app_store_environment::Environment;
use common::assert_codable_round_trips;
use common::transport_mock::{MockTransport, RequestVerifier};
use http::{Method, StatusCode};
use serde_json::Value;
use uuid::Uuid;

#[tokio::test]
async fn test_change_subscription_price() {
    let client = advanced_commerce_api_client_with_body_from_file(
        "tests/resources/models/advancedCommerceSubscriptionPriceChangeResponse.json",
        StatusCode::OK,
        Some(Box::new(|req, body| {
            assert_eq!(&Method::POST, req.method());
            assert_eq!(
                "https://local-testing-base-url/advancedCommerce/v1/subscription/changePrice/4124214",
                req.uri().to_string()
            );

            let decoded_json: Value = serde_json::from_slice(body).unwrap();
            assert_eq!(
                "7C80BB86-F892-4B21-A919-1357811D6C4F",
                decoded_json["requestInfo"]["requestReferenceId"]
                    .as_str()
                    .unwrap()
                    .to_uppercase()
            );
            assert_eq!("USD", decoded_json["currency"]);
            assert_eq!("USA", decoded_json["storefront"]);
            let item = &decoded_json["items"][0];
            assert_eq!("AD_FREE_1M", item["SKU"]);
            assert_eq!(12990, item["price"]);
            assert_eq!(
                serde_json::json!(["ADVANCED_FEATURES_1M"]),
                item["dependentSKUs"]
            );
        })),
    );

    let request = AdvancedCommerceSubscriptionPriceChangeRequest::new(
        "USD".to_string(),
        vec![AdvancedCommerceSubscriptionPriceChangeItem::new(
            "AD_FREE_1M".to_string(),
            12990,
            Some(vec!["ADVANCED_FEATURES_1M".to_string()]),
        )
        .unwrap()],
        Uuid::parse_str("7c80bb86-f892-4b21-a919-1357811d6c4f").unwrap(),
    )
    .unwrap()
    .with_storefront("USA".to_string());

    assert_codable_round_trips(&request);

    let response = client
        .change_subscription_price("4124214", &request)
        .await
        .unwrap();

    assert_eq!("signed_renewal_info", response.signed_renewal_info);
    assert_eq!("signed_transaction_info", response.signed_transaction_info);
}

#[tokio::test]
async fn test_cancel_subscription() {
    let client = advanced_commerce_api_client_with_body_from_file(
        "tests/resources/models/advancedCommerceSubscriptionCancelResponse.json",
        StatusCode::OK,
        Some(Box::new(|req, body| {
            assert_eq!(&Method::POST, req.method());
            assert_eq!(
                "https://local-testing-base-url/advancedCommerce/v1/subscription/cancel/4124214",
                req.uri().to_string()
            );

            let decoded_json: Value = serde_json::from_slice(body).unwrap();
            assert_eq!(
                "932C6903-0AB8-4469-9F21-015F6FAB013C",
                decoded_json["requestInfo"]["requestReferenceId"]
                    .as_str()
                    .unwrap()
                    .to_uppercase()
            );
            assert_eq!("USA", decoded_json["storefront"]);
        })),
    );

    let request = AdvancedCommerceSubscriptionCancelRequest::new(
        Uuid::parse_str("932c6903-0ab8-4469-9f21-015f6fab013c").unwrap(),
    )
    .with_storefront("USA".to_string());

    assert_codable_round_trips(&request);

    let response = client
        .cancel_subscription("4124214", &request)
        .await
        .unwrap();

    assert_eq!("signed_renewal_info", response.signed_renewal_info);
    assert_eq!("signed_transaction_info", response.signed_transaction_info);
}

#[tokio::test]
async fn test_revoke_subscription() {
    let client = advanced_commerce_api_client_with_body_from_file(
        "tests/resources/models/advancedCommerceSubscriptionRevokeResponse.json",
        StatusCode::OK,
        Some(Box::new(|req, body| {
            assert_eq!(&Method::POST, req.method());
            assert_eq!(
                "https://local-testing-base-url/advancedCommerce/v1/subscription/revoke/4124214",
                req.uri().to_string()
            );

            let decoded_json: Value = serde_json::from_slice(body).unwrap();
            assert_eq!(
                "932C6903-0AB8-4469-9F21-015F6FAB013C",
                decoded_json["requestInfo"]["requestReferenceId"]
                    .as_str()
                    .unwrap()
                    .to_uppercase()
            );
            assert_eq!("UNINTENDED_PURCHASE", decoded_json["refundReason"]);
            assert_eq!(true, decoded_json["refundRiskingPreference"]);
            assert_eq!("PRORATED", decoded_json["refundType"]);
            assert_eq!("USA", decoded_json["storefront"]);
        })),
    );

    let request = AdvancedCommerceSubscriptionRevokeRequest::new(
        Uuid::parse_str("932c6903-0ab8-4469-9f21-015f6fab013c").unwrap(),
        AdvancedCommerceRefundReason::UnintendedPurchase,
        true,
        AdvancedCommerceRefundType::Prorated,
    )
    .with_storefront("USA".to_string());

    assert_codable_round_trips(&request);

    let response = client
        .revoke_subscription("4124214", &request)
        .await
        .unwrap();

    assert_eq!("signed_renewal_info", response.signed_renewal_info);
    assert_eq!("signed_transaction_info", response.signed_transaction_info);
}

#[tokio::test]
async fn test_request_transaction_refund() {
    let client = advanced_commerce_api_client_with_body_from_file(
        "tests/resources/models/advancedCommerceRequestRefundResponse.json",
        StatusCode::OK,
        Some(Box::new(|req, body| {
            assert_eq!(&Method::POST, req.method());
            assert_eq!(
                "https://local-testing-base-url/advancedCommerce/v1/transaction/requestRefund/4124214",
                req.uri().to_string()
            );

            let decoded_json: Value = serde_json::from_slice(body).unwrap();
            assert_eq!(
                "932C6903-0AB8-4469-9F21-015F6FAB013C",
                decoded_json["requestInfo"]["requestReferenceId"]
                    .as_str()
                    .unwrap()
                    .to_uppercase()
            );
            assert_eq!(true, decoded_json["refundRiskingPreference"]);
            assert_eq!("USD", decoded_json["currency"]);
            assert_eq!("USA", decoded_json["storefront"]);
            let item = &decoded_json["items"][0];
            assert_eq!("AD_FREE_1M", item["SKU"]);
            assert_eq!("UNSATISFIED_WITH_PURCHASE", item["refundReason"]);
            assert_eq!("FULL", item["refundType"]);
            assert_eq!(true, item["revoke"]);
        })),
    );

    let request = AdvancedCommerceRequestRefundRequest::new(
        vec![AdvancedCommerceRequestRefundItem {
            sku: "AD_FREE_1M".to_string(),
            refund_amount: None,
            refund_reason: AdvancedCommerceRefundReason::UnsatisfiedWithPurchase,
            refund_type: AdvancedCommerceRefundType::Full,
            revoke: true,
        }],
        true,
        AdvancedCommerceRequestInfo::new(Uuid::parse_str("932c6903-0ab8-4469-9f21-015f6fab013c").unwrap()),
    )
    .unwrap()
    .with_currency("USD".to_string())
    .with_storefront("USA".to_string());

    assert_codable_round_trips(&request);

    let response = client
        .request_transaction_refund("4124214", &request)
        .await
        .unwrap();

    assert_eq!(
        "signed_transaction_info_value",
        response.signed_transaction_info
    );
}

#[tokio::test]
async fn test_change_subscription_metadata() {
    let client = advanced_commerce_api_client_with_body_from_file(
        "tests/resources/models/advancedCommerceSubscriptionChangeMetadataResponse.json",
        StatusCode::OK,
        Some(Box::new(|req, body| {
            assert_eq!(&Method::POST, req.method());
            assert_eq!(
                "https://local-testing-base-url/advancedCommerce/v1/subscription/changeMetadata/4124214",
                req.uri().to_string()
            );

            let decoded_json: Value = serde_json::from_slice(body).unwrap();
            assert_eq!(
                "932C6903-0AB8-4469-9F21-015F6FAB013C",
                decoded_json["requestInfo"]["requestReferenceId"]
                    .as_str()
                    .unwrap()
                    .to_uppercase()
            );
            let descriptors = &decoded_json["descriptors"];
            assert_eq!("NEXT_BILL_CYCLE", descriptors["effective"]);
            assert_eq!(
                "Remove ads and unlock advanced features.",
                descriptors["description"]
            );
            assert_eq!("Ad-free package", descriptors["displayName"]);
            let item = &decoded_json["items"][0];
            assert_eq!("AD_FREE_1M", item["currentSKU"]);
            assert_eq!("NEXT_BILL_CYCLE", item["effective"]);
            assert_eq!("AD_FREE_1M_V2", item["SKU"]);
            assert_eq!("USA", decoded_json["storefront"]);
            assert_eq!("C003-00-1", decoded_json["taxCode"]);
        })),
    );

    let request = AdvancedCommerceSubscriptionChangeMetadataRequest::new(
        Uuid::parse_str("932c6903-0ab8-4469-9f21-015f6fab013c").unwrap(),
    )
    .with_descriptors(
        AdvancedCommerceSubscriptionChangeMetadataDescriptors::new(AdvancedCommerceEffective::NextBillCycle)
            .with_description("Remove ads and unlock advanced features.".to_string())
            .with_display_name("Ad-free package".to_string()),
    )
    .with_items(vec![AdvancedCommerceSubscriptionChangeMetadataItem::new(
        "AD_FREE_1M".to_string(),
        AdvancedCommerceEffective::NextBillCycle,
    )
    .with_sku("AD_FREE_1M_V2".to_string())])
    .with_storefront("USA".to_string())
    .with_tax_code("C003-00-1".to_string());

    assert_codable_round_trips(&request);

    let response = client
        .change_subscription_metadata("4124214", &request)
        .await
        .unwrap();

    assert_eq!("signed_renewal_info", response.signed_renewal_info);
    assert_eq!("signed_transaction_info", response.signed_transaction_info);
}

#[tokio::test]
async fn test_migrate_subscription() {
    let client = advanced_commerce_api_client_with_body_from_file(
        "tests/resources/models/advancedCommerceSubscriptionMigrateResponse.json",
        StatusCode::OK,
        Some(Box::new(|req, body| {
            assert_eq!(&Method::POST, req.method());
            assert_eq!(
                "https://local-testing-base-url/advancedCommerce/v1/subscription/migrate/4124214",
                req.uri().to_string()
            );

            let decoded_json: Value = serde_json::from_slice(body).unwrap();
            assert_eq!(
                "932C6903-0AB8-4469-9F21-015F6FAB013C",
                decoded_json["requestInfo"]["requestReferenceId"]
                    .as_str()
                    .unwrap()
                    .to_uppercase()
            );
            let descriptors = &decoded_json["descriptors"];
            assert_eq!(
                "Remove ads and unlock advanced features.",
                descriptors["description"]
            );
            assert_eq!("Ad-free package", descriptors["displayName"]);
            let item = &decoded_json["items"][0];
            assert_eq!("AD_FREE_1M", item["SKU"]);
            assert_eq!("Remove ads for the service.", item["description"]);
            assert_eq!("Ad-free monthly plan", item["displayName"]);
            assert_eq!("com.example.base", decoded_json["targetProductId"]);
            assert_eq!("C003-00-1", decoded_json["taxCode"]);
            assert_eq!("USA", decoded_json["storefront"]);
        })),
    );

    let request = AdvancedCommerceSubscriptionMigrateRequest::new(
        Uuid::parse_str("932c6903-0ab8-4469-9f21-015f6fab013c").unwrap(),
        AdvancedCommerceSubscriptionMigrateDescriptors::new(
            "Remove ads and unlock advanced features.".to_string(),
            "Ad-free package".to_string(),
        ),
        vec![AdvancedCommerceSubscriptionMigrateItem::new(
            "AD_FREE_1M".to_string(),
            "Remove ads for the service.".to_string(),
            "Ad-free monthly plan".to_string(),
        )],
        "com.example.base".to_string(),
        "C003-00-1".to_string(),
    )
    .unwrap()
    .with_storefront("USA".to_string());

    assert_codable_round_trips(&request);

    let response = client
        .migrate_subscription("4124214", &request)
        .await
        .unwrap();

    assert_eq!("signed_renewal_info_value", response.signed_renewal_info);
    assert_eq!(
        "signed_transaction_info_value",
        response.signed_transaction_info
    );
}

#[test]
fn test_xcode_environment_is_not_supported() {
    let mock_transport = MockTransport::new(String::new(), StatusCode::OK, None);

    let result = AdvancedCommerceApiClient::new(
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
    let mock_transport = MockTransport::new(String::new(), StatusCode::OK, None);

    let result = AdvancedCommerceApiClient::new(
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
    let mock_transport = MockTransport::new(String::new(), StatusCode::OK, None);

    let result = AdvancedCommerceApiClient::new(
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
) -> AdvancedCommerceApiClient<MockTransport> {
    let body = fs::read_to_string(path).expect("Failed to read file");
    advanced_commerce_api_client(body, status, request_verifier)
}

fn advanced_commerce_api_client(
    body: String,
    status: StatusCode,
    request_verifier: Option<RequestVerifier>,
) -> AdvancedCommerceApiClient<MockTransport> {
    let key = fs::read("tests/resources/certs/testSigningKey.p8").expect("Failed to read file");

    let mock_transport = MockTransport::new(body, status, request_verifier);

    AdvancedCommerceApiClient::new(
        key,
        "keyId",
        "issuerId",
        "com.example",
        Environment::LocalTesting,
        mock_transport,
    )
    .expect("Error creating advanced commerce client")
}
