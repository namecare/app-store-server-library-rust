use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use sha2::{Digest, Sha256};

use crate::crypto::CryptoProvider;

#[derive(thiserror::Error, Debug)]
pub enum PromotionalOfferSignatureCreatorError {
    #[error("Key error: {0}")]
    KeyError(String),
    #[error("Signing error: {0}")]
    SigningError(String),
}

/// Trait for ECDSA P-256 signing used by promotional offers.
/// Implementations handle PEM parsing and actual cryptographic operations.
pub trait PromotionalOfferSigner: Send + Sync {
    /// Signs a message and returns the DER-encoded signature
    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, PromotionalOfferSignatureCreatorError>;
}

pub struct PromotionalOfferSignatureCreator {
    signer: Box<dyn PromotionalOfferSigner>,
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
        let signer = (provider.promotional_offer_signer)(private_key)?;

        Ok(Self {
            signer,
            key_id,
            bundle_id,
        })
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

        let mut hasher = Sha256::new();
        hasher.update(payload.as_bytes());
        let payload_hash = hasher.finalize();

        let signature = self.signer.sign(&payload_hash)?;
        Ok(BASE64_STANDARD.encode(&signature))
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

#[cfg(all(test, feature = "rust_crypto"))]
mod tests {
    use super::*;

    #[test]
    fn test_create_signature() {
        let private_key = include_str!("../tests/resources/certs/testSigningKey.p8");
        let creator = PromotionalOfferSignatureCreator::new(
            private_key,
            "key123".to_string(),
            "com.example.app".to_string(),
        )
        .unwrap();

        let nonce = uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let result = creator.create_signature("product1", "offer1", "user123", &nonce, 1234567890);

        assert!(result.is_ok());
        let sig = result.unwrap();
        assert!(!sig.is_empty());
        assert!(BASE64_STANDARD.decode(&sig).is_ok());
    }
}
