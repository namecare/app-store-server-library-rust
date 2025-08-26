mod common;
use common::{MockTransport, RequestVerifier};
use app_store_server_library::primitives::account_tenure::AccountTenure;
use app_store_server_library::primitives::consumption_request::ConsumptionRequest;
use app_store_server_library::primitives::consumption_status::ConsumptionStatus;
use app_store_server_library::primitives::delivery_status::DeliveryStatus;
use app_store_server_library::primitives::environment::Environment;
use app_store_server_library::primitives::extend_reason_code::ExtendReasonCode;
use app_store_server_library::primitives::extend_renewal_date_request::ExtendRenewalDateRequest;
use app_store_server_library::primitives::in_app_ownership_type::InAppOwnershipType;
use app_store_server_library::primitives::last_transactions_item::LastTransactionsItem;
use app_store_server_library::primitives::lifetime_dollars_purchased::LifetimeDollarsPurchased;
use app_store_server_library::primitives::lifetime_dollars_refunded::LifetimeDollarsRefunded;
use app_store_server_library::primitives::mass_extend_renewal_date_request::MassExtendRenewalDateRequest;
use app_store_server_library::primitives::notification_history_request::NotificationHistoryRequest;
use app_store_server_library::primitives::notification_history_response_item::NotificationHistoryResponseItem;
use app_store_server_library::primitives::notification_type_v2::NotificationTypeV2;
use app_store_server_library::primitives::order_lookup_status::OrderLookupStatus;
use app_store_server_library::primitives::platform::Platform;
use app_store_server_library::primitives::play_time::PlayTime;
use app_store_server_library::primitives::refund_preference::RefundPreference;
use app_store_server_library::primitives::send_attempt_item::SendAttemptItem;
use app_store_server_library::primitives::send_attempt_result::SendAttemptResult;
use app_store_server_library::primitives::status::Status;
use app_store_server_library::primitives::subscription_group_identifier_item::SubscriptionGroupIdentifierItem;
use app_store_server_library::primitives::subtype::Subtype;
use app_store_server_library::primitives::transaction_history_request::{
    Order, ProductType, TransactionHistoryRequest,
};
use app_store_server_library::primitives::update_app_account_token_request::UpdateAppAccountTokenRequest;
use app_store_server_library::primitives::user_status::UserStatus;
use base64::prelude::BASE64_STANDARD_NO_PAD;
use base64::Engine;
use chrono::DateTime;
use http::{Method, StatusCode};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use uuid::Uuid;
use app_store_server_library::api_client::api::app_store_server_api::api_error_code::APIErrorCode;
use app_store_server_library::api_client::api::app_store_server_api::{AppStoreServerAPIClient, GetTransactionHistoryVersion};
use app_store_server_library::api_client::error::ConfigurationError;

#[tokio::test]
async fn test_extend_renewal_date_for_all_active_subscribers() {
    let client = app_store_server_api_client_with_body_from_file(
        "tests/resources/models/extendRenewalDateForAllActiveSubscribersResponse.json",
        StatusCode::OK,
        Some(Box::new(|req, body| {
            assert_eq!(&Method::POST, req.method());
            assert_eq!(
                "https://local-testing-base-url/inApps/v1/subscriptions/extend/mass",
                req.uri().to_string()
            );

            let decoded_json: HashMap<&str, Value> = serde_json::from_slice(body).unwrap();
            assert_eq!(
                45,
                decoded_json
                    .get("extendByDays")
                    .unwrap()
                    .as_u64()
                    .unwrap()
            );
            assert_eq!(
                1,
                decoded_json
                    .get("extendReasonCode")
                    .unwrap()
                    .as_u64()
                    .unwrap()
            );
            assert_eq!(
                "fdf964a4-233b-486c-aac1-97d8d52688ac",
                decoded_json
                    .get("requestIdentifier")
                    .unwrap()
                    .as_str()
                    .unwrap()
            );
            assert_eq!(
                vec!["USA", "MEX"],
                decoded_json
                    .get("storefrontCountryCodes")
                    .unwrap()
                    .as_array()
                    .unwrap()
                    .to_vec()
            );
            assert_eq!(
                "com.example.productId",
                decoded_json
                    .get("productId")
                    .unwrap()
                    .as_str()
                    .unwrap()
            );
        })),
    );

    let dto = MassExtendRenewalDateRequest {
        extend_by_days: 45,
        extend_reason_code: ExtendReasonCode::CustomerSatisfaction,
        request_identifier: "fdf964a4-233b-486c-aac1-97d8d52688ac".to_string(),
        storefront_country_codes: vec!["USA".to_string(), "MEX".to_string()],
        product_id: "com.example.productId".to_string(),
    };

    let response = client
        .extend_renewal_date_for_all_active_subscribers(&dto)
        .await
        .unwrap();
    assert_eq!(
        "758883e8-151b-47b7-abd0-60c4d804c2f5",
        response
            .request_identifier
            .unwrap()
            .as_str()
    );
}

