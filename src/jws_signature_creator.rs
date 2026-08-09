use crate::crypto::jws;
use crate::crypto::{CryptoError, CryptoProvider, P256PrivateKey};
use crate::models::advanced_commerce_in_app_request::AdvancedCommerceInAppRequest;
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum JWSSignatureCreatorError {
    #[error("InvalidPrivateKey")]
    InvalidPrivateKey,

    #[error("SigningError: [{0}]")]
    SigningError(String),

    #[error("SerializationError: [{0}]")]
    SerializationError(#[from] serde_json::Error),
}

impl From<CryptoError> for JWSSignatureCreatorError {
    fn from(e: CryptoError) -> Self {
        match e {
            CryptoError::KeyError(_) => Self::InvalidPrivateKey,
            CryptoError::SigningError(m) | CryptoError::VerificationError(m) => Self::SigningError(m),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct BasePayload {
    nonce: String,
    iss: String,
    bid: String,
    aud: String,
    iat: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct PromotionalOfferV2Payload {
    #[serde(flatten)]
    base: BasePayload,
    #[serde(rename = "productId")]
    product_id: String,
    #[serde(rename = "offerIdentifier")]
    offer_identifier: String,
    #[serde(rename = "transactionId", skip_serializing_if = "Option::is_none")]
    transaction_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct IntroductoryOfferEligibilityPayload {
    #[serde(flatten)]
    base: BasePayload,
    #[serde(rename = "productId")]
    product_id: String,
    #[serde(rename = "allowIntroductoryOffer")]
    allow_introductory_offer: bool,
    #[serde(rename = "transactionId")]
    transaction_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AdvancedCommerceInAppPayload {
    #[serde(flatten)]
    base: BasePayload,
    request: String,
}

/// Base struct for creating JWS signatures for App Store requests
struct JWSSignatureCreator {
    audience: String,
    signing_key: Box<dyn P256PrivateKey>,
    key_id: String,
    issuer_id: String,
    bundle_id: String,
}

impl JWSSignatureCreator {
    fn new(
        audience: String,
        signing_key: &str,
        key_id: String,
        issuer_id: String,
        bundle_id: String,
    ) -> Result<Self, JWSSignatureCreatorError> {
        let provider = CryptoProvider::default_provider();
        let key = provider
            .p256_signing
            .private_key(signing_key)
            .map_err(|_| JWSSignatureCreatorError::InvalidPrivateKey)?;

        Ok(Self {
            audience,
            signing_key: key,
            key_id,
            issuer_id,
            bundle_id,
        })
    }

    fn get_base_payload(&self) -> BasePayload {
        BasePayload {
            nonce: Uuid::new_v4().to_string(),
            iss: self.issuer_id.clone(),
            bid: self.bundle_id.clone(),
            aud: self.audience.clone(),
            iat: Utc::now().timestamp(),
        }
    }

    fn create_signature<T: Serialize>(&self, payload: &T) -> Result<String, JWSSignatureCreatorError> {
        let header = serde_json::json!({
            "alg": "ES256",
            "kid": self.key_id,
            "typ": "JWT",
        });

        let encoded_header = jws::b64url_encode(&serde_json::to_vec(&header)?);
        let encoded_payload = jws::b64url_encode(&serde_json::to_vec(payload)?);
        let signing_input = format!("{encoded_header}.{encoded_payload}");

        let (raw, _) = self.signing_key.signature(signing_input.as_bytes())?;

        Ok(jws::encode_compact(
            &encoded_header,
            &encoded_payload,
            &raw,
        ))
    }
}

/// Creator for Promotional AdvancedCommerceOffer V2 signatures
pub struct PromotionalOfferV2SignatureCreator {
    base: JWSSignatureCreator,
}

impl PromotionalOfferV2SignatureCreator {
    /// Creates a new `PromotionalOfferV2SignatureCreator` instance.
    ///
    /// # Arguments
    ///
    /// * `signing_key` - Your private key downloaded from App Store Connect (in PEM format)
    /// * `key_id` - Your key ID from the Keys page in App Store Connect
    /// * `issuer_id` - Your issuer ID from the Keys page in App Store Connect
    /// * `bundle_id` - Your app's bundle ID
    ///
    /// # Returns
    ///
    /// A `Result` containing the `PromotionalOfferV2SignatureCreator` instance or an error.
    pub fn new(
        signing_key: &str,
        key_id: String,
        issuer_id: String,
        bundle_id: String,
    ) -> Result<Self, JWSSignatureCreatorError> {
        let base = JWSSignatureCreator::new(
            "promotional-offer".to_string(),
            signing_key,
            key_id,
            issuer_id,
            bundle_id,
        )?;

        Ok(Self { base })
    }

    /// Creates a promotional offer V2 signature.
    ///
    /// # Arguments
    ///
    /// * `product_id` - The unique identifier of the product
    /// * `offer_identifier` - The promotional offer identifier that you set up in App Store Connect
    /// * `transaction_id` - The unique identifier of any transaction that belongs to the customer.
    ///   You can use the customer's appTransactionId, even for customers who haven't made any
    ///   In-App Purchases in your app. This field is optional, but recommended.
    ///
    /// # Returns
    ///
    /// A `Result` containing the signed JWS string or an error.
    ///
    /// # References
    ///
    /// [Generating JWS to sign App Store requests](https://developer.apple.com/documentation/storekit/generating-jws-to-sign-app-store-requests)
    pub fn create_signature(
        &self,
        product_id: &str,
        offer_identifier: &str,
        transaction_id: Option<String>,
    ) -> Result<String, JWSSignatureCreatorError> {
        let base_payload = self.base.get_base_payload();
        let payload = PromotionalOfferV2Payload {
            base: base_payload,
            product_id: product_id.to_string(),
            offer_identifier: offer_identifier.to_string(),
            transaction_id,
        };

        self.base.create_signature(&payload)
    }
}

/// Creator for Introductory AdvancedCommerceOffer Eligibility signatures
pub struct IntroductoryOfferEligibilitySignatureCreator {
    base: JWSSignatureCreator,
}

impl IntroductoryOfferEligibilitySignatureCreator {
    /// Creates a new `IntroductoryOfferEligibilitySignatureCreator` instance.
    ///
    /// # Arguments
    ///
    /// * `signing_key` - Your private key downloaded from App Store Connect (in PEM format)
    /// * `key_id` - Your key ID from the Keys page in App Store Connect
    /// * `issuer_id` - Your issuer ID from the Keys page in App Store Connect
    /// * `bundle_id` - Your app's bundle ID
    ///
    /// # Returns
    ///
    /// A `Result` containing the `IntroductoryOfferEligibilitySignatureCreator` instance or an error.
    pub fn new(
        signing_key: &str,
        key_id: String,
        issuer_id: String,
        bundle_id: String,
    ) -> Result<Self, JWSSignatureCreatorError> {
        let base = JWSSignatureCreator::new(
            "introductory-offer-eligibility".to_string(),
            signing_key,
            key_id,
            issuer_id,
            bundle_id,
        )?;

        Ok(Self { base })
    }

    /// Creates an introductory offer eligibility signature.
    ///
    /// # Arguments
    ///
    /// * `product_id` - The unique identifier of the product
    /// * `allow_introductory_offer` - A boolean value that determines whether the customer
    ///   is eligible for an introductory offer
    /// * `transaction_id` - The unique identifier of any transaction that belongs to the customer.
    ///   You can use the customer's appTransactionId, even for customers who haven't made any
    ///   In-App Purchases in your app.
    ///
    /// # Returns
    ///
    /// A `Result` containing the signed JWS string or an error.
    ///
    /// # References
    ///
    /// [Generating JWS to sign App Store requests](https://developer.apple.com/documentation/storekit/generating-jws-to-sign-app-store-requests)
    pub fn create_signature(
        &self,
        product_id: &str,
        allow_introductory_offer: bool,
        transaction_id: &str,
    ) -> Result<String, JWSSignatureCreatorError> {
        let base_payload = self.base.get_base_payload();
        let payload = IntroductoryOfferEligibilityPayload {
            base: base_payload,
            product_id: product_id.to_string(),
            allow_introductory_offer,
            transaction_id: transaction_id.to_string(),
        };

        self.base.create_signature(&payload)
    }
}

/// Creator for Advanced Commerce In-App signatures
pub struct AdvancedCommerceInAppSignatureCreator {
    base: JWSSignatureCreator,
}

impl AdvancedCommerceInAppSignatureCreator {
    /// Creates a new `AdvancedCommerceInAppSignatureCreator` instance.
    ///
    /// # Arguments
    ///
    /// * `signing_key` - Your private key downloaded from App Store Connect (in PEM format)
    /// * `key_id` - Your key ID from the Keys page in App Store Connect
    /// * `issuer_id` - Your issuer ID from the Keys page in App Store Connect
    /// * `bundle_id` - Your app's bundle ID
    ///
    /// # Returns
    ///
    /// A `Result` containing the `AdvancedCommerceInAppSignatureCreator` instance or an error.
    pub fn new(
        signing_key: &str,
        key_id: String,
        issuer_id: String,
        bundle_id: String,
    ) -> Result<Self, JWSSignatureCreatorError> {
        let base = JWSSignatureCreator::new(
            "advanced-commerce-api".to_string(),
            signing_key,
            key_id,
            issuer_id,
            bundle_id,
        )?;

        Ok(Self { base })
    }

    /// Creates an Advanced Commerce in-app signed request.
    ///
    /// # Arguments
    ///
    /// * `advanced_commerce_in_app_request` - The request to be signed.
    ///
    /// # Returns
    ///
    /// A `Result` containing the signed JWS string or an error.
    ///
    /// # References
    ///
    /// [Generating JWS to sign App Store requests](https://developer.apple.com/documentation/storekit/generating-jws-to-sign-app-store-requests)
    pub fn create_signature<T: AdvancedCommerceInAppRequest>(
        &self,
        advanced_commerce_in_app_request: &T,
    ) -> Result<String, JWSSignatureCreatorError> {
        let json_data = serde_json::to_vec(advanced_commerce_in_app_request)?;
        let base64_encoded_body = BASE64.encode(&json_data);

        let base_payload = self.base.get_base_payload();
        let payload = AdvancedCommerceInAppPayload {
            base: base_payload,
            request: base64_encoded_body,
        };

        self.base.create_signature(&payload)
    }
}
