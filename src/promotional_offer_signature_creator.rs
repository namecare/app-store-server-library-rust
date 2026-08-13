use base64::prelude::BASE64_STANDARD;
use base64::Engine;

use crate::crypto::{CryptoError, CryptoProvider, P256PrivateKey};

#[derive(thiserror::Error, Debug)]
pub enum PromotionalOfferSignatureCreatorError {
    #[error("Key error: {0}")]
    KeyError(String),
    #[error("Signing error: {0}")]
    SigningError(String),
}

impl From<CryptoError> for PromotionalOfferSignatureCreatorError {
    fn from(e: CryptoError) -> Self {
        match e {
            CryptoError::KeyError(m) => Self::KeyError(m),
            CryptoError::SigningError(m) => Self::SigningError(m),
            CryptoError::VerificationError(m) => Self::SigningError(m),
        }
    }
}

pub struct PromotionalOfferSignatureCreator {
    key: Box<dyn P256PrivateKey>,
    key_id: String,
    bundle_id: String,
}

impl PromotionalOfferSignatureCreator {
    pub fn new(
        private_key: &str,
        key_id: String,
        bundle_id: String,
    ) -> Result<Self, PromotionalOfferSignatureCreatorError> {
        let provider = CryptoProvider::default_provider();
        let key = provider
            .p256_signing
            .private_key(private_key)?;

        Ok(Self { key, key_id, bundle_id })
    }

    pub fn create_signature(
        &self,
        product_identifier: &str,
        subscription_offer_id: &str,
        app_account_token: &str,
        nonce: &uuid::Uuid,
        timestamp: i64,
    ) -> Result<String, PromotionalOfferSignatureCreatorError> {
        let payload = self.payload(
            product_identifier,
            subscription_offer_id,
            app_account_token,
            nonce,
            timestamp,
        );

        // The backend's `sign` applies SHA-256 internally, so pass the raw
        // payload. Pre-hashing here would sign SHA256(SHA256(payload)) and
        // produce signatures Apple rejects.
        let (_, der) = self.key.signature(payload.as_bytes())?;
        Ok(BASE64_STANDARD.encode(der))
    }

    fn payload(
        &self,
        product_identifier: &str,
        subscription_offer_id: &str,
        app_account_token: &str,
        nonce: &uuid::Uuid,
        timestamp: i64,
    ) -> String {
        format!(
            "{}\u{2063}{}\u{2063}{}\u{2063}{}\u{2063}{}\u{2063}{}\u{2063}{}",
            self.bundle_id,
            self.key_id,
            product_identifier,
            subscription_offer_id,
            app_account_token.to_lowercase(),
            nonce.to_string().to_lowercase(),
            timestamp
        )
    }
}
