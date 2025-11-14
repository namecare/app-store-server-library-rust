mod common;
use common::transport_mock::{MockTransport, RequestVerifier};
use app_store_server_library::api_client::api::retention_messaging_api::RetentionMessagingApiClient;
use app_store_server_library::primitives::environment::Environment;
use app_store_server_library::primitives::retention_messaging::default_configuration_request::DefaultConfigurationRequest;
use app_store_server_library::primitives::retention_messaging::image_state::ImageState;
use app_store_server_library::primitives::retention_messaging::message_state::MessageState;
use app_store_server_library::primitives::retention_messaging::upload_message_image::UploadMessageImage;
use app_store_server_library::primitives::retention_messaging::upload_message_request_body::UploadMessageRequestBody;
use http::{Method, StatusCode};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use uuid::Uuid;

#[tokio::test]
async fn test_upload_image() {
    let client = retention_messaging_api_client(
        "".to_string(),
        StatusCode::OK,
        Some(Box::new(|req, body| {
            assert_eq!(&Method::PUT, req.method());
            assert_eq!(
                "https://local-testing-base-url/inApps/v1/messaging/image/a1b2c3d4-e5f6-7890-a1b2-c3d4e5f67890",
                req.uri().to_string()
            );
            assert_eq!(
                "image/png",
                req.headers()
                    .get("Content-Type")
                    .unwrap()
                    .to_str()
                    .unwrap()
            );
            assert!(body.len() > 0);
            assert_eq!(&vec![1, 2, 3], body);
        })),
    );

    let result = client
        .upload_image(
            Uuid::parse_str("a1b2c3d4-e5f6-7890-a1b2-c3d4e5f67890").unwrap(),
            vec![1, 2, 3],
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_image() {
    let client = retention_messaging_api_client(
        "".to_string(),
        StatusCode::OK,
        Some(Box::new(|req, _body| {
            assert_eq!(&Method::DELETE, req.method());
            assert_eq!(
                "https://local-testing-base-url/inApps/v1/messaging/image/a1b2c3d4-e5f6-7890-a1b2-c3d4e5f67890",
                req.uri().to_string()
            );
        })),
    );

    let result = client
        .delete_image(Uuid::parse_str("a1b2c3d4-e5f6-7890-a1b2-c3d4e5f67890").unwrap())
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_image_list() {
    let client = retention_messaging_api_client_with_body_from_file(
        "tests/resources/models/getImageListResponse.json",
        StatusCode::OK,
        Some(Box::new(|req, _body| {
            assert_eq!(&Method::GET, req.method());
            assert_eq!(
                "https://local-testing-base-url/inApps/v1/messaging/image/list",
                req.uri().to_string()
            );
        })),
    );

    let result = client.image_list().await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(1, response.image_identifiers.as_ref().unwrap().len());
    assert_eq!(
        Some(Uuid::parse_str("a1b2c3d4-e5f6-7890-a1b2-c3d4e5f67890").unwrap()),
        response.image_identifiers.as_ref().unwrap()[0].image_identifier
    );
    assert_eq!(
        Some(ImageState::Approved),
        response.image_identifiers.as_ref().unwrap()[0].image_state
    );
}

#[tokio::test]
async fn test_upload_message() {
    let client = retention_messaging_api_client(
        "".to_string(),
        StatusCode::OK,
        Some(Box::new(|req, body| {
            assert_eq!(&Method::PUT, req.method());
            assert_eq!(
                "https://local-testing-base-url/inApps/v1/messaging/message/a1b2c3d4-e5f6-7890-a1b2-c3d4e5f67890",
                req.uri().to_string()
            );
            let decoded_json: HashMap<String, Value> = serde_json::from_slice(body).unwrap();
            assert_eq!("Header text", decoded_json["header"].as_str().unwrap());
            assert_eq!("Body text", decoded_json["body"].as_str().unwrap());
        })),
    );

    let upload_message_request_body = UploadMessageRequestBody::new(
        "Header text".to_string(),
        "Body text".to_string(),
        None,
    )
    .unwrap();
    let result = client
        .upload_message(
            Uuid::parse_str("a1b2c3d4-e5f6-7890-a1b2-c3d4e5f67890").unwrap(),
            &upload_message_request_body,
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_upload_message_with_image() {
    let client = retention_messaging_api_client(
        "".to_string(),
        StatusCode::OK,
        Some(Box::new(|req, body| {
            assert_eq!(&Method::PUT, req.method());
            assert_eq!(
                "https://local-testing-base-url/inApps/v1/messaging/message/a1b2c3d4-e5f6-7890-a1b2-c3d4e5f67890",
                req.uri().to_string()
            );
            let decoded_json: HashMap<String, Value> = serde_json::from_slice(body).unwrap();
            assert_eq!("Header text", decoded_json["header"].as_str().unwrap());
            assert_eq!("Body text", decoded_json["body"].as_str().unwrap());
            let image = decoded_json["image"].as_object().unwrap();
            assert_eq!(
                "b2c3d4e5-f6a7-8901-b2c3-d4e5f6a78901",
                image["imageIdentifier"].as_str().unwrap()
            );
            assert_eq!("Alt text", image["altText"].as_str().unwrap());
        })),
    );

    let image = UploadMessageImage::new(
        Uuid::parse_str("b2c3d4e5-f6a7-8901-b2c3-d4e5f6a78901").unwrap(),
        "Alt text".to_string(),
    )
    .unwrap();
    let upload_message_request_body = UploadMessageRequestBody::new(
        "Header text".to_string(),
        "Body text".to_string(),
        Some(image),
    )
    .unwrap();
    let result = client
        .upload_message(
            Uuid::parse_str("a1b2c3d4-e5f6-7890-a1b2-c3d4e5f67890").unwrap(),
            &upload_message_request_body,
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_message() {
    let client = retention_messaging_api_client(
        "".to_string(),
        StatusCode::OK,
        Some(Box::new(|req, _body| {
            assert_eq!(&Method::DELETE, req.method());
            assert_eq!(
                "https://local-testing-base-url/inApps/v1/messaging/message/a1b2c3d4-e5f6-7890-a1b2-c3d4e5f67890",
                req.uri().to_string()
            );
        })),
    );

    let result = client
        .delete_message(Uuid::parse_str("a1b2c3d4-e5f6-7890-a1b2-c3d4e5f67890").unwrap())
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_message_list() {
    let client = retention_messaging_api_client_with_body_from_file(
        "tests/resources/models/getMessageListResponse.json",
        StatusCode::OK,
        Some(Box::new(|req, _body| {
            assert_eq!(&Method::GET, req.method());
            assert_eq!(
                "https://local-testing-base-url/inApps/v1/messaging/message/list",
                req.uri().to_string()
            );
        })),
    );

    let result = client.message_list().await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(1, response.message_identifiers.as_ref().unwrap().len());
    assert_eq!(
        Some(Uuid::parse_str("a1b2c3d4-e5f6-7890-a1b2-c3d4e5f67890").unwrap()),
        response.message_identifiers.as_ref().unwrap()[0].message_identifier
    );
    assert_eq!(
        Some(MessageState::Approved),
        response.message_identifiers.as_ref().unwrap()[0].message_state
    );
}

#[tokio::test]
async fn test_set_default_configuration() {
    let client = retention_messaging_api_client(
        "".to_string(),
        StatusCode::OK,
        Some(Box::new(|req, body| {
            assert_eq!(&Method::PUT, req.method());
            assert_eq!(
                "https://local-testing-base-url/inApps/v1/messaging/default/com.example.product/en-US",
                req.uri().to_string()
            );
            let decoded_json: HashMap<String, Value> = serde_json::from_slice(body).unwrap();
            assert_eq!(
                "a1b2c3d4-e5f6-7890-a1b2-c3d4e5f67890",
                decoded_json["messageIdentifier"].as_str().unwrap()
            );
        })),
    );

    let default_configuration_request = DefaultConfigurationRequest {
        message_identifier: Some(Uuid::parse_str("a1b2c3d4-e5f6-7890-a1b2-c3d4e5f67890").unwrap()),
    };
    let result = client
        .set_default_configuration("com.example.product", "en-US", &default_configuration_request)
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_default_configuration() {
    let client = retention_messaging_api_client(
        "".to_string(),
        StatusCode::OK,
        Some(Box::new(|req, _body| {
            assert_eq!(&Method::DELETE, req.method());
            assert_eq!(
                "https://local-testing-base-url/inApps/v1/messaging/default/com.example.product/en-US",
                req.uri().to_string()
            );
        })),
    );

    let result = client
        .delete_default_configuration("com.example.product", "en-US")
        .await;

    assert!(result.is_ok());
}

fn retention_messaging_api_client_with_body_from_file(
    path: &str,
    status: StatusCode,
    request_verifier: Option<RequestVerifier>,
) -> RetentionMessagingApiClient<MockTransport> {
    let body = fs::read_to_string(path).expect("Failed to read file");
    retention_messaging_api_client(body, status, request_verifier)
}

fn retention_messaging_api_client(
    body: String,
    status: StatusCode,
    request_verifier: Option<RequestVerifier>,
) -> RetentionMessagingApiClient<MockTransport> {
    let key = fs::read("tests/resources/certs/testSigningKey.p8").expect("Failed to read file");

    let mock_transport = MockTransport::new(body, status, request_verifier);

    RetentionMessagingApiClient::new(
        key,
        "keyId",
        "issuerId",
        "com.example",
        Environment::LocalTesting,
        mock_transport,
    )
    .expect("Error creating retention messaging client")
}