#[tokio::test]
async fn test_extend_subscription_renewal_date() {
    let client = app_store_server_api_client_with_body_from_file(
        "tests/resources/models/extendSubscriptionRenewalDateResponse.json",
        StatusCode::OK,
        Some(Box::new(|req, body| {
            assert_eq!(&Method::PUT, req.method());
            assert_eq!(
                "https://local-testing-base-url/inApps/v1/subscriptions/extend/4124214",
                req.uri().to_string()
            );

            let decoded_json: HashMap<&str, Value> = serde_json::from_slice(body).unwrap();
            assert_eq!(
                45,
                decoded_json
                    .get("extendByDays")
                    .unwrap()
                    .as_u64()
                    .unwrap()
            );
            assert_eq!(
                1,
                decoded_json
                    .get("extendReasonCode")
                    .unwrap()
                    .as_u64()
                    .unwrap()
            );
            assert_eq!(
                "fdf964a4-233b-486c-aac1-97d8d52688ac",
                decoded_json
                    .get("requestIdentifier")
                    .unwrap()
                    .as_str()
                    .unwrap()
            );
        })),
    );

    let extend_renewal_date_request = ExtendRenewalDateRequest {
        extend_by_days: Some(45),
        extend_reason_code: Some(ExtendReasonCode::CustomerSatisfaction),
        request_identifier: Some("fdf964a4-233b-486c-aac1-97d8d52688ac".to_string()),
    };

    let response = client
        .extend_subscription_renewal_date("4124214", &extend_renewal_date_request)
        .await
        .unwrap();
    assert_eq!(
        "2312412",
        response
            .original_transaction_id
            .unwrap()
            .as_str()
    );
    assert_eq!(
        "9993",
        response
            .web_order_line_item_id
            .unwrap()
            .as_str()
    );
    assert_eq!(true, response.success.unwrap());
    assert_eq!(
        1698148900,
        response
            .effective_date
            .unwrap()
            .timestamp()
    );
}

#[tokio::test]
async fn test_get_all_subscription_statuses() {
    let client = app_store_server_api_client_with_body_from_file(
        "tests/resources/models/getAllSubscriptionStatusesResponse.json",
        StatusCode::OK,
        Some(Box::new(|req, body| {
            assert_eq!(&Method::GET, req.method());
            assert_eq!(
                "https://local-testing-base-url/inApps/v1/subscriptions/4321?status=2&status=1",
                req.uri().to_string()
            );
            assert!(body.is_empty(), "GET request should have empty body");
        })),
    );

    let statuses = vec![Status::Expired, Status::Active];
    let response = client
        .get_all_subscription_statuses("4321", Some(&statuses))
        .await
        .unwrap();

    assert_eq!(Environment::LocalTesting, response.environment.unwrap());
    assert_eq!("com.example", response.bundle_id.as_str());
    assert_eq!(5454545, response.app_apple_id.unwrap());

    let item = SubscriptionGroupIdentifierItem {
        subscription_group_identifier: Some("sub_group_one".to_string()),
        last_transactions: Some(vec![
            LastTransactionsItem {
                status: Status::Active.into(),
                original_transaction_id: "3749183".to_string().into(),
                signed_transaction_info: "signed_transaction_one"
                    .to_string()
                    .into(),
                signed_renewal_info: "signed_renewal_one".to_string().into(),
            },
            LastTransactionsItem {
                status: Status::Revoked.into(),
                original_transaction_id: "5314314134".to_string().into(),
                signed_transaction_info: "signed_transaction_two"
                    .to_string()
                    .into(),
                signed_renewal_info: "signed_renewal_two".to_string().into(),
            },
        ]),
    };

    let second_item = SubscriptionGroupIdentifierItem {
        subscription_group_identifier: "sub_group_two".to_string().into(),
        last_transactions: vec![LastTransactionsItem {
            status: Status::Expired.into(),
            original_transaction_id: "3413453".to_string().into(),
            signed_transaction_info: "signed_transaction_three"
                .to_string()
                .into(),
            signed_renewal_info: "signed_renewal_three"
                .to_string()
                .into(),
        }]
        .into(),
    };

    assert_eq!(vec![item, second_item], response.data);
}

#[tokio::test]
async fn test_get_refund_history() {
    let client = app_store_server_api_client_with_body_from_file(
        "tests/resources/models/getRefundHistoryResponse.json",
        StatusCode::OK,
        Some(Box::new(|req, body| {
            assert_eq!(&Method::GET, req.method());
            assert_eq!(
                "https://local-testing-base-url/inApps/v2/refund/lookup/555555?revision=revision_input",
                req.uri().to_string()
            );
            assert!(body.is_empty(), "GET request should have empty body");
        })),
    );

    let response = client
        .get_refund_history("555555", "revision_input")
        .await
        .unwrap();

    assert_eq!(
        vec!["signed_transaction_one", "signed_transaction_two"],
        response.signed_transactions
    );
    assert_eq!("revision_output", response.revision);
    assert_eq!(true, response.has_more);
}

