use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use p256::ecdsa::signature::Signer;
use p256::ecdsa::{DerSignature, SigningKey};
use p256::pkcs8::DecodePrivateKey;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum PromotionalOfferSignatureCreatorError {
    #[error("SignatureError: [{0}]")]
    SignatureError(String),

    #[error("KeyRejectedError: [{0}]")]
    KeyRejectedError(String),

    #[error("InternalPemError: [{0}]")]
    InternalPemError(#[from] pem_rfc7468::Error),
}

/// Struct responsible for creating promotional offer signatures.
pub struct PromotionalOfferSignatureCreator {
    ec_private_key: SigningKey,
    key_id: String,
    bundle_id: String,
}

impl PromotionalOfferSignatureCreator {
    /// Creates a new `PromotionalOfferSignatureCreator` instance.
    ///
    /// # Arguments
    ///
    /// * `private_key`: A PEM-encoded private key.
    /// * `key_id`: A String representing the key ID.
    /// * `bundle_id`: A String representing the bundle ID.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `PromotionalOfferSignatureCreator` instance or an error.
    pub fn new(
        private_key: &str,
        key_id: String,
        bundle_id: String,
    ) -> Result<Self, PromotionalOfferSignatureCreatorError> {
        let mut buf = [0u8; 2048];
        let (_, private_key_der) = pem_rfc7468::decode(private_key.as_bytes(), &mut buf)?;
        let ec_private_key = SigningKey::from_pkcs8_der(private_key_der)
            .map_err(|e| PromotionalOfferSignatureCreatorError::KeyRejectedError(e.to_string()))?;

        Ok(PromotionalOfferSignatureCreator {
            ec_private_key,
            key_id,
            bundle_id,
        })
    }

    /// Creates a digital signature for a promotional offer.
    ///
    /// # Arguments
    ///
    /// * `product_identifier`: The product identifier.
    /// * `subscription_offer_id`: The subscription offer identifier.
    /// * `application_username`: The application username.
    /// * `nonce`: A UUID representing a unique value.
    /// * `timestamp`: A timestamp.
    ///
    /// # Returns
    ///
    /// A `Result` containing the Base64-encoded signature or an error.
    pub fn create_signature(
        &self,
        product_identifier: &str,
        subscription_offer_id: &str,
        application_username: &str,
        nonce: &uuid::Uuid,
        timestamp: i64,
    ) -> Result<String, PromotionalOfferSignatureCreatorError> {
        let payload = self.payload(
            product_identifier,
            subscription_offer_id,
            application_username,
            nonce,
            timestamp,
        );
        let signature = self.sign(payload.as_str());
        let signature_base64 = BASE64_STANDARD.encode(signature.as_ref());

        Ok(signature_base64)
    }

    fn payload(
        &self,
        product_identifier: &str,
        subscription_offer_id: &str,
        application_username: &str,
        nonce: &uuid::Uuid,
        timestamp: i64,
    ) -> String {
        format!(
            "{}\u{2063}{}\u{2063}{}\u{2063}{}\u{2063}{}\u{2063}{}\u{2063}{}",
            self.bundle_id,
            self.key_id,
            product_identifier,
            subscription_offer_id,
            application_username.to_lowercase(),
            nonce.to_string().to_lowercase(),
            timestamp
        )
    }

    fn sign(&self, payload: &str) -> DerSignature {
        self.ec_private_key.sign(payload.as_bytes())
    }

    #[cfg(test)]
    fn verifying_key(&self) -> p256::ecdsa::VerifyingKey {
        p256::ecdsa::VerifyingKey::from(&self.ec_private_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use p256::ecdsa::signature::Verifier;

    #[test]
    fn test_promotional_offer_signature_creator_verified() {
        let private_key = include_str!("../tests/resources/certs/testSigningKey.p8");
        let creator = PromotionalOfferSignatureCreator::new(
            private_key,
            "L256SYR32L".to_string(),
            "com.test.app".to_string(),
        )
        .unwrap();
        let payload = creator.payload(
            "com.test.product",
            "com.test.offer",
            uuid::Uuid::new_v4()
                .to_string()
                .as_str(),
            &uuid::Uuid::new_v4(),
            12345,
        );
        let signature = creator.sign(payload.as_str());

        // Verify
        let verifying_key = creator.verifying_key();
        verifying_key
            .verify(payload.as_bytes(), &signature)
            .unwrap();
    }
}