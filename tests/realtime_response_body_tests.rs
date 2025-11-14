 use app_store_server_library::primitives::retention_messaging::alternate_product::AlternateProduct;
use app_store_server_library::primitives::retention_messaging::message::Message;
use app_store_server_library::primitives::retention_messaging::promotional_offer::PromotionalOffer;
use app_store_server_library::primitives::retention_messaging::promotional_offer_signature_v1::PromotionalOfferSignatureV1;
use app_store_server_library::primitives::retention_messaging::realtime_response_body::RealtimeResponseBody;
use uuid::Uuid;

#[test]
fn test_realtime_response_body_with_message() {
    // Create a RealtimeResponseBody with a Message
    let message_id = Uuid::parse_str("a1b2c3d4-e5f6-7890-a1b2-c3d4e5f67890").unwrap();
    let message = Message {
        message_identifier: Some(message_id),
    };
    let response_body = RealtimeResponseBody {
        message: Some(message),
        alternate_product: None,
        promotional_offer: None,
    };

    // Serialize to JSON
    let json_data = serde_json::to_value(&response_body).unwrap();

    // Validate JSON structure
    assert!(json_data.get("message").is_some(), "JSON should have 'message' field");
    let message_dict = json_data["message"].as_object().unwrap();
    assert!(
        message_dict.contains_key("messageIdentifier"),
        "Message should have 'messageIdentifier' field"
    );
    assert_eq!(
        "a1b2c3d4-e5f6-7890-a1b2-c3d4e5f67890",
        message_dict["messageIdentifier"].as_str().unwrap()
    );
    assert!(json_data.get("alternateProduct").is_none(), "JSON should not have 'alternateProduct' field");
    assert!(json_data.get("promotionalOffer").is_none(), "JSON should not have 'promotionalOffer' field");

    // Deserialize back
    let json_str = serde_json::to_string(&response_body).unwrap();
    let deserialized: RealtimeResponseBody = serde_json::from_str(&json_str).unwrap();

    // Verify
    assert!(deserialized.message.is_some());
    assert_eq!(Some(message_id), deserialized.message.unwrap().message_identifier);
    assert!(deserialized.alternate_product.is_none());
    assert!(deserialized.promotional_offer.is_none());
}

#[test]
fn test_realtime_response_body_with_alternate_product() {
    // Create a RealtimeResponseBody with an AlternateProduct
    let message_id = Uuid::parse_str("b2c3d4e5-f6a7-8901-b2c3-d4e5f6a78901").unwrap();
    let product_id = "com.example.alternate.product".to_string();
    let alternate_product = AlternateProduct {
        message_identifier: Some(message_id),
        product_id: Some(product_id.clone()),
    };
    let response_body = RealtimeResponseBody {
        message: None,
        alternate_product: Some(alternate_product),
        promotional_offer: None,
    };

    // Serialize to JSON
    let json_data = serde_json::to_value(&response_body).unwrap();

    // Validate JSON structure
    assert!(
        json_data.get("alternateProduct").is_some(),
        "JSON should have 'alternateProduct' field"
    );
    let alternate_product_dict = json_data["alternateProduct"].as_object().unwrap();
    assert!(
        alternate_product_dict.contains_key("messageIdentifier"),
        "AlternateProduct should have 'messageIdentifier' field"
    );
    assert!(
        alternate_product_dict.contains_key("productId"),
        "AlternateProduct should have 'productId' field"
    );
    assert_eq!(
        "b2c3d4e5-f6a7-8901-b2c3-d4e5f6a78901",
        alternate_product_dict["messageIdentifier"].as_str().unwrap()
    );
    assert_eq!(
        "com.example.alternate.product",
        alternate_product_dict["productId"].as_str().unwrap()
    );
    assert!(json_data.get("message").is_none(), "JSON should not have 'message' field");
    assert!(json_data.get("promotionalOffer").is_none(), "JSON should not have 'promotionalOffer' field");

    // Deserialize back
    let json_str = serde_json::to_string(&response_body).unwrap();
    let deserialized: RealtimeResponseBody = serde_json::from_str(&json_str).unwrap();

    // Verify
    assert!(deserialized.message.is_none());
    assert!(deserialized.alternate_product.is_some());
    assert_eq!(Some(message_id), deserialized.alternate_product.as_ref().unwrap().message_identifier);
    assert_eq!(Some(product_id), deserialized.alternate_product.as_ref().unwrap().product_id);
    assert!(deserialized.promotional_offer.is_none());
}