#[tokio::test]
async fn test_get_status_of_subscription_renewal_date_extensions() {
    let client = app_store_server_api_client_with_body_from_file(
        "tests/resources/models/getStatusOfSubscriptionRenewalDateExtensionsResponse.json",
        StatusCode::OK,
        Some(Box::new(|req, body| {
            assert_eq!(&Method::GET, req.method());
            assert_eq!(
                "https://local-testing-base-url/inApps/v1/subscriptions/extend/mass/20fba8a0-2b80-4a7d-a17f-85c1854727f8/com.example.product",
                req.uri().to_string()
            );
            assert!(body.is_empty(), "GET request should have empty body");
        })),
    );

    let response = client
        .get_status_of_subscription_renewal_date_extensions(
            "com.example.product",
            "20fba8a0-2b80-4a7d-a17f-85c1854727f8",
        )
        .await
        .unwrap();

    assert_eq!(
        "20fba8a0-2b80-4a7d-a17f-85c1854727f8",
        response
            .request_identifier
            .unwrap()
            .as_str()
    );
    assert_eq!(true, response.complete.unwrap());
    assert_eq!(
        1698148900,
        response
            .complete_date
            .unwrap()
            .timestamp()
    );
    assert_eq!(30, response.succeeded_count.unwrap());
    assert_eq!(2, response.failed_count.unwrap());
}

#[tokio::test]
async fn test_get_test_notification_status() {
    let client = app_store_server_api_client_with_body_from_file(
        "tests/resources/models/getTestNotificationStatusResponse.json",
        StatusCode::OK,
        Some(Box::new(|req, body| {
            assert_eq!(&Method::GET, req.method());
            assert_eq!(
                "https://local-testing-base-url/inApps/v1/notifications/test/8cd2974c-f905-492a-bf9a-b2f47c791d19",
                req.uri().to_string()
            );
            assert!(body.is_empty(), "GET request should have empty body");
        })),
    );

    let response = client
        .get_test_notification_status("8cd2974c-f905-492a-bf9a-b2f47c791d19")
        .await
        .unwrap();
    assert_eq!("signed_payload", response.signed_payload.unwrap());

    let send_attempt_items = vec![
        SendAttemptItem {
            attempt_date: DateTime::from_timestamp(1698148900, 0),
            send_attempt_result: SendAttemptResult::NoResponse.into(),
        },
        SendAttemptItem {
            attempt_date: DateTime::from_timestamp(1698148950, 0),
            send_attempt_result: SendAttemptResult::Success.into(),
        },
    ];
    assert_eq!(send_attempt_items, response.send_attempts.unwrap());
}

#[tokio::test]
async fn test_get_notification_history() {
    let client = app_store_server_api_client_with_body_from_file(
        "tests/resources/models/getNotificationHistoryResponse.json",
        StatusCode::OK,
        Some(Box::new(|req, body| {
            assert_eq!(&Method::POST, req.method());
            assert_eq!(
                "https://local-testing-base-url/inApps/v1/notifications/history?paginationToken=a036bc0e-52b8-4bee-82fc-8c24cb6715d6",
                req.uri().to_string()
            );

            let decoded_json: HashMap<&str, Value> = serde_json::from_slice(body).unwrap();
            assert_eq!(
                1698148900000,
                decoded_json["startDate"]
                    .as_i64()
                    .unwrap()
            );
            assert_eq!(
                1698148950000,
                decoded_json["endDate"]
                    .as_i64()
                    .unwrap()
            );
            assert_eq!(
                "SUBSCRIBED",
                decoded_json["notificationType"]
                    .as_str()
                    .unwrap()
            );
            assert_eq!(
                "INITIAL_BUY",
                decoded_json["notificationSubtype"]
                    .as_str()
                    .unwrap()
            );
            assert_eq!(
                "999733843",
                decoded_json["transactionId"]
                    .as_str()
                    .unwrap()
            );
            assert_eq!(
                true,
                decoded_json["onlyFailures"]
                    .as_bool()
                    .unwrap()
            );
        })),
    );

    let notification_history_request = NotificationHistoryRequest {
        start_date: DateTime::from_timestamp(1698148900, 0),
        end_date: DateTime::from_timestamp(1698148950, 0),
        notification_type: NotificationTypeV2::Subscribed.into(),
        notification_subtype: Subtype::InitialBuy.into(),
        transaction_id: "999733843".to_string().into(),
        only_failures: true.into(),
    };

    let response = client
        .get_notification_history(
            "a036bc0e-52b8-4bee-82fc-8c24cb6715d6",
            &notification_history_request,
        )
        .await
        .unwrap();
    assert_eq!(
        "57715481-805a-4283-8499-1c19b5d6b20a",
        response.pagination_token.unwrap()
    );
    assert_eq!(true, response.has_more.unwrap());

    let expected_notification_history = vec![
        NotificationHistoryResponseItem {
            signed_payload: "signed_payload_one".to_string().into(),
            send_attempts: vec![
                SendAttemptItem {
                    attempt_date: DateTime::from_timestamp(1698148900, 0),
                    send_attempt_result: SendAttemptResult::NoResponse.into(),
                },
                SendAttemptItem {
                    attempt_date: DateTime::from_timestamp(1698148950, 0),
                    send_attempt_result: SendAttemptResult::Success.into(),
                },
            ]
            .into(),
        },
        NotificationHistoryResponseItem {
            signed_payload: "signed_payload_two".to_string().into(),
            send_attempts: vec![SendAttemptItem {
                attempt_date: DateTime::from_timestamp(1698148800, 0),
                send_attempt_result: SendAttemptResult::CircularRedirect.into(),
            }]
            .into(),
        },
    ];
    assert_eq!(
        expected_notification_history,
        response.notification_history.unwrap()
    );
}

