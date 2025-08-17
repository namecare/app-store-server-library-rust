use base64::engine::general_purpose::STANDARD;
use base64::{DecodeError, Engine};

use crate::chain_verifier::{verify_chain, ChainVerifierError};
use crate::primitives::app_transaction::AppTransaction;
use crate::primitives::environment::Environment;
use crate::primitives::jws_renewal_info_decoded_payload::JWSRenewalInfoDecodedPayload;
use crate::primitives::jws_transaction_decoded_payload::JWSTransactionDecodedPayload;
use crate::primitives::response_body_v2_decoded_payload::ResponseBodyV2DecodedPayload;
use crate::utils::{base64_url_to_base64, StringExt};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde::de::DeserializeOwned;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum SignedDataVerifierError {
    #[error("VerificationFailure")]
    VerificationFailure,

    #[error("InvalidAppIdentifier")]
    InvalidAppIdentifier,

    #[error("InvalidEnvironment")]
    InvalidEnvironment,

    #[error("InternalChainVerifierError")]
    InternalChainVerifierError(#[from] ChainVerifierError),

    #[error("InternalDecodeError: [{0}]")]
    InternalDecodeError(#[from] DecodeError),

    #[error("InternalJWTError: [{0}]")]
    InternalJWTError(#[from] jsonwebtoken::errors::Error),
}

/// A verifier for signed data, commonly used for verifying and decoding
/// signed Apple server notifications and transactions.
pub struct SignedDataVerifier {
    root_certificates: Vec<Vec<u8>>,
    environment: Environment,
    bundle_id: String,
    app_apple_id: Option<i64>,
}

impl SignedDataVerifier {
    /// Creates a new `SignedDataVerifier` instance with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `root_certificates` - A vector of DER-encoded root certificates used for verification.
    /// * `environment` - The environment (e.g., `Environment::PRODUCTION` or `Environment::SANDBOX`).
    /// * `bundle_id` - The bundle ID associated with the application.
    /// * `app_apple_id` - An optional Apple ID associated with the application.
    ///
    /// # Returns
    ///
    /// A new `SignedDataVerifier` instance.
    pub fn new(
        root_certificates: Vec<Vec<u8>>,
        environment: Environment,
        bundle_id: String,
        app_apple_id: Option<i64>,
    ) -> Self {
        return SignedDataVerifier {
            root_certificates,
            environment,
            bundle_id,
            app_apple_id,
        };
    }
}

impl SignedDataVerifier {
    /// Verifies and decodes a signed renewal info.
    ///
    /// This method takes a signed renewal info string, verifies its authenticity and
    /// integrity, and returns the decoded payload as a `JWSRenewalInfoDecodedPayload`
    /// if the verification is successful.
    ///
    /// # Arguments
    ///
    /// * `signed_renewal_info` - The signed renewal info string to verify and decode.
    ///
    /// # Returns
    ///
    /// - `Ok(JWSRenewalInfoDecodedPayload)` if verification and decoding are successful.
    /// - `Err(SignedDataVerifierError)` if verification or decoding fails, with error details.
    pub fn verify_and_decode_renewal_info(
        &self,
        signed_renewal_info: &str,
    ) -> Result<JWSRenewalInfoDecodedPayload, SignedDataVerifierError> {
        Ok(self.decode_signed_object(signed_renewal_info)?)
    }

    /// Verifies and decodes a signed transaction.
    ///
    /// This method takes a signed transaction string, verifies its authenticity and
    /// integrity, and returns the decoded payload as a `JWSTransactionDecodedPayload`
    /// if the verification is successful.
    ///
    /// # Arguments
    ///
    /// * `signed_transaction` - The signed transaction string to verify and decode.
    ///
    /// # Returns
    ///
    /// - `Ok(JWSTransactionDecodedPayload)` if verification and decoding are successful.
    /// - `Err(SignedDataVerifierError)` if verification or decoding fails, with error details.
    pub fn verify_and_decode_signed_transaction(
        &self,
        signed_transaction: &str,
    ) -> Result<JWSTransactionDecodedPayload, SignedDataVerifierError> {
        let decoded_signed_tx: JWSTransactionDecodedPayload = self.decode_signed_object(signed_transaction)?;

        if decoded_signed_tx.bundle_id.as_ref() != Some(&self.bundle_id) {
            return Err(SignedDataVerifierError::InvalidAppIdentifier);
        }

        if decoded_signed_tx.environment.as_ref() != Some(&self.environment) {
            return Err(SignedDataVerifierError::InvalidEnvironment);
        }

        Ok(decoded_signed_tx)
    }

    /// Verifies and decodes a signed notification.
    ///
    /// This method takes a signed notification string, verifies its authenticity and
    /// integrity, and returns the decoded payload as a `ResponseBodyV2DecodedPayload`
    /// if the verification is successful.
    ///
    /// # Arguments
    ///
    /// * `signed_payload` - The signed notification string to verify and decode.
    ///
    /// # Returns
    ///
    /// - `Ok(ResponseBodyV2DecodedPayload)` if verification and decoding are successful.
    /// - `Err(SignedDataVerifierError)` if verification or decoding fails, with error details.
    pub fn verify_and_decode_notification(
        &self,
        signed_payload: &str,
    ) -> Result<ResponseBodyV2DecodedPayload, SignedDataVerifierError> {
        let decoded_signed_notification: ResponseBodyV2DecodedPayload = self.decode_signed_object(signed_payload)?;

        let bundle_id;
        let app_apple_id;
        let environment;

        if let Some(data) = &decoded_signed_notification.data {
            bundle_id = data.bundle_id.clone();
            app_apple_id = data.app_apple_id.clone();
            environment = data.environment.clone();
        } else if let Some(summary) = &decoded_signed_notification.summary {
            bundle_id = summary.bundle_id.clone();
            app_apple_id = summary.app_apple_id.clone();
            environment = summary.environment.clone();
        } else if let Some(external_purchase_token) = &decoded_signed_notification.external_purchase_token {
            bundle_id = external_purchase_token.bundle_id.clone();
            app_apple_id = external_purchase_token.app_apple_id.clone();

            if let Some(external_purchase_id) = &external_purchase_token.external_purchase_id {
                if external_purchase_id.starts_with("SANDBOX") {
                    environment = Some(Environment::Sandbox)
                } else {
                    environment = Some(Environment::Production)
                }
            } else {
                environment = Some(Environment::Production)
            }
        } else {
            bundle_id = None;
            app_apple_id = None;
            environment = None;
        }

        self.verify_notification_app_identifier_and_environment(bundle_id, app_apple_id, environment)?;

        Ok(decoded_signed_notification)
    }

    fn verify_notification_app_identifier_and_environment(
        &self,
        bundle_id: Option<String>,
        app_apple_id: Option<i64>,
        environment: Option<Environment>,
    ) -> Result<(), SignedDataVerifierError> {
        if let Some(bundle_id) = bundle_id {
            if bundle_id != self.bundle_id {
                return Err(SignedDataVerifierError::InvalidAppIdentifier);
            }
        }

        if self.environment == Environment::Production && self.app_apple_id != app_apple_id {
            return Err(SignedDataVerifierError::InvalidAppIdentifier);
        }

        if let Some(environment) = environment {
            if self.environment != Environment::LocalTesting && self.environment != environment {
                return Err(SignedDataVerifierError::InvalidEnvironment);
            }
        }

        Ok(())
    }

    /// Verifies and decodes a signed notification.
    ///
    /// This method takes a signed notification string, verifies its authenticity and
    /// integrity, and returns the decoded payload as a `ResponseBodyV2DecodedPayload`
    /// if the verification is successful.
    ///
    /// # Arguments
    ///
    /// * `signed_payload` - The signed notification string to verify and decode.
    ///
    /// # Returns
    ///
    /// - `Ok(ResponseBodyV2DecodedPayload)` if verification and decoding are successful.
    /// - `Err(SignedDataVerifierError)` if verification or decoding fails, with error details.
    pub fn verify_and_decode_app_transaction(
        &self,
        signed_app_transaction: &str,
    ) -> Result<AppTransaction, SignedDataVerifierError> {
        let decoded_app_transaction: AppTransaction = self.decode_signed_object(signed_app_transaction)?;

        if decoded_app_transaction.bundle_id.as_ref() != Some(&self.bundle_id) {
            return Err(SignedDataVerifierError::InvalidAppIdentifier);
        }

        if decoded_app_transaction.receipt_type.as_ref() != Some(&self.environment) {
            return Err(SignedDataVerifierError::InvalidEnvironment);
        }

        Ok(decoded_app_transaction)
    }

    /// Private method used for decoding a signed object (internal use).
    fn decode_signed_object<T: DeserializeOwned>(&self, signed_obj: &str) -> Result<T, SignedDataVerifierError> {
        // Data is not signed by the App Store, and verification should be skipped
        // The environment MUST be checked in the public method calling this
        if self.environment == Environment::Xcode || self.environment == Environment::LocalTesting {
            const EXPECTED_JWT_SEGMENTS: usize = 3;

            let body_segments: Vec<&str> = signed_obj.split('.').collect();

            if body_segments.len() != EXPECTED_JWT_SEGMENTS {
                return Err(SignedDataVerifierError::VerificationFailure);
            }

            let _ = jsonwebtoken::decode_header(&signed_obj)?;
            let body_data = base64_url_to_base64(body_segments[1]);

            let decoded_body = match STANDARD.decode(body_data) {
                Ok(decoded_body) => match serde_json::from_slice(&decoded_body) {
                    Ok(decoded) => decoded,
                    Err(_) => return Err(SignedDataVerifierError::VerificationFailure),
                },
                Err(_) => return Err(SignedDataVerifierError::VerificationFailure),
            };

            return Ok(decoded_body);
        }

        let header = jsonwebtoken::decode_header(signed_obj)?;

        let Some(x5c) = header.x5c else {
            return Err(SignedDataVerifierError::VerificationFailure);
        };

        if x5c.is_empty() {
            return Err(SignedDataVerifierError::VerificationFailure);
        }

        let x5c: Result<Vec<Vec<u8>>, DecodeError> = x5c.iter().map(|c| c.as_der_bytes()).collect();
        let chain = x5c?;

        if header.alg != Algorithm::ES256 {
            return Err(SignedDataVerifierError::VerificationFailure);
        }

        let pub_key = verify_chain(&chain, &self.root_certificates, None)?;
        let pub_key = &pub_key[pub_key.len() - 65..];

        let decoding_key = DecodingKey::from_ec_der(pub_key);
        let claims: [&str; 0] = [];

        let mut validator = Validation::new(Algorithm::ES256);
        validator.validate_exp = false;
        validator.set_required_spec_claims(&claims);

        let payload = jsonwebtoken::decode::<T>(signed_obj, &decoding_key, &validator).expect("Expect Payload");
        return Ok(payload.claims);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::auto_renew_status::AutoRenewStatus;
    use crate::primitives::consumption_request_reason::ConsumptionRequestReason;
    use crate::primitives::expiration_intent::ExpirationIntent;
    use crate::primitives::in_app_ownership_type::InAppOwnershipType;
    use crate::primitives::notification_type_v2::NotificationTypeV2;
    use crate::primitives::offer_discount_type::OfferDiscountType;
    use crate::primitives::offer_type::OfferType;
    use crate::primitives::price_increase_status::PriceIncreaseStatus;
    use crate::primitives::product_type::ProductType;
    use crate::primitives::purchase_platform::PurchasePlatform;
    use crate::primitives::revocation_reason::RevocationReason;
    use crate::primitives::status::Status;
    use crate::primitives::subtype::Subtype;
    use crate::primitives::transaction_reason::TransactionReason;
    use ring::signature::ECDSA_P256_SHA256_FIXED_SIGNING;
    use serde_json::{Map, Value};
    use std::fs;

    const ROOT_CA_BASE64_ENCODED: &str = "MIIBgjCCASmgAwIBAgIJALUc5ALiH5pbMAoGCCqGSM49BAMDMDYxCzAJBgNVBAYTAlVTMRMwEQYDVQQIDApDYWxpZm9ybmlhMRIwEAYDVQQHDAlDdXBlcnRpbm8wHhcNMjMwMTA1MjEzMDIyWhcNMzMwMTAyMjEzMDIyWjA2MQswCQYDVQQGEwJVUzETMBEGA1UECAwKQ2FsaWZvcm5pYTESMBAGA1UEBwwJQ3VwZXJ0aW5vMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEc+/Bl+gospo6tf9Z7io5tdKdrlN1YdVnqEhEDXDShzdAJPQijamXIMHf8xWWTa1zgoYTxOKpbuJtDplz1XriTaMgMB4wDAYDVR0TBAUwAwEB/zAOBgNVHQ8BAf8EBAMCAQYwCgYIKoZIzj0EAwMDRwAwRAIgemWQXnMAdTad2JDJWng9U4uBBL5mA7WI05H7oH7c6iQCIHiRqMjNfzUAyiu9h6rOU/K+iTR0I/3Y/NSWsXHX+acc";
    const XCODE_BUNDLE_ID: &str = "com.example.naturelab.backyardbirds.example";

    #[test]
    fn test_app_store_server_notification_decoding() {
        let test_notification_data = fs::read_to_string("resources/mock_signed_data/testNotification")
            .expect("Failed to read file");
        let verifier = get_signed_data_verifier(Environment::Sandbox, "com.example", None);
        let notification = verifier
            .verify_and_decode_notification(&test_notification_data)
            .unwrap();
        assert_eq!(notification.notification_type, NotificationTypeV2::Test);
    }

    #[test]
    fn test_app_store_server_notification_decoding_production() {
        let test_notification_data = fs::read_to_string("resources/mock_signed_data/testNotification")
            .expect("Failed to read file");
        let verifier = get_signed_data_verifier(Environment::Production, "com.example", None);
        let error = verifier
            .verify_and_decode_notification(&test_notification_data)
            .err()
            .unwrap();

        assert_eq!(error, SignedDataVerifierError::InvalidEnvironment);
    }

    #[test]
    fn test_missing_x5c_header() {
        let missing_x5c_header_claim_data = fs::read_to_string("resources/mock_signed_data/missingX5CHeaderClaim")
            .expect("Failed to read file");
        let verifier = get_signed_data_verifier(Environment::Sandbox, "com.example", None);
        let result = verifier.verify_and_decode_notification(&missing_x5c_header_claim_data);
        assert_eq!(
            result.err().unwrap(),
            SignedDataVerifierError::VerificationFailure
        );
    }

    #[test]
    fn test_wrong_bundle_id_for_server_notification() {
        let wrong_bundle_id_data = fs::read_to_string("resources/mock_signed_data/wrongBundleId")
            .expect("Failed to read file");
        let verifier = get_signed_data_verifier(Environment::Sandbox, "com.example", None);
        let result = verifier.verify_and_decode_notification(&wrong_bundle_id_data);
        assert_eq!(
            result.err().unwrap(),
            SignedDataVerifierError::InvalidAppIdentifier
        );
    }

    #[test]
    fn test_wrong_app_apple_id_for_server_notification() {
        let test_notification_data = fs::read_to_string("resources/mock_signed_data/testNotification")
            .expect("Failed to read file");
        let verifier = get_signed_data_verifier(Environment::Production, "com.example", Some(1235));
        let result = verifier.verify_and_decode_notification(&test_notification_data);
        assert_eq!(
            result.err().unwrap(),
            SignedDataVerifierError::InvalidAppIdentifier
        );
    }

    #[test]
    fn test_renewal_info_decoding() {
        let renewal_info_data = fs::read_to_string("resources/mock_signed_data/renewalInfo").expect("Failed to read file");
        let verifier = get_signed_data_verifier(Environment::Sandbox, "com.example", None);
        let renewal_info = verifier
            .verify_and_decode_renewal_info(&renewal_info_data)
            .unwrap();
        assert_eq!(renewal_info.environment, Some(Environment::Sandbox));
    }

    #[test]
    fn test_external_purchase_token_notification_decoding() {
        let signed_notification = create_signed_data_from_json("resources/models/signedExternalPurchaseTokenNotification.json");
        let signed_data_verifier = get_signed_data_verifier(Environment::LocalTesting, "com.example", Some(55555));

        match signed_data_verifier.verify_and_decode_notification(&signed_notification) {
            Ok(notification) => {
                assert_eq!(
                    NotificationTypeV2::ExternalPurchaseToken,
                    notification.notification_type
                );
                assert_eq!(
                    Subtype::Unreported,
                    notification.subtype.expect("Expect subtype")
                );
                assert_eq!(
                    "002e14d5-51f5-4503-b5a8-c3a1af68eb20",
                    &notification.notification_uuid
                );
                assert_eq!("2.0", &notification.version.expect("Expect version"));
                assert_eq!(
                    1698148900,
                    notification
                        .signed_date
                        .expect("Expect signed_date")
                        .timestamp()
                );
                assert!(notification.data.is_none());
                assert!(notification.summary.is_none());
                assert!(notification.external_purchase_token.is_some());

                if let Some(external_purchase_token) = notification.external_purchase_token {
                    assert_eq!(
                        "b2158121-7af9-49d4-9561-1f588205523e",
                        &external_purchase_token
                            .external_purchase_id
                            .expect("Expect external_purchase_id")
                    );
                    assert_eq!(
                        1698148950,
                        external_purchase_token
                            .token_creation_date
                            .unwrap()
                            .timestamp()
                    );
                    assert_eq!(55555, external_purchase_token.app_apple_id.unwrap());
                    assert_eq!("com.example", &external_purchase_token.bundle_id.unwrap());
                } else {
                    panic!("External purchase token is expected to be Some, but it was None");
                }
            }
            Err(err) => {
                panic!("Failed to verify and decode app transaction: {:?}", err)
            }
        }
    }

    #[test]
    fn test_external_purchase_token_sanbox_notification_decoding() {
        let signed_notification = create_signed_data_from_json("resources/models/signedExternalPurchaseTokenSandboxNotification.json");
        let signed_data_verifier = get_signed_data_verifier(Environment::LocalTesting, "com.example", Some(55555));

        match signed_data_verifier.verify_and_decode_notification(&signed_notification) {
            Ok(notification) => {
                assert_eq!(
                    NotificationTypeV2::ExternalPurchaseToken,
                    notification.notification_type
                );
                assert_eq!(
                    Subtype::Unreported,
                    notification.subtype.expect("Expect subtype")
                );
                assert_eq!(
                    "002e14d5-51f5-4503-b5a8-c3a1af68eb20",
                    &notification.notification_uuid
                );
                assert_eq!("2.0", &notification.version.expect("Expect version"));
                assert_eq!(
                    1698148900,
                    notification
                        .signed_date
                        .expect("Expect signed_date")
                        .timestamp()
                );
                assert!(notification.data.is_none());
                assert!(notification.summary.is_none());
                assert!(notification.external_purchase_token.is_some());

                if let Some(external_purchase_token) = notification.external_purchase_token {
                    assert_eq!(
                        "SANDBOX_b2158121-7af9-49d4-9561-1f588205523e",
                        &external_purchase_token
                            .external_purchase_id
                            .expect("Expect external_purchase_id")
                    );
                    assert_eq!(
                        1698148950,
                        external_purchase_token
                            .token_creation_date
                            .unwrap()
                            .timestamp()
                    );
                    assert_eq!(55555, external_purchase_token.app_apple_id.unwrap());
                    assert_eq!("com.example", &external_purchase_token.bundle_id.unwrap());
                } else {
                    panic!("External purchase token is expected to be Some, but it was None");
                }
            }
            Err(err) => {
                panic!("Failed to verify and decode app transaction: {:?}", err)
            }
        }
    }

    #[test]
    fn test_transaction_info_decoding() {
        let transaction_info_data = fs::read_to_string("resources/mock_signed_data/transactionInfo")
            .expect("Failed to read file");
        let verifier = get_signed_data_verifier(Environment::Sandbox, "com.example", None);
        let notification = verifier
            .verify_and_decode_signed_transaction(&transaction_info_data)
            .unwrap();
        assert_eq!(notification.environment, Some(Environment::Sandbox));
    }

    #[test]
    fn test_malformed_jwt_with_too_many_parts() {
        let verifier = get_signed_data_verifier(Environment::Sandbox, "com.example", None);
        let result = verifier.verify_and_decode_notification("a.b.c.d");
        assert!(result
            .err()
            .unwrap()
            .to_string()
            .contains("InternalJWTError"));
    }

    #[test]
    fn test_malformed_jwt_with_malformed_data() {
        let verifier = get_signed_data_verifier(Environment::Sandbox, "com.example", None);
        let result = verifier.verify_and_decode_notification("a.b.c");
        assert!(result
            .err()
            .unwrap()
            .to_string()
            .contains("InternalJWTError"));
    }

    fn get_signed_data_verifier(
        environment: Environment,
        bundle_id: &str,
        app_apple_id: Option<i64>,
    ) -> SignedDataVerifier {
        let verifier = SignedDataVerifier::new(
            vec![ROOT_CA_BASE64_ENCODED.as_der_bytes().unwrap()],
            environment,
            bundle_id.to_string(),
            app_apple_id.or(Some(1234)),
        );

        verifier
    }

    #[test]
    fn test_decoded_payloads_app_transaction_decoding() {
        let signed_app_transaction = create_signed_data_from_json("resources/models/appTransaction.json");

        let signed_data_verifier = get_default_signed_data_verifier();

        match signed_data_verifier.verify_and_decode_app_transaction(&signed_app_transaction) {
            Ok(app_transaction) => {
                assert_eq!(
                    Some(&Environment::LocalTesting),
                    app_transaction.receipt_type.as_ref()
                );
                assert_eq!(
                    531412,
                    app_transaction.app_apple_id.expect("Expect app_apple_id")
                );
                assert_eq!(
                    "com.example",
                    app_transaction.bundle_id.expect("Expect bundle_id")
                );
                assert_eq!(
                    "1.2.3",
                    app_transaction
                        .application_version
                        .expect("Expect application_version")
                );
                assert_eq!(
                    512,
                    app_transaction
                        .version_external_identifier
                        .expect("Expect version_external_identifier")
                );
                assert_eq!(
                    1698148900,
                    app_transaction
                        .receipt_creation_date
                        .expect("Expect receipt_creation_date")
                        .timestamp()
                );
                assert_eq!(
                    1698148800,
                    app_transaction
                        .original_purchase_date
                        .expect("Expect original_purchase_date")
                        .timestamp()
                );
                assert_eq!(
                    "1.1.2",
                    app_transaction
                        .original_application_version
                        .expect("Expect original_application_version")
                );
                assert_eq!(
                    "device_verification_value",
                    app_transaction
                        .device_verification
                        .expect("Expect device_verification")
                );
                assert_eq!(
                    "48ccfa42-7431-4f22-9908-7e88983e105a",
                    app_transaction
                        .device_verification_nonce
                        .expect("Expect device_verification_nonce")
                        .to_string()
                );
                assert_eq!(
                    1698148700,
                    app_transaction
                        .preorder_date
                        .expect("Expect preorder_date")
                        .timestamp()
                );
                assert_eq!(
                    "71134",
                    app_transaction
                        .app_transaction_id
                        .expect("Expect app_transaction_id")
                        .to_string()
                );
                assert_eq!(
                    PurchasePlatform::IOS,
                    app_transaction
                        .original_platform
                        .expect("Expect original_platform")
                );
            }
            Err(err) => panic!("Failed to verify and decode app transaction: {:?}", err),
        }
    }

    #[test]
    fn test_decoded_payloads_transaction_decoding() {
        let signed_transaction = create_signed_data_from_json("resources/models/signedTransaction.json");

        let signed_data_verifier = get_default_signed_data_verifier();

        match signed_data_verifier.verify_and_decode_signed_transaction(&signed_transaction) {
            Ok(transaction) => {
                assert_eq!(
                    "12345",
                    transaction
                        .original_transaction_id
                        .as_deref()
                        .expect("Expect original_transaction_id")
                );
                assert_eq!(
                    "23456",
                    transaction
                        .transaction_id
                        .as_deref()
                        .expect("Expect transaction_id")
                );
                assert_eq!(
                    "34343",
                    transaction
                        .web_order_line_item_id
                        .as_deref()
                        .expect("Expect web_order_line_item_id")
                );
                assert_eq!(
                    "com.example",
                    transaction.bundle_id.as_deref().expect("Expect bundle_id")
                );
                assert_eq!(
                    "com.example.product",
                    transaction
                        .product_id
                        .as_deref()
                        .expect("Expect product_id")
                );
                assert_eq!(
                    "55555",
                    transaction
                        .subscription_group_identifier
                        .as_deref()
                        .expect("Expect subscription_group_identifier")
                );
                assert_eq!(
                    1698148800,
                    transaction
                        .original_purchase_date
                        .expect("Expect original_purchase_date")
                        .timestamp()
                );
                assert_eq!(
                    1698148900,
                    transaction
                        .purchase_date
                        .expect("Expect purchase_date")
                        .timestamp()
                );
                assert_eq!(
                    1698148950,
                    transaction
                        .revocation_date
                        .expect("Expect revocation_date")
                        .timestamp()
                );
                assert_eq!(
                    1698149000,
                    transaction
                        .expires_date
                        .expect("Expect expires_date")
                        .timestamp()
                );
                assert_eq!(1, transaction.quantity.expect("Expect quantity"));
                assert_eq!(
                    ProductType::AutoRenewableSubscription,
                    transaction.r#type.expect("Expect type")
                );
                assert_eq!(
                    "7e3fb20b-4cdb-47cc-936d-99d65f608138",
                    transaction
                        .app_account_token
                        .expect("Expect app_account_token")
                        .to_string()
                );
                assert_eq!(
                    InAppOwnershipType::Purchased,
                    transaction
                        .in_app_ownership_type
                        .expect("Expect in_app_ownership_type")
                );
                assert_eq!(
                    1698148900,
                    transaction
                        .signed_date
                        .expect("Expect signed_date")
                        .timestamp()
                );
                assert_eq!(
                    RevocationReason::RefundedDueToIssue,
                    transaction
                        .revocation_reason
                        .expect("Expect revocation_reason")
                );
                assert_eq!(
                    "abc.123",
                    transaction
                        .offer_identifier
                        .as_deref()
                        .expect("Expect offer_identifier")
                );
                assert!(transaction.is_upgraded.unwrap_or_default());
                assert_eq!(
                    OfferType::IntroductoryOffer,
                    transaction.offer_type.expect("Expect offer_type")
                );
                assert_eq!(
                    "USA",
                    transaction
                        .storefront
                        .as_deref()
                        .expect("Expect storefront")
                );
                assert_eq!(
                    "143441",
                    transaction
                        .storefront_id
                        .as_deref()
                        .expect("Expect storefront_id")
                );
                assert_eq!(
                    TransactionReason::Purchase,
                    transaction
                        .transaction_reason
                        .expect("Expect transaction_reason")
                );
                assert_eq!(
                    Environment::LocalTesting,
                    transaction.environment.expect("Expect environment")
                );
                assert_eq!(10990, transaction.price.expect("Expect price"));
                assert_eq!(
                    "USD",
                    transaction.currency.as_deref().expect("Expect currency")
                );
                assert_eq!(
                    OfferDiscountType::PayAsYouGo,
                    transaction
                        .offer_discount_type
                        .expect("Expect offer_discount_type")
                );
                assert_eq!(
                    "71134",
                    transaction
                        .app_transaction_id
                        .expect("Expect app_transaction_id")
                        .to_string()
                );
                assert_eq!(
                    "P1Y",
                    transaction
                        .offer_period
                        .expect("Expect offer_period")
                        .to_string()
                );
            }
            Err(err) => panic!("Failed to verify and decode signed transaction: {:?}", err),
        }
    }

    #[test]
    fn test_decoded_payloads_renewal_info_decoding() {
        let signed_renewal_info = create_signed_data_from_json("resources/models/signedRenewalInfo.json");

        let signed_data_verifier = get_default_signed_data_verifier();

        match signed_data_verifier.verify_and_decode_renewal_info(&signed_renewal_info) {
            Ok(renewal_info) => {
                assert_eq!(
                    ExpirationIntent::CustomerCancelled,
                    renewal_info
                        .expiration_intent
                        .expect("Expect expiration_intent")
                );
                assert_eq!(
                    "12345",
                    renewal_info
                        .original_transaction_id
                        .as_deref()
                        .expect("Expect original_transaction_id")
                );
                assert_eq!(
                    "com.example.product.2",
                    renewal_info
                        .auto_renew_product_id
                        .as_deref()
                        .expect("Expect auto_renew_product_id")
                );
                assert_eq!(
                    "com.example.product",
                    renewal_info
                        .product_id
                        .as_deref()
                        .expect("Expect product_id")
                );
                assert_eq!(
                    AutoRenewStatus::On,
                    renewal_info
                        .auto_renew_status
                        .expect("Expect auto_renew_status")
                );
                assert!(renewal_info.is_in_billing_retry_period.unwrap_or_default());
                assert_eq!(
                    PriceIncreaseStatus::CustomerHasNotResponded,
                    renewal_info
                        .price_increase_status
                        .expect("Expect price_increase_status")
                );
                assert_eq!(
                    1698148900,
                    renewal_info
                        .grace_period_expires_date
                        .expect("Expect grace_period_expires_date")
                        .timestamp()
                );
                assert_eq!(
                    OfferType::PromotionalOffer,
                    renewal_info.offer_type.expect("Expect offer_type")
                );
                assert_eq!(
                    "abc.123",
                    renewal_info
                        .offer_identifier
                        .as_deref()
                        .expect("Expect offer_identifier")
                );
                assert_eq!(
                    1698148800,
                    renewal_info
                        .signed_date
                        .expect("Expect signed_date")
                        .timestamp()
                );
                assert_eq!(
                    Environment::LocalTesting,
                    renewal_info.environment.expect("Expect environment")
                );
                assert_eq!(
                    1698148800,
                    renewal_info
                        .recent_subscription_start_date
                        .expect("Expect recent_subscription_start_date")
                        .timestamp()
                );
                assert_eq!(
                    1698148850,
                    renewal_info
                        .renewal_date
                        .expect("Expect renewal_date")
                        .timestamp()
                );
                assert_eq!(
                    "71134",
                    renewal_info
                        .app_transaction_id
                        .expect("Expect app_transaction_id")
                        .to_string()
                );
                assert_eq!(
                    "P1Y",
                    renewal_info
                        .offer_period
                        .expect("Expect offer_period")
                        .to_string()
                );
                assert_eq!(
                    "7e3fb20b-4cdb-47cc-936d-99d65f608138",
                    renewal_info
                        .app_account_token
                        .expect("Expect app_account_token")
                        .to_string()
                );
            }
            Err(err) => panic!("Failed to verify and decode renewal info: {:?}", err),
        }
    }

    #[test]
    fn test_decoded_payloads_notification_decoding() {
        let signed_notification = create_signed_data_from_json("resources/models/signedNotification.json");

        let signed_data_verifier = get_default_signed_data_verifier();

        match signed_data_verifier.verify_and_decode_notification(&signed_notification) {
            Ok(notification) => {
                assert_eq!(
                    NotificationTypeV2::Subscribed,
                    notification.notification_type
                );
                assert_eq!(
                    Subtype::InitialBuy,
                    notification.subtype.expect("Expect subtype")
                );
                assert_eq!(
                    "002e14d5-51f5-4503-b5a8-c3a1af68eb20",
                    notification.notification_uuid
                );
                assert_eq!(
                    "2.0",
                    notification.version.as_deref().expect("Expect version")
                );
                assert_eq!(
                    1698148900,
                    notification
                        .signed_date
                        .expect("Expect signed_date")
                        .timestamp()
                );
                assert!(notification.data.is_some());
                assert!(notification.summary.is_none());
                assert!(notification.external_purchase_token.is_none());

                if let Some(data) = notification.data {
                    assert_eq!(
                        Environment::LocalTesting,
                        data.environment.expect("Expect environment")
                    );
                    assert_eq!(41234, data.app_apple_id.expect("Expect app_apple_id"));
                    assert_eq!(
                        "com.example",
                        data.bundle_id.as_deref().expect("Expect bundle_id")
                    );
                    assert_eq!(
                        "1.2.3",
                        data.bundle_version
                            .as_deref()
                            .expect("Expect bundle_version")
                    );
                    assert_eq!(
                        "signed_transaction_info_value",
                        data.signed_transaction_info
                            .as_deref()
                            .expect("Expect signed_transaction_info")
                    );
                    assert_eq!(
                        "signed_renewal_info_value",
                        data.signed_renewal_info
                            .as_deref()
                            .expect("Expect signed_renewal_info")
                    );
                    assert_eq!(Status::Active, data.status.expect("Expect status"));
                    assert!(data.consumption_request_reason.is_none());
                } else {
                    panic!("Data field is expected to be present in the notification");
                }
            }
            Err(err) => panic!("Failed to verify and decode notification: {:?}", err),
        }
    }

    #[test]
    fn test_consumption_request_notification_decoding() {
        let signed_notification = create_signed_data_from_json("resources/models/signedConsumptionRequestNotification.json");

        let signed_data_verifier = get_default_signed_data_verifier();

        match signed_data_verifier.verify_and_decode_notification(&signed_notification) {
            Ok(notification) => {
                assert_eq!(
                    NotificationTypeV2::ConsumptionRequest,
                    notification.notification_type
                );
                assert!(notification.subtype.is_none());
                assert_eq!(
                    "002e14d5-51f5-4503-b5a8-c3a1af68eb20",
                    notification.notification_uuid
                );
                assert_eq!("2.0", notification.version.unwrap());
                assert_eq!(1698148900, notification.signed_date.unwrap().timestamp());
                assert!(notification.data.is_some());
                assert!(notification.summary.is_none());
                assert!(notification.external_purchase_token.is_none());

                if let Some(data) = notification.data {
                    assert_eq!(Environment::LocalTesting, data.environment.unwrap());
                    assert_eq!(41234, data.app_apple_id.unwrap());
                    assert_eq!("com.example", data.bundle_id.unwrap());
                    assert_eq!("1.2.3", data.bundle_version.unwrap());
                    assert_eq!(
                        "signed_transaction_info_value",
                        data.signed_transaction_info.unwrap()
                    );
                    assert_eq!(
                        "signed_renewal_info_value",
                        data.signed_renewal_info.unwrap()
                    );
                    assert_eq!(Status::Active, data.status.unwrap());
                    assert_eq!(
                        ConsumptionRequestReason::UnintendedPurchase,
                        data.consumption_request_reason.unwrap()
                    );
                }
            }
            Err(err) => panic!(
                "Failed to verify and decode consumption request notification: {:?}",
                err
            ),
        }
    }

    #[test]
    fn test_summary_notification_decoding() {
        let signed_summary_notification = create_signed_data_from_json("resources/models/signedSummaryNotification.json");

        let signed_data_verifier = get_default_signed_data_verifier();

        match signed_data_verifier.verify_and_decode_notification(&signed_summary_notification) {
            Ok(notification) => {
                assert_eq!(
                    NotificationTypeV2::RenewalExtension,
                    notification.notification_type
                );
                assert_eq!(
                    Subtype::Summary,
                    notification.subtype.expect("Expect subtype")
                );
                assert_eq!(
                    "002e14d5-51f5-4503-b5a8-c3a1af68eb20",
                    notification.notification_uuid
                );
                assert_eq!(
                    "2.0",
                    notification.version.as_deref().expect("Expect version")
                );
                assert_eq!(
                    1698148900,
                    notification
                        .signed_date
                        .expect("Expect signed_date")
                        .timestamp()
                );
                assert!(notification.data.is_none());
                assert!(notification.summary.is_some());
                assert!(notification.external_purchase_token.is_none());

                if let Some(summary) = notification.summary {
                    assert_eq!(
                        Environment::LocalTesting,
                        summary.environment.expect("Expect environment")
                    );
                    assert_eq!(41234, summary.app_apple_id.expect("Expect app_apple_id"));
                    assert_eq!(
                        "com.example",
                        summary.bundle_id.as_deref().expect("Expect bundle_id")
                    );
                    assert_eq!(
                        "com.example.product",
                        summary.product_id.as_deref().expect("Expect product_id")
                    );
                    assert_eq!(
                        "efb27071-45a4-4aca-9854-2a1e9146f265",
                        summary.request_identifier
                    );
                    assert_eq!(vec!["CAN", "USA", "MEX"], summary.storefront_country_codes);
                    assert_eq!(5, summary.succeeded_count);
                    assert_eq!(2, summary.failed_count);
                } else {
                    panic!("Summary field is expected to be present in the notification");
                }
            }
            Err(err) => panic!(
                "Failed to verify and decode summary notification: {:?}",
                err
            ),
        }
    }

    #[test]
    fn test_xcode_signed_app_transaction() {
        let verifier = get_signed_data_verifier(Environment::Xcode, XCODE_BUNDLE_ID, None);
        let encoded_app_transaction = fs::read_to_string("resources/xcode/xcode-signed-app-transaction")
            .expect("Failed to read file");

        if let Ok(app_transaction) = verifier.verify_and_decode_app_transaction(&encoded_app_transaction) {
            assert_eq!(
                XCODE_BUNDLE_ID,
                app_transaction
                    .bundle_id
                    .as_deref()
                    .expect("Expect bundle_id")
            );
            assert_eq!(
                "1",
                app_transaction
                    .application_version
                    .as_deref()
                    .expect("Expect application_version")
            );
            assert_eq!(None, app_transaction.version_external_identifier);
            assert_eq!(
                -62135769600000,
                app_transaction
                    .original_purchase_date
                    .expect("Expect value")
                    .timestamp_millis()
            );
            assert_eq!(
                "1",
                app_transaction
                    .original_application_version
                    .as_deref()
                    .expect("Expect original_application_version")
            );
            assert_eq!(
                "cYUsXc53EbYc0pOeXG5d6/31LGHeVGf84sqSN0OrJi5u/j2H89WWKgS8N0hMsMlf",
                app_transaction
                    .device_verification
                    .as_deref()
                    .expect("Expect device_verification")
            );
            assert_eq!(
                "48c8b92d-ce0d-4229-bedf-e61b4f9cfc92",
                app_transaction
                    .device_verification_nonce
                    .expect("Expect device_verification_nonce")
                    .to_string()
            );
            assert_eq!(None, app_transaction.preorder_date);
            assert_eq!(Environment::Xcode, app_transaction.receipt_type.unwrap());
        } else {
            panic!("Failed to verify and decode app transaction");
        }
    }

    #[test]
    fn test_xcode_signed_transaction() {
        let verifier = get_signed_data_verifier(Environment::Xcode, XCODE_BUNDLE_ID, None);
        let encoded_app_transaction = fs::read_to_string("resources/xcode/xcode-signed-transaction")
            .expect("Failed to read file");

        if let Ok(transaction) = verifier.verify_and_decode_signed_transaction(&encoded_app_transaction) {
            assert_eq!(
                "0",
                transaction
                    .original_transaction_id
                    .as_deref()
                    .expect("Expect original_transaction_id")
            );
            assert_eq!(
                "0",
                transaction
                    .transaction_id
                    .as_deref()
                    .expect("Expect transaction_id")
            );
            assert_eq!(
                "0",
                transaction
                    .web_order_line_item_id
                    .as_deref()
                    .expect("Expect web_order_line_item_id")
            );
            assert_eq!(
                XCODE_BUNDLE_ID,
                transaction.bundle_id.as_deref().expect("Expect bundle_id")
            );
            assert_eq!(
                "pass.premium",
                transaction
                    .product_id
                    .as_deref()
                    .expect("Expect product_id")
            );
            assert_eq!(
                "6F3A93AB",
                transaction
                    .subscription_group_identifier
                    .as_deref()
                    .expect("Expect subscription_group_identifier")
            );
            assert_eq!(
                1697679936049,
                transaction.purchase_date.unwrap().timestamp_millis()
            );
            assert_eq!(
                1697679936049,
                transaction
                    .original_purchase_date
                    .unwrap()
                    .timestamp_millis()
            );
            assert_eq!(
                1700358336049,
                transaction.expires_date.unwrap().timestamp_millis()
            );
            assert_eq!(1, transaction.quantity.expect("Expect quantity"));
            assert_eq!(
                ProductType::AutoRenewableSubscription,
                transaction.r#type.expect("Expect type")
            );
            assert_eq!(None, transaction.app_account_token);
            assert_eq!(
                InAppOwnershipType::Purchased,
                transaction
                    .in_app_ownership_type
                    .expect("Expect in_app_ownership_type")
            );
            assert_eq!(
                1697679936056,
                transaction.signed_date.unwrap().timestamp_millis()
            );
            assert_eq!(None, transaction.revocation_reason);
            assert_eq!(None, transaction.revocation_date);
            assert!(!transaction.is_upgraded.unwrap_or(false));
            assert_eq!(
                OfferType::IntroductoryOffer,
                transaction.offer_type.expect("Expect offer_type")
            );
            assert_eq!(None, transaction.offer_identifier);
            assert_eq!(
                Environment::Xcode,
                transaction.environment.expect("Expect environment")
            );
            assert_eq!("USA", transaction.storefront.expect("Expect storefront"));
            assert_eq!(
                "143441",
                transaction
                    .storefront_id
                    .as_deref()
                    .expect("Expect storefront_id")
            );
            assert_eq!(
                TransactionReason::Purchase,
                transaction
                    .transaction_reason
                    .expect("Expect transaction_reason")
            );
        } else {
            panic!("Failed to verify and decode signed transaction");
        }
    }

    #[test]
    fn test_xcode_signed_renewal_info() {
        let verifier = get_signed_data_verifier(Environment::Xcode, XCODE_BUNDLE_ID, None);
        let encoded_renewal_info = fs::read_to_string("resources/xcode/xcode-signed-renewal-info")
            .expect("Failed to read file");

        if let Ok(renewal_info) = verifier.verify_and_decode_renewal_info(&encoded_renewal_info) {
            assert_eq!(None, renewal_info.expiration_intent);
            assert_eq!(
                "0",
                renewal_info
                    .original_transaction_id
                    .as_deref()
                    .expect("Expect original_transaction_id")
            );
            assert_eq!(
                "pass.premium",
                renewal_info
                    .auto_renew_product_id
                    .as_deref()
                    .expect("Expect auto_renew_product_id")
            );
            assert_eq!(
                "pass.premium",
                renewal_info
                    .product_id
                    .as_deref()
                    .expect("Expect product_id")
            );
            assert_eq!(
                AutoRenewStatus::On,
                renewal_info
                    .auto_renew_status
                    .expect("Expect auto_renew_status")
            );
            assert_eq!(None, renewal_info.is_in_billing_retry_period);
            assert_eq!(None, renewal_info.price_increase_status);
            assert_eq!(None, renewal_info.grace_period_expires_date);
            assert_eq!(None, renewal_info.offer_type);
            assert_eq!(None, renewal_info.offer_identifier);
            assert_eq!(
                1697679936711,
                renewal_info.signed_date.unwrap().timestamp_millis()
            );
            assert_eq!(
                Environment::Xcode,
                renewal_info.environment.expect("Expect environment")
            );
            assert_eq!(
                1697679936049,
                renewal_info
                    .recent_subscription_start_date
                    .unwrap()
                    .timestamp_millis()
            );
            assert_eq!(
                1700358336049,
                renewal_info.renewal_date.unwrap().timestamp_millis()
            );
        } else {
            panic!("Failed to verify and decode signed renewal info");
        }
    }

    #[test]
    fn test_xcode_signed_app_transaction_with_production_environment() {
        let verifier = get_signed_data_verifier(Environment::Production, XCODE_BUNDLE_ID, None);
        let encoded_app_transaction = fs::read_to_string("resources/xcode/xcode-signed-app-transaction")
            .expect("Failed to read file");

        if let Err(_) = verifier.verify_and_decode_app_transaction(&encoded_app_transaction) {
            return;
        }
        panic!("Expected VerificationException, but no exception was raised");
    }

    fn get_default_signed_data_verifier() -> SignedDataVerifier {
        return get_signed_data_verifier(Environment::LocalTesting, "com.example", None);
    }

    fn create_signed_data_from_json(path: &str) -> String {
        let json_payload = fs::read_to_string(path).expect("Failed to read JSON file");
        let json: Map<String, Value> = serde_json::from_str(json_payload.as_str()).expect("Expect JSON");

        let header = jsonwebtoken::Header::new(Algorithm::ES256);
        let private_key = generate_p256_private_key();
        let key = jsonwebtoken::EncodingKey::from_ec_der(private_key.as_ref());
        let payload = jsonwebtoken::encode(&header, &json, &key).expect("Failed to encode JWT");
        payload
    }

    fn generate_p256_private_key() -> Vec<u8> {
        let rng = ring::rand::SystemRandom::new();
        let private_key = ring::signature::EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng)
            .expect("Failed to generate private key");

        private_key.as_ref().to_vec()
    }
}
