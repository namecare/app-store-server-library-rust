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

    #[test]
    fn signature_verifies_against_raw_payload() {
        use p256::ecdsa::signature::Verifier;
        use p256::ecdsa::{DerSignature, SigningKey, VerifyingKey};
        use p256::pkcs8::DecodePrivateKey;

        let private_key = include_str!("../tests/resources/certs/testSigningKey.p8");
        let creator = PromotionalOfferSignatureCreator::new(
            private_key,
            "key123".to_string(),
            "com.example.app".to_string(),
        )
        .unwrap();

        let nonce = uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let sig_b64 = creator
            .create_signature("product1", "offer1", "user123", &nonce, 1234567890)
            .unwrap();
        let sig_der = BASE64_STANDARD
            .decode(&sig_b64)
            .unwrap();

        // Rebuild the exact payload the creator signed.
        let payload = creator.payload("product1", "offer1", "user123", &nonce, 1234567890);

        let signing_key = SigningKey::from_pkcs8_pem(private_key).unwrap();
        let verifying_key = VerifyingKey::from(&signing_key);
        let signature = DerSignature::try_from(sig_der.as_slice()).unwrap();

        // Verifying against the RAW payload proves exactly one SHA-256 was
        // applied. If the creator pre-hashed, this fails.
        assert!(verifying_key
            .verify(payload.as_bytes(), &signature)
            .is_ok());
    }
}