#[test]
fn test_realtime_response_body_with_promotional_offer_v2() {
    // Create a RealtimeResponseBody with a PromotionalOffer (V2 signature)
    let message_id = Uuid::parse_str("c3d4e5f6-a789-0123-c3d4-e5f6a7890123").unwrap();
    let signature_v2 = "signature2".to_string();
    let promotional_offer = PromotionalOffer {
        message_identifier: Some(message_id),
        promotional_offer_signature_v2: Some(signature_v2.clone()),
        promotional_offer_signature_v1: None,
    };
    let response_body = RealtimeResponseBody {
        message: None,
        alternate_product: None,
        promotional_offer: Some(promotional_offer),
    };

    // Serialize to JSON
    let json_data = serde_json::to_value(&response_body).unwrap();

    // Validate JSON structure
    assert!(
        json_data.get("promotionalOffer").is_some(),
        "JSON should have 'promotionalOffer' field"
    );
    let promotional_offer_dict = json_data["promotionalOffer"].as_object().unwrap();
    assert!(
        promotional_offer_dict.contains_key("messageIdentifier"),
        "PromotionalOffer should have 'messageIdentifier' field"
    );
    assert!(
        promotional_offer_dict.contains_key("promotionalOfferSignatureV2"),
        "PromotionalOffer should have 'promotionalOfferSignatureV2' field"
    );
    assert_eq!(
        "c3d4e5f6-a789-0123-c3d4-e5f6a7890123",
        promotional_offer_dict["messageIdentifier"].as_str().unwrap()
    );
    assert_eq!(
        "signature2",
        promotional_offer_dict["promotionalOfferSignatureV2"].as_str().unwrap()
    );
    assert!(
        !promotional_offer_dict.contains_key("promotionalOfferSignatureV1"),
        "PromotionalOffer should not have 'promotionalOfferSignatureV1' field"
    );
    assert!(json_data.get("message").is_none(), "JSON should not have 'message' field");
    assert!(json_data.get("alternateProduct").is_none(), "JSON should not have 'alternateProduct' field");

    // Deserialize back
    let json_str = serde_json::to_string(&response_body).unwrap();
    let deserialized: RealtimeResponseBody = serde_json::from_str(&json_str).unwrap();

    // Verify
    assert!(deserialized.message.is_none());
    assert!(deserialized.alternate_product.is_none());
    assert!(deserialized.promotional_offer.is_some());
    assert_eq!(Some(message_id), deserialized.promotional_offer.as_ref().unwrap().message_identifier);
    assert_eq!(
        Some(signature_v2),
        deserialized.promotional_offer.as_ref().unwrap().promotional_offer_signature_v2
    );
    assert!(deserialized.promotional_offer.as_ref().unwrap().promotional_offer_signature_v1.is_none());
}

