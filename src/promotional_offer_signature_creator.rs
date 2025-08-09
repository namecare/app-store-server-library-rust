use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use pem_rfc7468::{decode};
use ring::signature::{EcdsaKeyPair, Signature, ECDSA_P256_SHA256_ASN1_SIGNING};
use ring::{error, rand};
use std::fmt::{Display, Formatter};
use thiserror::Error;
use x509_parser::nom::AsBytes;

#[derive(Error, Debug)]
pub struct KeyRejectedWrapped(error::KeyRejected);

impl PartialEq for KeyRejectedWrapped {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_string() == other.0.to_string()
    }
}

impl Display for KeyRejectedWrapped {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum PromotionalOfferSignatureCreatorError {
    #[error("UnspecifiedRingError: [{0}]")]
    UnspecifiedRingError(#[from] error::Unspecified),

    #[error("KeyRejectedError: [{0}]")]
    KeyRejectedError(#[from] KeyRejectedWrapped),

    #[error("InternalPemError: [{0}]")]
    InternalPemError(#[from] pem_rfc7468::Error),
}

/// Struct responsible for creating promotional offer signatures.
pub struct PromotionalOfferSignatureCreator {
    ec_private_key: EcdsaKeyPair,
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
        let (label, private_key) = pem_rfc7468::decode(private_key.as_bytes(), &mut buf)?;
        let alg = &ECDSA_P256_SHA256_ASN1_SIGNING;
        let rng = rand::SystemRandom::new();

        let ec_private_key =
            EcdsaKeyPair::from_pkcs8(alg, private_key, &rng).map_err(KeyRejectedWrapped)?;

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
        let signature = self.sign(payload.as_str())?;
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

    fn sign(&self, payload: &str) -> Result<Signature, PromotionalOfferSignatureCreatorError> {
        Ok(self
            .ec_private_key
            .sign(&ring::rand::SystemRandom::new(), payload.as_bytes())?)
    }

    #[cfg(test)]
    fn public_key(&self) -> Vec<u8> {
        use ring::signature::KeyPair;

        return self.ec_private_key.public_key().as_ref().to_vec();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::system_timestamp;
    use ring::signature::{UnparsedPublicKey, ECDSA_P256_SHA256_ASN1};

    #[test]
    fn test_promotional_offer_signature_creator() {
        let private_key = include_str!("../resources/certs/testSigningKey.p8");
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
                uuid::Uuid::new_v4().to_string().as_str(),
                &uuid::Uuid::new_v4(),
                i64::try_from(system_timestamp()).unwrap(),
            )
            .unwrap();

        assert!(!r.is_empty())
    }

    #[test]
    fn test_promotional_offer_signature_creator_verified() {
        let private_key = include_str!("../resources/certs/testSigningKey.p8");
        let creator = PromotionalOfferSignatureCreator::new(
            private_key,
            "L256SYR32L".to_string(),
            "com.test.app".to_string(),
        )
        .unwrap();
        let payload = creator.payload(
            "com.test.product",
            "com.test.offer",
            uuid::Uuid::new_v4().to_string().as_str(),
            &uuid::Uuid::new_v4(),
            i64::try_from(system_timestamp()).unwrap(),
        );
        let signature = creator.sign(payload.as_str()).unwrap();

        // Verify
        let public_key = creator.public_key();
        let public_key = UnparsedPublicKey::new(&ECDSA_P256_SHA256_ASN1, public_key.as_slice());
        assert_eq!(
            (),
            public_key
                .verify(payload.as_bytes(), signature.as_ref())
                .unwrap()
        );
    }
}
