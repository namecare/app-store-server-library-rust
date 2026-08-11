//! Framework-agnostic request handling.
//!
//! Both example servers call straight into these functions, so the only thing
//! that differs between the Axum and Actix binaries is the adapter code.

use std::sync::Arc;

use app_store_server_library::models::notification_type_v2::NotificationTypeV2;
use app_store_server_library::models::response_body_v2_decoded_payload::ResponseBodyV2DecodedPayload;
use app_store_server_library::models::subtype::Subtype;
use app_store_server_library::promotional_offer_signature_creator::PromotionalOfferSignatureCreator;
use app_store_server_library::signed_data_verifier::SignedDataVerifier;
use serde::{Deserialize, Serialize};

use crate::common::certs::load_roots;
use crate::common::config::Config;
use crate::common::error::AppError;

/// Long-lived state shared by every request.
pub struct AppState {
    pub verifier: SignedDataVerifier,
    pub promo: PromotionalOfferSignatureCreator,
    // Duplicates the key id already moved into `promo` above; kept because
    // `PromotionalOfferSignatureCreator` exposes no accessor for it.
    pub promo_key_id: String,
}

/// Builds the shared state from configuration. Called once at startup.
pub fn state(config: &Config) -> Result<Arc<AppState>, Box<dyn std::error::Error>> {
    let roots = load_roots(&config.root_source)?;

    let verifier = SignedDataVerifier::new(
        roots,
        config.environment.clone(),
        config.bundle_id.clone(),
        config.app_apple_id,
        false,
    )?;

    let promo = PromotionalOfferSignatureCreator::new(
        &config.promo_key_pem,
        config.promo_key_id.clone(),
        config.bundle_id.clone(),
    )?;

    Ok(Arc::new(AppState {
        verifier,
        promo,
        promo_key_id: config.promo_key_id.clone(),
    }))
}

#[derive(Debug, Deserialize)]
pub struct NotificationRequest {
    #[serde(rename = "signedPayload")]
    pub signed_payload: String,
}

#[derive(Debug, Serialize)]
pub struct NotificationResponse {
    #[serde(rename = "notificationType")]
    pub notification_type: NotificationTypeV2,
    pub subtype: Option<Subtype>,
    #[serde(rename = "notificationUUID")]
    pub notification_uuid: String,
}

#[derive(Debug, Deserialize)]
pub struct PromotionalOfferRequest {
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(rename = "offerId")]
    pub offer_id: String,
    #[serde(rename = "applicationUsername")]
    pub application_username: String,
}

#[derive(Debug, Serialize)]
pub struct PromotionalOfferResponse {
    pub signature: String,
    pub nonce: String,
    pub timestamp: i64,
    #[serde(rename = "keyIdentifier")]
    pub key_identifier: String,
}

/// Verifies an App Store Server Notification and dispatches on its type.
///
/// Returning `Ok` is what makes the server answer 200. Apple retries any
/// non-2xx response, so a real integration must not return 200 until the
/// notification has actually been handled durably.
pub fn handle_notification(state: &AppState, body: &NotificationRequest) -> Result<NotificationResponse, AppError> {
    if body.signed_payload.trim().is_empty() {
        return Err(AppError::BadRequest(
            "signedPayload must not be empty".into(),
        ));
    }

    let payload = state
        .verifier
        .verify_and_decode_notification(&body.signed_payload)?;

    dispatch(&payload);

    Ok(NotificationResponse {
        notification_type: payload.notification_type.clone(),
        subtype: payload.subtype.clone(),
        notification_uuid: payload.notification_uuid.clone(),
    })
}

/// Where a real integration would act on the notification. This example logs.
fn dispatch(payload: &ResponseBodyV2DecodedPayload) {
    use app_store_server_library::models::notification_type_v2::NotificationTypeV2::*;

    let bundle_id = payload
        .data
        .as_ref()
        .and_then(|data| data.bundle_id.as_deref())
        .unwrap_or("<none>");

    match &payload.notification_type {
        Subscribed => println!("[notify] new subscription for {}", bundle_id),
        DidRenew => println!("[notify] subscription renewed for {}", bundle_id),
        DidChangeRenewalStatus => {
            println!(
                "[notify] auto-renew toggled for {} ({:?})",
                bundle_id, payload.subtype
            )
        }
        DidFailToRenew => println!("[notify] renewal failed for {} - billing retry", bundle_id),
        Expired => println!("[notify] subscription expired for {}", bundle_id),
        Revoke => println!(
            "[notify] access revoked for {} - remove entitlement",
            bundle_id
        ),
        Test => println!("[notify] test notification received - endpoint is reachable"),
        other => println!(
            "[notify] unhandled notification type {:?} for {}",
            other, bundle_id
        ),
    }
}