#[test]
fn test_realtime_response_body_with_promotional_offer_v1() {
    // Create a RealtimeResponseBody with a PromotionalOffer (V1 signature)
    let message_id = Uuid::parse_str("d4e5f6a7-8901-2345-d4e5-f6a789012345").unwrap();
    let nonce = Uuid::parse_str("e5f6a789-0123-4567-e5f6-a78901234567").unwrap();
    let app_account_token = Uuid::parse_str("f6a78901-2345-6789-f6a7-890123456789").unwrap();
    let signature_v1 = PromotionalOfferSignatureV1 {
        encoded_signature: "base64encodedSignature".to_string(),
        product_id: "com.example.product".to_string(),
        nonce,
        timestamp: 1698148900000,
        key_id: "keyId123".to_string(),
        offer_identifier: "offer123".to_string(),
        app_account_token: Some(app_account_token),
    };

    let promotional_offer = PromotionalOffer {
        message_identifier: Some(message_id),
        promotional_offer_signature_v1: Some(signature_v1.clone()),
        promotional_offer_signature_v2: None,
    };
    let response_body = RealtimeResponseBody {
        message: None,
        alternate_product: None,
        promotional_offer: Some(promotional_offer),
    };

    // Serialize to JSON
    let json_data = serde_json::to_value(&response_body).unwrap();

    // Validate JSON structure
    assert!(
        json_data.get("promotionalOffer").is_some(),
        "JSON should have 'promotionalOffer' field"
    );
    let promotional_offer_dict = json_data["promotionalOffer"].as_object().unwrap();
    assert!(
        promotional_offer_dict.contains_key("messageIdentifier"),
        "PromotionalOffer should have 'messageIdentifier' field"
    );
    assert!(
        promotional_offer_dict.contains_key("promotionalOfferSignatureV1"),
        "PromotionalOffer should have 'promotionalOfferSignatureV1' field"
    );
    assert_eq!(
        "d4e5f6a7-8901-2345-d4e5-f6a789012345",
        promotional_offer_dict["messageIdentifier"].as_str().unwrap()
    );

    let v1_dict = promotional_offer_dict["promotionalOfferSignatureV1"].as_object().unwrap();
    assert!(v1_dict.contains_key("encodedSignature"), "V1 signature should have 'encodedSignature' field");
    assert!(v1_dict.contains_key("productId"), "V1 signature should have 'productId' field");
    assert!(v1_dict.contains_key("nonce"), "V1 signature should have 'nonce' field");
    assert!(v1_dict.contains_key("timestamp"), "V1 signature should have 'timestamp' field");
    assert!(v1_dict.contains_key("keyId"), "V1 signature should have 'keyId' field");
    assert!(v1_dict.contains_key("offerIdentifier"), "V1 signature should have 'offerIdentifier' field");
    assert!(v1_dict.contains_key("appAccountToken"), "V1 signature should have 'appAccountToken' field");
    assert_eq!("base64encodedSignature", v1_dict["encodedSignature"].as_str().unwrap());
    assert_eq!("com.example.product", v1_dict["productId"].as_str().unwrap());
    assert_eq!("e5f6a789-0123-4567-e5f6-a78901234567", v1_dict["nonce"].as_str().unwrap());
    assert_eq!(1698148900000, v1_dict["timestamp"].as_i64().unwrap());
    assert_eq!("keyId123", v1_dict["keyId"].as_str().unwrap());
    assert_eq!("offer123", v1_dict["offerIdentifier"].as_str().unwrap());
    assert_eq!("f6a78901-2345-6789-f6a7-890123456789", v1_dict["appAccountToken"].as_str().unwrap());

    assert!(
        !promotional_offer_dict.contains_key("promotionalOfferSignatureV2"),
        "PromotionalOffer should not have 'promotionalOfferSignatureV2' field"
    );
    assert!(json_data.get("message").is_none(), "JSON should not have 'message' field");
    assert!(json_data.get("alternateProduct").is_none(), "JSON should not have 'alternateProduct' field");

    // Deserialize back
    let json_str = serde_json::to_string(&response_body).unwrap();
    let deserialized: RealtimeResponseBody = serde_json::from_str(&json_str).unwrap();

    // Verify
    assert!(deserialized.message.is_none());
    assert!(deserialized.alternate_product.is_none());
    assert!(deserialized.promotional_offer.is_some());
    assert_eq!(Some(message_id), deserialized.promotional_offer.as_ref().unwrap().message_identifier);
    assert!(deserialized.promotional_offer.as_ref().unwrap().promotional_offer_signature_v2.is_none());
    assert!(deserialized.promotional_offer.as_ref().unwrap().promotional_offer_signature_v1.is_some());

    let deserialized_v1 = deserialized.promotional_offer.as_ref().unwrap().promotional_offer_signature_v1.as_ref().unwrap();
    assert_eq!("com.example.product", deserialized_v1.product_id);
    assert_eq!("offer123", deserialized_v1.offer_identifier);
    assert_eq!(nonce, deserialized_v1.nonce);
    assert_eq!(1698148900000, deserialized_v1.timestamp);
    assert_eq!("keyId123", deserialized_v1.key_id);
    assert_eq!(Some(app_account_token), deserialized_v1.app_account_token);
    assert_eq!("base64encodedSignature", deserialized_v1.encoded_signature);
}

#[test]
fn test_realtime_response_body_serialization() {
    // Test that JSON serialization uses correct field names
    let message_id = Uuid::parse_str("12345678-1234-1234-1234-123456789012").unwrap();
    let message = Message {
        message_identifier: Some(message_id),
    };
    let response_body = RealtimeResponseBody {
        message: Some(message),
        alternate_product: None,
        promotional_offer: None,
    };

    let json_string = serde_json::to_string(&response_body).unwrap();

    // Verify JSON contains correct field names
    assert!(json_string.contains("\"message\""));
    assert!(json_string.contains("\"messageIdentifier\""));
    assert!(json_string.contains("\"12345678-1234-1234-1234-123456789012\""));
}