#[tokio::test]
async fn test_get_transaction_history_v1() {
    let client = app_store_server_api_client_with_body_from_file(
        "tests/resources/models/transactionHistoryResponse.json",
        StatusCode::OK,
        Some(Box::new(|req, body| {
            assert_eq!(&Method::GET, req.method());
            assert_eq!("/inApps/v1/history/1234", req.uri().path());
            assert!(body.is_empty(), "GET request should have empty body");
        })),
    );

    let request = TransactionHistoryRequest {
        start_date: DateTime::from_timestamp(123, 455000000),
        end_date: DateTime::from_timestamp(123, 456000000),
        product_ids: Some(vec![
            "com.example.1".to_string(),
            "com.example.2".to_string(),
        ]),
        product_types: Some(vec![ProductType::Consumable, ProductType::AutoRenewable]),
        sort: Some(Order::Ascending),
        subscription_group_identifiers: Some(vec![
            "sub_group_id".to_string(),
            "sub_group_id_2".to_string(),
        ]),
        in_app_ownership_type: Some(InAppOwnershipType::FamilyShared),
        revoked: Some(false),
    };

    let response = client
        .get_transaction_history("1234", Some("revision_input"), request)
        .await
        .unwrap();

    assert_eq!("revision_output", response.revision.unwrap());
    assert_eq!(response.has_more, Some(true));
    assert_eq!("com.example", response.bundle_id.unwrap().as_str());
    assert_eq!(323232, response.app_apple_id.unwrap());
    assert_eq!(Environment::LocalTesting, response.environment.unwrap());
    assert_eq!(
        vec!["signed_transaction_value", "signed_transaction_value2"],
        response.signed_transactions.unwrap()
    );
}

#[tokio::test]
async fn test_get_transaction_history_v2() {
    let client = app_store_server_api_client_with_body_from_file(
        "tests/resources/models/transactionHistoryResponse.json",
        StatusCode::OK,
        Some(Box::new(|req, body| {
            assert_eq!(&Method::GET, req.method());
            let url = req.uri();
            assert_eq!("/inApps/v2/history/1234", url.path());

            let query = url.query().unwrap_or("");
            let params: HashMap<String, Vec<String>> = query
                .split('&')
                .filter(|s| !s.is_empty())
                .fold(HashMap::new(), |mut acc, pair| {
                    if let Some((k, v)) = pair.split_once('=') {
                        acc.entry(k.to_string())
                            .or_insert_with(Vec::new)
                            .push(v.to_string());
                    }
                    acc
                });

            assert_eq!(
                vec!["revision_input".to_string()],
                *params.get("revision").unwrap()
            );
            assert_eq!(vec!["123455"], *params.get("startDate").unwrap());
            assert_eq!(vec!["123456"], *params.get("endDate").unwrap());
            assert_eq!(
                vec!["com.example.1", "com.example.2"],
                *params.get("productId").unwrap()
            );
            assert_eq!(
                vec!["CONSUMABLE", "AUTO_RENEWABLE"],
                *params.get("productType").unwrap()
            );
            assert_eq!(vec!["ASCENDING"], *params.get("sort").unwrap());
            assert_eq!(
                vec!["sub_group_id", "sub_group_id_2"],
                *params
                    .get("subscriptionGroupIdentifier")
                    .unwrap()
            );
            assert_eq!(
                vec!["FAMILY_SHARED"],
                *params
                    .get("inAppOwnershipType")
                    .unwrap()
            );
            assert_eq!(vec!["false"], *params.get("revoked").unwrap());

            assert!(body.is_empty(), "GET request should have empty body");
        })),
    );

    let request = TransactionHistoryRequest {
        start_date: DateTime::from_timestamp(123, 455000000),
        end_date: DateTime::from_timestamp(123, 456000000),
        product_ids: Some(vec![
            "com.example.1".to_string(),
            "com.example.2".to_string(),
        ]),
        product_types: Some(vec![ProductType::Consumable, ProductType::AutoRenewable]),
        sort: Some(Order::Ascending),
        subscription_group_identifiers: Some(vec![
            "sub_group_id".to_string(),
            "sub_group_id_2".to_string(),
        ]),
        in_app_ownership_type: Some(InAppOwnershipType::FamilyShared),
        revoked: Some(false),
    };

    let response = client
        .get_transaction_history_with_version(
            "1234",
            Some("revision_input"),
            &request,
            GetTransactionHistoryVersion::V2,
        )
        .await
        .unwrap();

    assert_eq!("revision_output", response.revision.unwrap());
    assert_eq!(response.has_more, Some(true));
    assert_eq!("com.example", response.bundle_id.unwrap().as_str());
    assert_eq!(323232, response.app_apple_id.unwrap());
    assert_eq!(Environment::LocalTesting, response.environment.unwrap());
    assert_eq!(
        vec!["signed_transaction_value", "signed_transaction_value2"],
        response.signed_transactions.unwrap()
    );
}