/// Signs a subscription promotional offer for a StoreKit client.
///
/// `nonce` and `timestamp` are parameters rather than generated here so the
/// function stays deterministic and testable; the binaries supply a fresh v4
/// UUID and the current unix time per request.
pub fn handle_promotional_offer(
    state: &AppState,
    body: &PromotionalOfferRequest,
    nonce: uuid::Uuid,
    timestamp: i64,
) -> Result<PromotionalOfferResponse, AppError> {
    if body.product_id.trim().is_empty() {
        return Err(AppError::BadRequest("productId must not be empty".into()));
    }
    if body.offer_id.trim().is_empty() {
        return Err(AppError::BadRequest("offerId must not be empty".into()));
    }

    let signature = state
        .promo
        .create_signature(
            &body.product_id,
            &body.offer_id,
            &body.application_username,
            &nonce,
            timestamp,
        )
        .map_err(|error| AppError::Internal(format!("failed to sign offer: {}", error)))?;

    Ok(PromotionalOfferResponse {
        signature,
        nonce: nonce.to_string(),
        timestamp,
        key_identifier: state.promo_key_id.clone(),
    })
}

#[cfg(test)]
mod tests {
    use app_store_server_library::models::app_store_environment::Environment;

    use super::*;
    use crate::common::certs::RootSource;

    /// A config matching the bundled fixture: demo CA, com.example, sandbox.
    fn demo_config() -> Config {
        Config {
            port: 0,
            bundle_id: "com.example".to_string(),
            app_apple_id: Some(1234),
            environment: Environment::Sandbox,
            root_source: RootSource::Demo,
            promo_key_pem: include_str!("../../assets/testSigningKey.p8").to_string(),
            promo_key_id: "DEMOKEYID".to_string(),
        }
    }

    #[test]
    fn valid_fixture_notification_is_decoded() {
        let state = state(&demo_config()).unwrap();
        let signed_payload = include_str!("../../assets/testNotification")
            .trim()
            .to_string();

        let response = handle_notification(&state, &NotificationRequest { signed_payload }).unwrap();

        assert_eq!(response.notification_type, NotificationTypeV2::Test);
        assert_eq!(
            response.notification_uuid,
            "9ad56bd2-0bc6-42e0-af24-fd996d87a1e6"
        );
    }

    #[test]
    fn garbage_payload_is_unauthorized() {
        let state = state(&demo_config()).unwrap();

        let error = handle_notification(
            &state,
            &NotificationRequest {
                signed_payload: "not-a-jws".to_string(),
            },
        )
        .unwrap_err();

        assert!(matches!(error, AppError::Unauthorized(_)));
    }

    #[test]
    fn empty_payload_is_bad_request() {
        let state = state(&demo_config()).unwrap();

        let error = handle_notification(
            &state,
            &NotificationRequest {
                signed_payload: String::new(),
            },
        )
        .unwrap_err();

        assert!(matches!(error, AppError::BadRequest(_)));
    }

    #[test]
    fn promotional_offer_signature_is_base64() {
        use base64::engine::general_purpose::STANDARD;
        use base64::Engine;

        let state = state(&demo_config()).unwrap();
        let nonce = uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();

        let response = handle_promotional_offer(
            &state,
            &PromotionalOfferRequest {
                product_id: "com.example.pro".to_string(),
                offer_id: "welcome_offer".to_string(),
                application_username: "user-123".to_string(),
            },
            nonce,
            1_700_000_000,
        )
        .unwrap();

        assert!(STANDARD
            .decode(&response.signature)
            .is_ok());
        assert_eq!(response.nonce, nonce.to_string());
        assert_eq!(response.timestamp, 1_700_000_000);
        assert_eq!(response.key_identifier, "DEMOKEYID");
    }

    #[test]
    fn empty_product_id_is_bad_request() {
        let state = state(&demo_config()).unwrap();
        let nonce = uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();

        let error = handle_promotional_offer(
            &state,
            &PromotionalOfferRequest {
                product_id: String::new(),
                offer_id: "welcome_offer".to_string(),
                application_username: "user-123".to_string(),
            },
            nonce,
            1_700_000_000,
        )
        .unwrap_err();

        assert!(matches!(error, AppError::BadRequest(_)));
    }
}
