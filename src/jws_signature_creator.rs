use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum JWSSignatureCreatorError {
    #[error("InvalidPrivateKey")]
    InvalidPrivateKey,

    #[error("JWTEncodingError: [{0}]")]
    JWTEncodingError(#[from] jsonwebtoken::errors::Error),

    #[error("SerializationError: [{0}]")]
    SerializationError(#[from] serde_json::Error),
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

/// Trait for Advanced Commerce in-app requests
pub trait AdvancedCommerceInAppRequest: Serialize {}

/// Base struct for creating JWS signatures for App Store requests
struct JWSSignatureCreator {
    audience: String,
    signing_key: EncodingKey,
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
        let key = EncodingKey::from_ec_pem(signing_key.as_bytes())
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
        let mut header = Header::new(Algorithm::ES256);
        header.kid = Some(self.key_id.clone());
        header.typ = Some("JWT".to_string());

        let token = encode(&header, payload, &self.signing_key)?;
        Ok(token)
    }
}

/// Creator for Promotional Offer V2 signatures
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

/// Creator for Introductory Offer Eligibility signatures
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