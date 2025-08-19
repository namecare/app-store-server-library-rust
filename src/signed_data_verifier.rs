use base64::engine::general_purpose::STANDARD;
use base64::{DecodeError, Engine};

use crate::chain_verifier::{ChainVerifier, ChainVerifierError};
use crate::primitives::app_transaction::AppTransaction;
use crate::primitives::environment::Environment;
use crate::primitives::jws_renewal_info_decoded_payload::JWSRenewalInfoDecodedPayload;
use crate::primitives::jws_transaction_decoded_payload::JWSTransactionDecodedPayload;
use crate::primitives::response_body_v2_decoded_payload::ResponseBodyV2DecodedPayload;
use crate::utils::{base64_url_to_base64, StringExt};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde::de::DeserializeOwned;
use crate::chain_verifier::ChainVerificationFailureReason::InvalidChainLength;

#[derive(thiserror::Error, Debug)]
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

    #[error("InternalDeserializationError: [{0}]")]
    InternalDeserializationError(#[from] serde_json::Error),

    #[error("InternalJWTError: [{0}]")]
    InternalJWTError(#[from] jsonwebtoken::errors::Error),
}

const EXPECTED_CHAIN_LENGTH: usize = 3;

/// A verifier for signed data, commonly used for verifying and decoding
/// signed Apple server notifications and transactions.
pub struct SignedDataVerifier {
    environment: Environment,
    bundle_id: String,
    app_apple_id: Option<i64>,
    chain_verifier: ChainVerifier,
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
        let chain_verifier = ChainVerifier::new(root_certificates);

        SignedDataVerifier {
            environment,
            bundle_id,
            app_apple_id,
            chain_verifier
        }
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
            bundle_id = external_purchase_token
                .bundle_id
                .clone();
            app_apple_id = external_purchase_token
                .app_apple_id
                .clone();

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

        if decoded_app_transaction
            .bundle_id
            .as_ref()
            != Some(&self.bundle_id)
        {
            return Err(SignedDataVerifierError::InvalidAppIdentifier);
        }

        if decoded_app_transaction
            .receipt_type
            .as_ref()
            != Some(&self.environment)
        {
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
            let body_base64 = base64_url_to_base64(body_segments[1]);
            let body_data = STANDARD.decode(body_base64)?;
            let decoded_body = serde_json::from_slice(&body_data)?;
            return Ok(decoded_body);
        }

        let header = jsonwebtoken::decode_header(signed_obj)?;

        let Some(x5c) = header.x5c else {
            return Err(SignedDataVerifierError::VerificationFailure);
        };

        if x5c.is_empty() {
            return Err(SignedDataVerifierError::VerificationFailure);
        }

        let x5c: Result<Vec<Vec<u8>>, DecodeError> = x5c
            .iter()
            .map(|c| c.as_der_bytes())
            .collect();
        let chain = x5c?;

        if header.alg != Algorithm::ES256 {
            return Err(SignedDataVerifierError::VerificationFailure);
        }

        let pub_key = self.verify_chain(&chain, None)?;
        let pub_key = &pub_key[pub_key.len() - 65..];

        let decoding_key = DecodingKey::from_ec_der(pub_key);
        let claims: [&str; 0] = [];

        let mut validator = Validation::new(Algorithm::ES256);
        validator.validate_exp = false;
        validator.set_required_spec_claims(&claims);

        let payload = jsonwebtoken::decode::<T>(signed_obj, &decoding_key, &validator)?;
        Ok(payload.claims)
    }

    fn verify_chain(&self, chain: &Vec<Vec<u8>>, effective_date: Option<u64>) -> Result<Vec<u8>, ChainVerifierError> {
        if chain.len() != EXPECTED_CHAIN_LENGTH {
            return Err(ChainVerifierError::VerificationFailure(InvalidChainLength))
        }

        let leaf = &chain[0];
        let intermediate = &chain[1];

        Ok(self.chain_verifier.verify(leaf, intermediate, effective_date)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::chain_verifier::ChainVerificationFailureReason::InvalidChainLength;
    use super::*;

    #[test]
    fn test_invalid_chain_length() -> Result<(), ChainVerifierError> {
        let root = Vec::new();
        let leaf = Vec::new();
        let intermediate = Vec::new();

        let chain = vec![leaf, intermediate, Vec::new(), Vec::new()];
        let verifier = SignedDataVerifier::new(vec![root], Environment::Production, "com.example".into(), Some(1234));
        let public_key = verifier.verify_chain(&chain, None);

        assert!(
            matches!(
                public_key.expect_err("Expect error"),
                ChainVerifierError::VerificationFailure(InvalidChainLength)
            )
        );
        Ok(())
    }
}
