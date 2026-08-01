use base64::DecodeError;

use crate::chain_verifier::{ChainVerificationFailureReason, ChainVerifier, ChainVerifierError};
use crate::crypto::jws;
use crate::crypto::CryptoProvider;
use crate::models::app_store_environment::Environment;
use crate::models::app_transaction::AppTransaction;
use crate::models::decoded_realtime_request_body::DecodedRealtimeRequestBody;
use crate::models::decoded_signed_data::DecodedSignedData;
use crate::models::jws_renewal_info_decoded_payload::JWSRenewalInfoDecodedPayload;
use crate::models::jws_transaction_decoded_payload::JWSTransactionDecodedPayload;
use crate::models::response_body_v2_decoded_payload::ResponseBodyV2DecodedPayload;
use crate::utils::StringExt;
use serde::de::DeserializeOwned;

#[derive(thiserror::Error, Debug)]
pub enum SignedDataVerifierError {
    #[error("VerificationFailure")]
    VerificationFailure,

    #[error("InvalidAppIdentifier")]
    InvalidAppIdentifier,

    #[error("InvalidEnvironment")]
    InvalidEnvironment,

    #[error("InvalidAppAppleId")]
    InvalidAppAppleId,

    #[error("InternalChainVerifierError")]
    InternalChainVerifierError(#[from] ChainVerifierError),

    #[error("InternalDecodeError: [{0}]")]
    InternalDecodeError(#[from] DecodeError),

    #[error("InternalDeserializationError: [{0}]")]
    InternalDeserializationError(#[from] serde_json::Error),

    #[error("InternalJWSError: [{0}]")]
    InternalJWSError(#[from] crate::crypto::jws::JwsError),
}

const EXPECTED_CHAIN_LENGTH: usize = 3;

/// A verifier for signed data, commonly used for verifying and decoding
/// signed Apple server notifications and transactions.
pub struct SignedDataVerifier {
    environment: Environment,
    bundle_id: String,
    app_apple_id: Option<i64>,
    enable_online_checks: bool,
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
    /// * `app_apple_id` - An optional Apple ID associated with the application. Required when
    ///   `environment` is `Environment::Production`.
    /// * `enable_online_checks` - Whether to enable revocation checking (OCSP) and check
    ///   certificate expiration against the current date rather than the JWS's signed date.
    ///
    /// # Returns
    ///
    /// - `Ok(SignedDataVerifier)` on success.
    /// - `Err(SignedDataVerifierError::InvalidAppAppleId)` if `environment` is
    ///   `Environment::Production` and `app_apple_id` is `None`.
    pub fn new(
        root_certificates: Vec<Vec<u8>>,
        environment: Environment,
        bundle_id: String,
        app_apple_id: Option<i64>,
        enable_online_checks: bool,
    ) -> Result<Self, SignedDataVerifierError> {
        if environment == Environment::Production && app_apple_id.is_none() {
            return Err(SignedDataVerifierError::InvalidAppAppleId);
        }

        Ok(SignedDataVerifier {
            environment,
            bundle_id,
            app_apple_id,
            enable_online_checks,
            chain_verifier: ChainVerifier::new(root_certificates),
        })
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
        let decoded_renewal_info: JWSRenewalInfoDecodedPayload = self.decode_signed_object(signed_renewal_info)?;

        if decoded_renewal_info.environment.as_ref() != Some(&self.environment) {
            return Err(SignedDataVerifierError::InvalidEnvironment);
        }

        Ok(decoded_renewal_info)
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
        } else if let Some(app_data) = &decoded_signed_notification.app_data {
            bundle_id = app_data.bundle_id.clone();
            app_apple_id = app_data.app_apple_id.clone();
            environment = app_data.environment.clone();
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
        if self.environment == Environment::LocalTesting {
            return Ok(());
        }

        if bundle_id.as_deref() != Some(self.bundle_id.as_str())
            || (self.environment == Environment::Production && self.app_apple_id != app_apple_id)
        {
            return Err(SignedDataVerifierError::InvalidAppIdentifier);
        }

        if environment.as_ref() != Some(&self.environment) {
            return Err(SignedDataVerifierError::InvalidEnvironment);
        }

        Ok(())
    }

    /// Verifies and decodes a signed app transaction.
    ///
    /// This method takes a signed app transaction string, verifies its authenticity and
    /// integrity, and returns the decoded payload as an `AppTransaction`
    /// if the verification is successful.
    ///
    /// # Arguments
    ///
    /// * `signed_app_transaction` - The signed app transaction string to verify and decode.
    ///
    /// # Returns
    ///
    /// - `Ok(AppTransaction)` if verification and decoding are successful.
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

    /// Verifies and decodes a realtime request the App Store sends to your Get Retention Message endpoint.
    ///
    /// This method takes a signed realtime request string, verifies its authenticity and
    /// integrity, and returns the decoded payload as a `DecodedRealtimeRequestBody`
    /// if the verification is successful.
    ///
    /// # Arguments
    ///
    /// * `signed_payload` - The payload the App Store server sends to your server.
    ///
    /// # Returns
    ///
    /// - `Ok(DecodedRealtimeRequestBody)` if verification and decoding are successful.
    /// - `Err(SignedDataVerifierError)` if verification or decoding fails, with error details.
    pub fn verify_and_decode_realtime_request(
        &self,
        signed_payload: &str,
    ) -> Result<DecodedRealtimeRequestBody, SignedDataVerifierError> {
        let decoded_realtime_request: DecodedRealtimeRequestBody = self.decode_signed_object(signed_payload)?;

        if self.environment == Environment::Production
            && self.app_apple_id != Some(decoded_realtime_request.app_apple_id)
        {
            return Err(SignedDataVerifierError::InvalidAppIdentifier);
        }

        if self.environment != decoded_realtime_request.environment {
            return Err(SignedDataVerifierError::InvalidEnvironment);
        }

        Ok(decoded_realtime_request)
    }

    /// Private method used for decoding a signed object (internal use).
    fn decode_signed_object<T: DeserializeOwned + DecodedSignedData>(
        &self,
        signed_obj: &str,
    ) -> Result<T, SignedDataVerifierError> {
        // Data is not signed by the App Store, and verification should be skipped.
        // The environment MUST be checked in the public method calling this.
        if self.environment == Environment::Xcode || self.environment == Environment::LocalTesting {
            let _ = jws::decode_header(signed_obj)?;
            return Ok(jws::decode_payload(signed_obj)?);
        }

        let header = jws::decode_header(signed_obj)?;

        if header.alg.as_deref() != Some("ES256") {
            return Err(SignedDataVerifierError::VerificationFailure);
        }

        let Some(x5c) = header.x5c else {
            return Err(SignedDataVerifierError::VerificationFailure);
        };

        if x5c.len() != EXPECTED_CHAIN_LENGTH {
            return Err(SignedDataVerifierError::InternalChainVerifierError(
                ChainVerifierError::VerificationFailure(
                    ChainVerificationFailureReason::InvalidChainLength,
                ),
            ));
        }

        let chain: Vec<Vec<u8>> = x5c
            .iter()
            .map(|c| c.as_der_bytes())
            .collect::<Result<_, DecodeError>>()?;

        let decoded_body: T = jws::decode_payload(signed_obj)?;

        let effective_date = if self.enable_online_checks {
            chrono::Utc::now().timestamp() as u64
        } else {
            match decoded_body.signed_date_optional() {
                Some(date) => date.timestamp() as u64,
                None => chrono::Utc::now().timestamp() as u64,
            }
        };

        let spki = self.chain_verifier.verify(
            &chain[0],
            &chain[1],
            Some(effective_date),
            self.enable_online_checks,
        )?;

        let signature_bytes = jws::decode_signature_bytes(signed_obj)?;
        let raw: [u8; 64] = signature_bytes
            .try_into()
            .map_err(|_| SignedDataVerifierError::VerificationFailure)?;

        let provider = CryptoProvider::default_provider();
        let public_key = provider
            .p256_signing
            .public_key(&spki)
            .map_err(|_| SignedDataVerifierError::VerificationFailure)?;
        let signature = provider
            .p256_signing
            .signature_from_raw(&raw)
            .map_err(|_| SignedDataVerifierError::VerificationFailure)?;

        let signing_input = jws::signing_input(signed_obj)?;
        public_key
            .is_valid_signature(signature.as_ref(), signing_input.as_bytes())
            .map_err(|_| SignedDataVerifierError::VerificationFailure)?;

        Ok(decoded_body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chain_verifier::ChainVerificationFailureReason::InvalidChainLength;

    #[test]
    fn test_invalid_chain_length() {
        // The length check happens before any signature verification, so a
        // minimal unsigned JWS with a 4-element x5c is enough to exercise it
        // through the public API.
        let header = jws::b64url_encode(br#"{"alg":"ES256","x5c":["YQ","YQ","YQ","YQ"]}"#);
        let payload = jws::b64url_encode(b"{}");
        let signature = jws::b64url_encode(b"sig");
        let jws_token = format!("{header}.{payload}.{signature}");

        let verifier = SignedDataVerifier::new(
            vec![Vec::new()],
            Environment::Production,
            "com.example".into(),
            Some(1234),
            false,
        )
        .expect("valid config");

        let result = verifier.verify_and_decode_app_transaction(&jws_token);

        assert!(matches!(
            result.expect_err("expect error"),
            SignedDataVerifierError::InternalChainVerifierError(ChainVerifierError::VerificationFailure(
                InvalidChainLength
            ))
        ));
    }
}
