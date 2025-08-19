use app_store_server_library::promotional_offer_signature_creator::PromotionalOfferSignatureCreator;

#[test]
fn test_promotional_offer_signature_creator() {
    let private_key = include_str!("../tests/resources/certs/testSigningKey.p8");
    let creator = PromotionalOfferSignatureCreator::new(
        private_key,
        "L256SYR32L".to_string(),
        "com.test.app".to_string(),
    )
    .unwrap();
    let r = creator
        .create_signature(
            "com.test.product",
            "com.test.offer",
            uuid::Uuid::new_v4()
                .to_string()
                .as_str(),
            &uuid::Uuid::new_v4(),
            12345,
        )
        .unwrap();

    assert!(!r.is_empty())
}