#[tokio::test]
async fn test_get_transaction_info() {
    let client = app_store_server_api_client_with_body_from_file(
        "tests/resources/models/transactionInfoResponse.json",
        StatusCode::OK,
        Some(Box::new(|req, body| {
            assert_eq!(&Method::GET, req.method());
            assert_eq!(
                "https://local-testing-base-url/inApps/v1/transactions/1234",
                req.uri().to_string()
            );
            assert!(body.is_empty(), "GET request should have empty body");
        })),
    );

    let response = client
        .get_transaction_info("1234")
        .await
        .unwrap();
    assert_eq!(
        "signed_transaction_info_value",
        response
            .signed_transaction_info
            .unwrap()
    );
}

#[tokio::test]
async fn test_look_up_order_id() {
    let client = app_store_server_api_client_with_body_from_file(
        "tests/resources/models/lookupOrderIdResponse.json",
        StatusCode::OK,
        Some(Box::new(|req, body| {
            assert_eq!(&Method::GET, req.method());
            assert_eq!(
                "https://local-testing-base-url/inApps/v1/lookup/W002182",
                req.uri().to_string()
            );
            // GET request should have empty body
            assert!(body.is_empty(), "GET request should have empty body");
        })),
    );

    let response = client
        .look_up_order_id("W002182")
        .await
        .unwrap();
    assert_eq!(OrderLookupStatus::Invalid, response.status);
    assert_eq!(
        vec!["signed_transaction_one", "signed_transaction_two"],
        response.signed_transactions
    );
}

#[tokio::test]
async fn test_request_test_notification() {
    let client = app_store_server_api_client_with_body_from_file(
        "tests/resources/models/requestTestNotificationResponse.json",
        StatusCode::OK,
        Some(Box::new(|req, body| {
            assert_eq!(&Method::POST, req.method());
            assert_eq!(
                "https://local-testing-base-url/inApps/v1/notifications/test",
                req.uri().to_string()
            );
            // POST request with no parameters should have empty body
            assert!(
                body.is_empty(),
                "POST request should have empty body for test notification"
            );
        })),
    );

    let response = client
        .request_test_notification()
        .await
        .unwrap();
    assert_eq!(
        "ce3af791-365e-4c60-841b-1674b43c1609",
        response
            .test_notification_token
            .unwrap()
    );
}

#[tokio::test]
async fn test_send_consumption_data() {
    let client = app_store_server_api_client(
        "".into(),
        StatusCode::OK,
        Some(Box::new(|req, body| {
            assert_eq!(&Method::PUT, req.method());
            assert_eq!(
                "https://local-testing-base-url/inApps/v1/transactions/consumption/49571273",
                req.uri().to_string()
            );
            assert_eq!(
                "application/json",
                req.headers()
                    .get("Content-Type")
                    .unwrap()
                    .to_str()
                    .unwrap()
            );
            let decoded_json: HashMap<String, Value> = serde_json::from_slice(body).unwrap();
            assert_eq!(
                true,
                decoded_json["customerConsented"]
                    .as_bool()
                    .unwrap()
            );
            assert_eq!(
                1,
                decoded_json["consumptionStatus"]
                    .as_i64()
                    .unwrap()
            );
            assert_eq!(
                2,
                decoded_json["platform"]
                    .as_i64()
                    .unwrap()
            );
            assert_eq!(
                false,
                decoded_json["sampleContentProvided"]
                    .as_bool()
                    .unwrap()
            );
            assert_eq!(
                3,
                decoded_json["deliveryStatus"]
                    .as_i64()
                    .unwrap()
            );
            assert_eq!(
                "7389A31A-FB6D-4569-A2A6-DB7D85D84813"
                    .to_lowercase()
                    .as_str(),
                decoded_json["appAccountToken"]
                    .as_str()
                    .unwrap()
            );
            assert_eq!(
                4,
                decoded_json["accountTenure"]
                    .as_i64()
                    .unwrap()
            );
            assert_eq!(
                5,
                decoded_json["playTime"]
                    .as_i64()
                    .unwrap()
            );
            assert_eq!(
                6,
                decoded_json["lifetimeDollarsRefunded"]
                    .as_i64()
                    .unwrap()
            );
            assert_eq!(
                7,
                decoded_json["lifetimeDollarsPurchased"]
                    .as_i64()
                    .unwrap()
            );
            assert_eq!(
                4,
                decoded_json["userStatus"]
                    .as_i64()
                    .unwrap()
            );
            assert_eq!(
                3,
                decoded_json["refundPreference"]
                    .as_i64()
                    .unwrap()
            );
        })),
    );

    let consumption_request = ConsumptionRequest {
        customer_consented: true.into(),
        consumption_status: ConsumptionStatus::NotConsumed.into(),
        platform: Platform::NonApple.into(),
        sample_content_provided: false.into(),
        delivery_status: DeliveryStatus::DidNotDeliverDueToServerOutage.into(),
        app_account_token: Some(Uuid::parse_str("7389a31a-fb6d-4569-a2a6-db7d85d84813").unwrap()),
        account_tenure: AccountTenure::ThirtyDaysToNinetyDays.into(),
        play_time: PlayTime::OneDayToFourDays.into(),
        lifetime_dollars_refunded:
            LifetimeDollarsRefunded::OneThousandDollarsToOneThousandNineHundredNinetyNineDollarsAndNinetyNineCents
                .into(),
        lifetime_dollars_purchased: LifetimeDollarsPurchased::TwoThousandDollarsOrGreater.into(),
        user_status: UserStatus::LimitedAccess.into(),
        refund_preference: RefundPreference::NoPreference.into(),
    };

    let _ = client
        .send_consumption_data("49571273", &consumption_request)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_set_app_account_token() {
    let client = app_store_server_api_client(
        "".to_string(),
        StatusCode::OK,
        Some(Box::new(|req, body| {
            assert_eq!(&Method::PUT, req.method());
            assert_eq!(
                "https://local-testing-base-url/inApps/v1/transactions/555555/appAccountToken",
                req.uri().to_string()
            );

            let decoded_json: HashMap<&str, Value> = serde_json::from_slice(body).unwrap();
            assert_eq!(
                "550e8400-e29b-41d4-a716-446655440000",
                decoded_json["appAccountToken"]
                    .as_str()
                    .unwrap()
            );
        })),
    );

    let token = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
    let request = UpdateAppAccountTokenRequest::new(token);

    let _ = client
        .set_app_account_token("555555", &request)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_invalid_app_account_token_uuid_error() {
    let client = app_store_server_api_client_with_body_from_file(
        "tests/resources/models/invalidAppAccountTokenUUIDError.json",
        StatusCode::BAD_REQUEST,
        None,
    );

    let token = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
    let request = UpdateAppAccountTokenRequest::new(token);
    let result = client
        .set_app_account_token("555555", &request)
        .await;

    match result {
        Ok(_) => {
            assert!(false, "Unexpected response type");
        }
        Err(error) => {
            assert_eq!(400, error.http_status_code);
            assert_eq!(
                APIErrorCode::InvalidAppAccountTokenUUID,
                error.api_error.unwrap()
            );
            assert_eq!(Some(4000183), error.error_code);
            assert_eq!(
                "Invalid request. The app account token field must be a valid UUID.",
                error.error_message.unwrap()
            );
        }
    }
}

#[tokio::test]
async fn test_family_transaction_not_supported_error() {
    let client = app_store_server_api_client_with_body_from_file(
        "tests/resources/models/familyTransactionNotSupportedError.json",
        StatusCode::BAD_REQUEST,
        None,
    );

    let token = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
    let request = UpdateAppAccountTokenRequest::new(token);
    let result = client
        .set_app_account_token("555555", &request)
        .await;

    match result {
        Ok(_) => {
            assert!(false, "Unexpected response type");
        }
        Err(error) => {
            assert_eq!(400, error.http_status_code);
            assert_eq!(
                APIErrorCode::FamilyTransactionNotSupported,
                error.api_error.unwrap()
            );
            assert_eq!(Some(4000185), error.error_code);
            assert_eq!(
                "Invalid request. Family Sharing transactions aren't supported by this endpoint.",
                error.error_message.unwrap()
            );
        }
    }
}

#[tokio::test]
async fn test_transaction_id_not_original_transaction_id_error() {
    let client = app_store_server_api_client_with_body_from_file(
        "tests/resources/models/transactionIdNotOriginalTransactionId.json",
        StatusCode::BAD_REQUEST,
        None,
    );

    let token = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
    let request = UpdateAppAccountTokenRequest::new(token);
    let result = client
        .set_app_account_token("555555", &request)
        .await;

    match result {
        Ok(_) => {
            assert!(false, "Unexpected response type");
        }
        Err(error) => {
            assert_eq!(400, error.http_status_code);
            assert_eq!(
                APIErrorCode::TransactionIdNotOriginalTransactionId,
                error.api_error.unwrap()
            );
            assert_eq!(Some(4000187), error.error_code);
            assert_eq!(
                "Invalid request. The transaction ID provided is not an original transaction ID.",
                error.error_message.unwrap()
            );
        }
    }
}

#[tokio::test]
async fn test_headers() {
    let client = app_store_server_api_client_with_body_from_file(
        "tests/resources/models/transactionInfoResponse.json",
        StatusCode::OK,
        Some(Box::new(|req, body| {
            let headers = req.headers();
            assert!(headers
                .get("User-Agent")
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with("app-store-server-library/rust"));
            assert_eq!("application/json", headers.get("Accept").unwrap());
            let authorization = headers
                .get("Authorization")
                .unwrap()
                .to_str()
                .unwrap();
            assert!(authorization.starts_with("Bearer "));
            let token_components: Vec<&str> = authorization[7..].split('.').collect();
            let header_data = BASE64_STANDARD_NO_PAD
                .decode(token_components[0])
                .unwrap();
            let payload_data = BASE64_STANDARD_NO_PAD
                .decode(token_components[1])
                .unwrap();
            let header: HashMap<String, Value> = serde_json::from_slice(&header_data).unwrap();
            let payload: HashMap<String, Value> = serde_json::from_slice(&payload_data).unwrap();

            assert_eq!("appstoreconnect-v1", payload["aud"].as_str().unwrap());
            assert_eq!("issuerId", payload["iss"].as_str().unwrap());
            assert_eq!("keyId", header["kid"].as_str().unwrap());
            assert_eq!("com.example", payload["bid"].as_str().unwrap());
            assert_eq!("ES256", header["alg"].as_str().unwrap());
            assert!(body.is_empty(), "GET request should have empty body");
        })),
    );

    let _ = client
        .get_transaction_info("1234")
        .await;
}

#[tokio::test]
async fn test_api_error() {
    let client = app_store_server_api_client_with_body_from_file(
        "tests/resources/models/apiException.json",
        StatusCode::INTERNAL_SERVER_ERROR,
        None,
    );
    let result = client
        .get_transaction_info("1234")
        .await;

    match result {
        Ok(_) => {
            assert!(false, "Unexpected response type");
        }
        Err(error) => {
            assert_eq!(500, error.http_status_code);
            assert_eq!(APIErrorCode::GeneralInternal, error.api_error.unwrap());
            assert_eq!(5000000, error.error_code.unwrap());
            assert_eq!("An unknown error occurred.", error.error_message.unwrap());
        }
    }
}

#[tokio::test]
async fn test_api_too_many_requests() {
    let client = app_store_server_api_client_with_body_from_file(
        "tests/resources/models/apiTooManyRequestsException.json",
        StatusCode::TOO_MANY_REQUESTS,
        None,
    );
    let result = client
        .get_transaction_info("1234")
        .await;

    match result {
        Ok(_) => {
            assert!(false, "Unexpected response type");
        }
        Err(error) => {
            assert_eq!(429, error.http_status_code);
            assert_eq!(APIErrorCode::RateLimitExceeded, error.api_error.unwrap());
            assert_eq!(Some(4290000), error.error_code);
            assert_eq!("Rate limit exceeded.", error.error_message.unwrap());
        }
    }
}

#[tokio::test]
async fn test_api_unknown_error() {
    let client = app_store_server_api_client_with_body_from_file(
        "tests/resources/models/apiUnknownError.json",
        StatusCode::BAD_REQUEST,
        None,
    );
    let result = client
        .get_transaction_info("1234")
        .await;

    match result {
        Ok(_) => {
            assert!(false, "Unexpected response type");
        }
        Err(error) => {
            assert_eq!(400, error.http_status_code);
            assert_eq!(Some(APIErrorCode::Unknown), error.api_error);
            assert_eq!("Testing error.", error.error_message.unwrap());
        }
    }
}

#[tokio::test]
async fn test_decoding_with_malformed_json() {
    let client = app_store_server_api_client_with_body_from_file(
        "tests/resources/models/transactionHistoryResponseWithMalformedAppAppleId.json",
        StatusCode::OK,
        None,
    );

    let request = TransactionHistoryRequest {
        start_date: DateTime::from_timestamp(123, 455000000),
        end_date: DateTime::from_timestamp(123, 456000000),
        product_ids: vec!["com.example.1".to_string(), "com.example.2".to_string()].into(),
        product_types: vec![ProductType::Consumable, ProductType::AutoRenewable].into(),
        sort: Some(Order::Ascending),
        subscription_group_identifiers: vec!["sub_group_id".to_string(), "sub_group_id_2".to_string()].into(),
        in_app_ownership_type: Some(InAppOwnershipType::FamilyShared),
        revoked: Some(false),
    };

    let result = client
        .get_transaction_history("1234", Some("revision_input"), request)
        .await;
    match result {
        Ok(_) => {
            assert!(false, "Unexpected response type");
        }
        Err(error) => {
            assert_eq!(500, error.http_status_code);
            assert_eq!(None, error.api_error);
            assert_eq!(
                "Failed to deserialize response JSON",
                error.error_message.unwrap()
            );
        }
    }
}

#[tokio::test]
async fn test_send_consumption_data_with_null_app_account_token() {
    let client = app_store_server_api_client(
        "".into(),
        StatusCode::OK,
        Some(Box::new(|req, body| {
            assert_eq!(&Method::PUT, req.method());
            assert_eq!(
                "https://local-testing-base-url/inApps/v1/transactions/consumption/49571273",
                req.uri().to_string()
            );
            assert_eq!(
                "application/json",
                req.headers()
                    .get("Content-Type")
                    .unwrap()
                    .to_str()
                    .unwrap()
            );
            let decoded_json: HashMap<String, Value> = serde_json::from_slice(body).unwrap();
            assert_eq!(
                true,
                decoded_json["customerConsented"]
                    .as_bool()
                    .unwrap()
            );
            assert_eq!(
                1,
                decoded_json["consumptionStatus"]
                    .as_i64()
                    .unwrap()
            );
            assert_eq!(
                2,
                decoded_json["platform"]
                    .as_i64()
                    .unwrap()
            );
            assert_eq!(
                false,
                decoded_json["sampleContentProvided"]
                    .as_bool()
                    .unwrap()
            );
            assert_eq!(
                3,
                decoded_json["deliveryStatus"]
                    .as_i64()
                    .unwrap()
            );
            // When app_account_token is None, it should not be included in the JSON at all
            assert!(
                !decoded_json.contains_key("appAccountToken"),
                "appAccountToken field should be omitted when None"
            );
            assert_eq!(
                4,
                decoded_json["accountTenure"]
                    .as_i64()
                    .unwrap()
            );
            assert_eq!(
                5,
                decoded_json["playTime"]
                    .as_i64()
                    .unwrap()
            );
            assert_eq!(
                6,
                decoded_json["lifetimeDollarsRefunded"]
                    .as_i64()
                    .unwrap()
            );
            assert_eq!(
                7,
                decoded_json["lifetimeDollarsPurchased"]
                    .as_i64()
                    .unwrap()
            );
            assert_eq!(
                4,
                decoded_json["userStatus"]
                    .as_i64()
                    .unwrap()
            );
            // refund_preference is also omitted in Swift test when None
        })),
    );

    let consumption_request = ConsumptionRequest {
        customer_consented: true.into(),
        consumption_status: ConsumptionStatus::NotConsumed.into(),
        platform: Platform::NonApple.into(),
        sample_content_provided: false.into(),
        delivery_status: DeliveryStatus::DidNotDeliverDueToServerOutage.into(),
        app_account_token: None,
        account_tenure: AccountTenure::ThirtyDaysToNinetyDays.into(),
        play_time: PlayTime::OneDayToFourDays.into(),
        lifetime_dollars_refunded: LifetimeDollarsRefunded::OneThousandDollarsToOneThousandNineHundredNinetyNineDollarsAndNinetyNineCents.into(),
        lifetime_dollars_purchased: LifetimeDollarsPurchased::TwoThousandDollarsOrGreater.into(),
        user_status: UserStatus::LimitedAccess.into(),
        refund_preference: None.into(),
    };

    let _ = client
        .send_consumption_data("49571273", &consumption_request)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_get_notification_history_with_microsecond_values() {
    let client = app_store_server_api_client_with_body_from_file(
        "tests/resources/models/getNotificationHistoryResponse.json",
        StatusCode::OK,
        Some(Box::new(|_req, body| {
            let decoded_json: HashMap<String, Value> = serde_json::from_slice(body).unwrap();
            // Microseconds should be truncated to milliseconds
            // When 900_000 nanoseconds (0.9ms) is added, it rounds to 1698148900001
            // When 1_000_000 nanoseconds (1ms) is added, it becomes 1698148950001
            assert_eq!(
                1698148900001,
                decoded_json["startDate"]
                    .as_i64()
                    .unwrap()
            );
            assert_eq!(
                1698148950001,
                decoded_json["endDate"]
                    .as_i64()
                    .unwrap()
            );
        })),
    );

    let notification_history_request = NotificationHistoryRequest {
        start_date: DateTime::from_timestamp(1698148900, 900_000).into(), // 900_000 nanoseconds = 0.9 milliseconds
        end_date: DateTime::from_timestamp(1698148950, 1_000_000).into(), // 1_000_000 nanoseconds = 1 millisecond
        notification_type: NotificationTypeV2::Subscribed.into(),
        notification_subtype: Subtype::InitialBuy.into(),
        transaction_id: Some("999733843".to_string()),
        only_failures: Some(true),
    };

    let _ = client
        .get_notification_history(
            "a036bc0e-52b8-4bee-82fc-8c24cb6715d6",
            &notification_history_request,
        )
        .await;
}

#[test]
fn test_xcode_environment_is_rejected() {
    // Xcode environment should not be allowed for AppStoreServerAPIClient
    // This test ensures we don't accidentally allow it in the future
    // Note: In Rust, we handle this at compile time with the Environment enum,
    // but we can test that LocalTesting environment (which maps to Xcode in some contexts) works
    let mock_transport = MockTransport::new(
        String::new(),
        StatusCode::OK,
        None
    );

    let result = AppStoreServerAPIClient::new(
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

    let result = AppStoreServerAPIClient::new(
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

    let result = AppStoreServerAPIClient::new(
        vec![],
        "test_key_id",
        "test_issuer_id",
        "com.test.app",
        Environment::Production,
        mock_transport,
    );

    assert!(result.is_ok());
}

fn app_store_server_api_client_with_body_from_file(
    path: &str,
    status: StatusCode,
    request_verifier: Option<RequestVerifier>,
) -> AppStoreServerAPIClient<MockTransport> {
    let body = fs::read_to_string(path).expect("Failed to read file");
    app_store_server_api_client(body, status, request_verifier)
}

fn app_store_server_api_client(
    body: String,
    status: StatusCode,
    request_verifier: Option<RequestVerifier>,
) -> AppStoreServerAPIClient<MockTransport> {
    let key = fs::read("tests/resources/certs/testSigningKey.p8").expect("Failed to read file");

    let mock_transport = MockTransport::new(body, status, request_verifier);

    AppStoreServerAPIClient::new(
        key,
        "keyId",
        "issuerId",
        "com.example",
        Environment::LocalTesting,
        mock_transport,
    )
    .expect("Error creating app store client")
}
