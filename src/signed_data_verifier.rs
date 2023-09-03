use base64::{DecodeError, Engine};
use base64::engine::general_purpose::STANDARD;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde::de::DeserializeOwned;
use crate::chain_verifier::{ChainVerifierError, verify_chain};
use crate::primitives::environment::Environment;
use crate::primitives::jws_transaction_decoded_payload::JWSTransactionDecodedPayload;
use crate::primitives::response_body_v2_decoded_payload::ResponseBodyV2DecodedPayload;
use crate::utils::StringExt;

#[derive(thiserror::Error, Debug, PartialEq)]
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
    InternalDecodeError(#[from] base64::DecodeError),

    #[error("InternalJWTError: [{0}]")]
    InternalJWTError(#[from] jsonwebtoken::errors::Error)
}

pub struct SignedDataVerifier {
    root_certificates: Vec<Vec<u8>>,
    environment: Environment,
    bundle_id: String,
    app_apple_id: Option<i64>,
}

impl SignedDataVerifier {
    pub fn new(root_certificates: Vec<Vec<u8>>,
           environment: Environment,
           bundle_id: String,
           app_apple_id: Option<i64>,
    ) -> Self {
        return SignedDataVerifier {
            root_certificates,
            environment,
            bundle_id,
            app_apple_id,
        };
    }
}

impl SignedDataVerifier {
    pub fn verify_and_decode_signed_transaction(&self, signed_transaction: &str) -> Result<JWSTransactionDecodedPayload, SignedDataVerifierError> {
        let decoded_signed_tx: JWSTransactionDecodedPayload  = self.decode_signed_object(signed_transaction)?;

        if decoded_signed_tx.bundle_id.as_ref() != Some(&self.bundle_id) {
            return Err(SignedDataVerifierError::InvalidAppIdentifier)
        }

        if decoded_signed_tx.environment.as_ref() != Some(&self.environment) {
            return Err(SignedDataVerifierError::InvalidEnvironment)
        }

        Ok(decoded_signed_tx)
    }
    pub fn verify_and_decode_notification(&self, signed_payload: &str) -> Result<ResponseBodyV2DecodedPayload, SignedDataVerifierError> {
        let decoded_signed_notification: ResponseBodyV2DecodedPayload  = self.decode_signed_object(signed_payload)?;

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
        } else {
            return Err(SignedDataVerifierError::InvalidAppIdentifier)
        }

        if bundle_id.as_ref() != Some(&self.bundle_id) || (self.environment == Environment::Production && app_apple_id.as_ref() != self.app_apple_id.as_ref() ) {
            return Err(SignedDataVerifierError::InvalidAppIdentifier)
        }

        if environment.as_ref() != Some(&self.environment) {
            return Err(SignedDataVerifierError::InvalidEnvironment)
        }

        Ok(decoded_signed_notification)
    }

    fn decode_signed_object<T: DeserializeOwned>(&self, signed_obj: &str) -> Result<T, SignedDataVerifierError> {
        let header = jsonwebtoken::decode_header(signed_obj)?;

        let Some(x5c) = header.x5c else {
            return Err(SignedDataVerifierError::VerificationFailure);
        };

        if x5c.is_empty() {
            return Err(SignedDataVerifierError::VerificationFailure);
        }

        let x5c: Result<Vec<Vec<u8>>, DecodeError> = x5c.iter().map(|c| c.as_der_bytes()).collect();
        let chain = x5c?;

        if header.alg != Algorithm::ES256 {
            return Err(SignedDataVerifierError::VerificationFailure);
        }

        let pub_key = verify_chain(&chain, &self.root_certificates, None)?;
        let pub_key = &pub_key[pub_key.len() - 65..];

        let decoding_key = DecodingKey::from_ec_der(pub_key);
        let claims: [&str; 0] = [];

        let mut validator = Validation::new(Algorithm::ES256);
        validator.validate_exp = false;
        validator.set_required_spec_claims(&claims);

        let payload = jsonwebtoken::decode::<T>(signed_obj, &decoding_key, &validator).expect("Expect Payload");
        return Ok(payload.claims);
    }
}

#[cfg(test)]
mod tests {
    use base64::Engine;
    use base64::engine::general_purpose::STANDARD;
    use jsonwebtoken::errors::ErrorKind;
    use crate::primitives::notification_type_v2::NotificationTypeV2;
    use super::*;

    const ROOT_CA_BASE64_ENCODED: &str = "MIIBgjCCASmgAwIBAgIJALUc5ALiH5pbMAoGCCqGSM49BAMDMDYxCzAJBgNVBAYTAlVTMRMwEQYDVQQIDApDYWxpZm9ybmlhMRIwEAYDVQQHDAlDdXBlcnRpbm8wHhcNMjMwMTA1MjEzMDIyWhcNMzMwMTAyMjEzMDIyWjA2MQswCQYDVQQGEwJVUzETMBEGA1UECAwKQ2FsaWZvcm5pYTESMBAGA1UEBwwJQ3VwZXJ0aW5vMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEc+/Bl+gospo6tf9Z7io5tdKdrlN1YdVnqEhEDXDShzdAJPQijamXIMHf8xWWTa1zgoYTxOKpbuJtDplz1XriTaMgMB4wDAYDVR0TBAUwAwEB/zAOBgNVHQ8BAf8EBAMCAQYwCgYIKoZIzj0EAwMDRwAwRAIgemWQXnMAdTad2JDJWng9U4uBBL5mA7WI05H7oH7c6iQCIHiRqMjNfzUAyiu9h6rOU/K+iTR0I/3Y/NSWsXHX+acc";

    const TEST_NOTIFICATION: &str = "eyJ4NWMiOlsiTUlJQm9EQ0NBVWFnQXdJQkFnSUJDekFLQmdncWhrak9QUVFEQWpCTk1Rc3dDUVlEVlFRR0V3SlZVekVUTUJFR0ExVUVDQXdLUTJGc2FXWnZjbTVwWVRFU01CQUdBMVVFQnd3SlEzVndaWEowYVc1dk1SVXdFd1lEVlFRS0RBeEpiblJsY20xbFpHbGhkR1V3SGhjTk1qTXdNVEEwTVRZek56TXhXaGNOTXpJeE1qTXhNVFl6TnpNeFdqQkZNUXN3Q1FZRFZRUUdFd0pWVXpFVE1CRUdBMVVFQ0F3S1EyRnNhV1p2Y201cFlURVNNQkFHQTFVRUJ3d0pRM1Z3WlhKMGFXNXZNUTB3Q3dZRFZRUUtEQVJNWldGbU1Ga3dFd1lIS29aSXpqMENBUVlJS29aSXpqMERBUWNEUWdBRTRyV0J4R21GYm5QSVBRSTB6c0JLekx4c2o4cEQydnFicjB5UElTVXgyV1F5eG1yTnFsOWZoSzhZRUV5WUZWNysrcDVpNFlVU1Ivbzl1UUlnQ1BJaHJLTWZNQjB3Q1FZRFZSMFRCQUl3QURBUUJnb3Foa2lHOTJOa0Jnc0JCQUlUQURBS0JnZ3Foa2pPUFFRREFnTklBREJGQWlFQWtpRVprb0ZNa2o0Z1huK1E5alhRWk1qWjJnbmpaM2FNOE5ZcmdmVFVpdlFDSURKWVowRmFMZTduU0lVMkxXTFRrNXRYVENjNEU4R0pTWWYvc1lSeEVGaWUiLCJNSUlCbHpDQ0FUMmdBd0lCQWdJQkJqQUtCZ2dxaGtqT1BRUURBakEyTVFzd0NRWURWUVFHRXdKVlV6RVRNQkVHQTFVRUNBd0tRMkZzYVdadmNtNXBZVEVTTUJBR0ExVUVCd3dKUTNWd1pYSjBhVzV2TUI0WERUSXpNREV3TkRFMk1qWXdNVm9YRFRNeU1USXpNVEUyTWpZd01Wb3dUVEVMTUFrR0ExVUVCaE1DVlZNeEV6QVJCZ05WQkFnTUNrTmhiR2xtYjNKdWFXRXhFakFRQmdOVkJBY01DVU4xY0dWeWRHbHViekVWTUJNR0ExVUVDZ3dNU1c1MFpYSnRaV1JwWVhSbE1Ga3dFd1lIS29aSXpqMENBUVlJS29aSXpqMERBUWNEUWdBRUZRM2xYMnNxTjlHSXdBaWlNUURRQy9reW5TZ1g0N1J3dmlET3RNWFh2eUtkUWU2Q1BzUzNqbzJ1UkR1RXFBeFdlT2lDcmpsRFdzeXo1d3dkVTBndGFxTWxNQ013RHdZRFZSMFRCQWd3QmdFQi93SUJBREFRQmdvcWhraUc5Mk5rQmdJQkJBSVRBREFLQmdncWhrak9QUVFEQWdOSUFEQkZBaUVBdm56TWNWMjY4Y1JiMS9GcHlWMUVoVDNXRnZPenJCVVdQNi9Ub1RoRmF2TUNJRmJhNXQ2WUt5MFIySkR0eHF0T2pKeTY2bDZWN2QvUHJBRE5wa21JUFcraSIsIk1JSUJYRENDQVFJQ0NRQ2ZqVFVHTERuUjlqQUtCZ2dxaGtqT1BRUURBekEyTVFzd0NRWURWUVFHRXdKVlV6RVRNQkVHQTFVRUNBd0tRMkZzYVdadmNtNXBZVEVTTUJBR0ExVUVCd3dKUTNWd1pYSjBhVzV2TUI0WERUSXpNREV3TkRFMk1qQXpNbG9YRFRNek1ERXdNVEUyTWpBek1sb3dOakVMTUFrR0ExVUVCaE1DVlZNeEV6QVJCZ05WQkFnTUNrTmhiR2xtYjNKdWFXRXhFakFRQmdOVkJBY01DVU4xY0dWeWRHbHViekJaTUJNR0J5cUdTTTQ5QWdFR0NDcUdTTTQ5QXdFSEEwSUFCSFB2d1pmb0tMS2FPclgvV2U0cU9iWFNuYTVUZFdIVlo2aElSQTF3MG9jM1FDVDBJbzJwbHlEQjMvTVZsazJ0YzRLR0U4VGlxVzdpYlE2WmM5VjY0azB3Q2dZSUtvWkl6ajBFQXdNRFNBQXdSUUloQU1USGhXdGJBUU4waFN4SVhjUDRDS3JEQ0gvZ3N4V3B4NmpUWkxUZVorRlBBaUIzNW53azVxMHpjSXBlZnZZSjBNVS95R0dIU1dlejBicTBwRFlVTy9ubUR3PT0iXSwidHlwIjoiSldUIiwiYWxnIjoiRVMyNTYifQ.eyJkYXRhIjp7ImFwcEFwcGxlSWQiOjEyMzQsImVudmlyb25tZW50IjoiU2FuZGJveCIsImJ1bmRsZUlkIjoiY29tLmV4YW1wbGUifSwibm90aWZpY2F0aW9uVVVJRCI6IjlhZDU2YmQyLTBiYzYtNDJlMC1hZjI0LWZkOTk2ZDg3YTFlNiIsInNpZ25lZERhdGUiOjE2ODEzMTQzMjQwMDAsIm5vdGlmaWNhdGlvblR5cGUiOiJURVNUIn0.VVXYwuNm2Y3XsOUva-BozqatRCsDuykA7xIe_CCRw6aIAAxJ1nb2sw871jfZ6dcgNhUuhoZ93hfbc1v_5zB7Og";
    const MISSING_X5C_HEADER_CLAIM: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJFUzI1NiIsIng1Y3dyb25nIjpbIk1JSUJvRENDQVVhZ0F3SUJBZ0lCRERBS0JnZ3Foa2pPUFFRREF6QkZNUXN3Q1FZRFZRUUdFd0pWVXpFTE1Ba0dBMVVFQ0F3Q1EwRXhFakFRQmdOVkJBY01DVU4xY0dWeWRHbHViekVWTUJNR0ExVUVDZ3dNU1c1MFpYSnRaV1JwWVhSbE1CNFhEVEl6TURFd05USXhNekV6TkZvWERUTXpNREV3TVRJeE16RXpORm93UFRFTE1Ba0dBMVVFQmhNQ1ZWTXhDekFKQmdOVkJBZ01Ba05CTVJJd0VBWURWUVFIREFsRGRYQmxjblJwYm04eERUQUxCZ05WQkFvTUJFeGxZV1l3V1RBVEJnY3Foa2pPUFFJQkJnZ3Foa2pPUFFNQkJ3TkNBQVRpdFlIRWFZVnVjOGc5QWpUT3dFck12R3lQeWtQYStwdXZUSThoSlRIWlpETEdhczJxWDErRXJ4Z1FUSmdWWHY3Nm5tTGhoUkpIK2oyNUFpQUk4aUdzb3k4d0xUQUpCZ05WSFJNRUFqQUFNQTRHQTFVZER3RUIvd1FFQXdJSGdEQVFCZ29xaGtpRzkyTmtCZ3NCQkFJRkFEQUtCZ2dxaGtqT1BRUURBd05JQURCRkFpQlg0YytUMEZwNW5KNVFSQ2xSZnU1UFNCeVJ2TlB0dWFUc2swdlBCM1dBSUFJaEFOZ2FhdUFqL1lQOXMwQWtFaHlKaHhRTy82UTJ6b3VaK0gxQ0lPZWhuTXpRIiwiTUlJQm56Q0NBVVdnQXdJQkFnSUJDekFLQmdncWhrak9QUVFEQXpBMk1Rc3dDUVlEVlFRR0V3SlZVekVUTUJFR0ExVUVDQXdLUTJGc2FXWnZjbTVwWVRFU01CQUdBMVVFQnd3SlEzVndaWEowYVc1dk1CNFhEVEl6TURFd05USXhNekV3TlZvWERUTXpNREV3TVRJeE16RXdOVm93UlRFTE1Ba0dBMVVFQmhNQ1ZWTXhDekFKQmdOVkJBZ01Ba05CTVJJd0VBWURWUVFIREFsRGRYQmxjblJwYm04eEZUQVRCZ05WQkFvTURFbHVkR1Z5YldWa2FXRjBaVEJaTUJNR0J5cUdTTTQ5QWdFR0NDcUdTTTQ5QXdFSEEwSUFCQlVONVY5cktqZlJpTUFJb2pFQTBBdjVNcDBvRitPMGNMNGd6clRGMTc4aW5VSHVnajdFdDQ2TnJrUTdoS2dNVm5qb2dxNDVRMXJNcytjTUhWTklMV3FqTlRBek1BOEdBMVVkRXdRSU1BWUJBZjhDQVFBd0RnWURWUjBQQVFIL0JBUURBZ0VHTUJBR0NpcUdTSWIzWTJRR0FnRUVBZ1VBTUFvR0NDcUdTTTQ5QkFNREEwZ0FNRVVDSVFDbXNJS1lzNDF1bGxzc0hYNHJWdmVVVDBaN0lzNS9oTEsxbEZQVHR1bjNoQUlnYzIrMlJHNStnTmNGVmNzK1hKZUVsNEdaK29qbDNST09tbGwreWU3ZHluUT0iLCJNSUlCZ2pDQ0FTbWdBd0lCQWdJSkFMVWM1QUxpSDVwYk1Bb0dDQ3FHU000OUJBTURNRFl4Q3pBSkJnTlZCQVlUQWxWVE1STXdFUVlEVlFRSURBcERZV3hwWm05eWJtbGhNUkl3RUFZRFZRUUhEQWxEZFhCbGNuUnBibTh3SGhjTk1qTXdNVEExTWpFek1ESXlXaGNOTXpNd01UQXlNakV6TURJeVdqQTJNUXN3Q1FZRFZRUUdFd0pWVXpFVE1CRUdBMVVFQ0F3S1EyRnNhV1p2Y201cFlURVNNQkFHQTFVRUJ3d0pRM1Z3WlhKMGFXNXZNRmt3RXdZSEtvWkl6ajBDQVFZSUtvWkl6ajBEQVFjRFFnQUVjKy9CbCtnb3NwbzZ0ZjlaN2lvNXRkS2RybE4xWWRWbnFFaEVEWERTaHpkQUpQUWlqYW1YSU1IZjh4V1dUYTF6Z29ZVHhPS3BidUp0RHBsejFYcmlUYU1nTUI0d0RBWURWUjBUQkFVd0F3RUIvekFPQmdOVkhROEJBZjhFQkFNQ0FRWXdDZ1lJS29aSXpqMEVBd01EUndBd1JBSWdlbVdRWG5NQWRUYWQySkRKV25nOVU0dUJCTDVtQTdXSTA1SDdvSDdjNmlRQ0lIaVJxTWpOZnpVQXlpdTloNnJPVS9LK2lUUjBJLzNZL05TV3NYSFgrYWNjIl19.eyJkYXRhIjp7ImJ1bmRsZUlkIjoiY29tLmV4YW1wbGUifSwibm90aWZpY2F0aW9uVVVJRCI6IjlhZDU2YmQyLTBiYzYtNDJlMC1hZjI0LWZkOTk2ZDg3YTFlNiIsIm5vdGlmaWNhdGlvblR5cGUiOiJURVNUIn0.1TFhjDR4WwQJNgizVGYXz3WE3ajxTdH1wKLQQ71MtrkadSxxOo3yPo_6L9Z03unIU7YK-NRNzSIb5bh5WqTprQ";
    const WRONG_BUNDLE_ID: &str = "eyJ4NWMiOlsiTUlJQm9EQ0NBVWFnQXdJQkFnSUJEREFLQmdncWhrak9QUVFEQXpCRk1Rc3dDUVlEVlFRR0V3SlZVekVMTUFrR0ExVUVDQXdDUTBFeEVqQVFCZ05WQkFjTUNVTjFjR1Z5ZEdsdWJ6RVZNQk1HQTFVRUNnd01TVzUwWlhKdFpXUnBZWFJsTUI0WERUSXpNREV3TlRJeE16RXpORm9YRFRNek1ERXdNVEl4TXpFek5Gb3dQVEVMTUFrR0ExVUVCaE1DVlZNeEN6QUpCZ05WQkFnTUFrTkJNUkl3RUFZRFZRUUhEQWxEZFhCbGNuUnBibTh4RFRBTEJnTlZCQW9NQkV4bFlXWXdXVEFUQmdjcWhrak9QUUlCQmdncWhrak9QUU1CQndOQ0FBVGl0WUhFYVlWdWM4ZzlBalRPd0VyTXZHeVB5a1BhK3B1dlRJOGhKVEhaWkRMR2FzMnFYMStFcnhnUVRKZ1ZYdjc2bm1MaGhSSkgrajI1QWlBSThpR3NveTh3TFRBSkJnTlZIUk1FQWpBQU1BNEdBMVVkRHdFQi93UUVBd0lIZ0RBUUJnb3Foa2lHOTJOa0Jnc0JCQUlGQURBS0JnZ3Foa2pPUFFRREF3TklBREJGQWlCWDRjK1QwRnA1bko1UVJDbFJmdTVQU0J5UnZOUHR1YVRzazB2UEIzV0FJQUloQU5nYWF1QWovWVA5czBBa0VoeUpoeFFPLzZRMnpvdVorSDFDSU9laG5NelEiLCJNSUlCbnpDQ0FVV2dBd0lCQWdJQkN6QUtCZ2dxaGtqT1BRUURBekEyTVFzd0NRWURWUVFHRXdKVlV6RVRNQkVHQTFVRUNBd0tRMkZzYVdadmNtNXBZVEVTTUJBR0ExVUVCd3dKUTNWd1pYSjBhVzV2TUI0WERUSXpNREV3TlRJeE16RXdOVm9YRFRNek1ERXdNVEl4TXpFd05Wb3dSVEVMTUFrR0ExVUVCaE1DVlZNeEN6QUpCZ05WQkFnTUFrTkJNUkl3RUFZRFZRUUhEQWxEZFhCbGNuUnBibTh4RlRBVEJnTlZCQW9NREVsdWRHVnliV1ZrYVdGMFpUQlpNQk1HQnlxR1NNNDlBZ0VHQ0NxR1NNNDlBd0VIQTBJQUJCVU41VjlyS2pmUmlNQUlvakVBMEF2NU1wMG9GK08wY0w0Z3pyVEYxNzhpblVIdWdqN0V0NDZOcmtRN2hLZ01WbmpvZ3E0NVExck1zK2NNSFZOSUxXcWpOVEF6TUE4R0ExVWRFd1FJTUFZQkFmOENBUUF3RGdZRFZSMFBBUUgvQkFRREFnRUdNQkFHQ2lxR1NJYjNZMlFHQWdFRUFnVUFNQW9HQ0NxR1NNNDlCQU1EQTBnQU1FVUNJUUNtc0lLWXM0MXVsbHNzSFg0clZ2ZVVUMFo3SXM1L2hMSzFsRlBUdHVuM2hBSWdjMisyUkc1K2dOY0ZWY3MrWEplRWw0R1orb2psM1JPT21sbCt5ZTdkeW5RPSIsIk1JSUJnakNDQVNtZ0F3SUJBZ0lKQUxVYzVBTGlINXBiTUFvR0NDcUdTTTQ5QkFNRE1EWXhDekFKQmdOVkJBWVRBbFZUTVJNd0VRWURWUVFJREFwRFlXeHBabTl5Ym1saE1SSXdFQVlEVlFRSERBbERkWEJsY25ScGJtOHdIaGNOTWpNd01UQTFNakV6TURJeVdoY05Nek13TVRBeU1qRXpNREl5V2pBMk1Rc3dDUVlEVlFRR0V3SlZVekVUTUJFR0ExVUVDQXdLUTJGc2FXWnZjbTVwWVRFU01CQUdBMVVFQnd3SlEzVndaWEowYVc1dk1Ga3dFd1lIS29aSXpqMENBUVlJS29aSXpqMERBUWNEUWdBRWMrL0JsK2dvc3BvNnRmOVo3aW81dGRLZHJsTjFZZFZucUVoRURYRFNoemRBSlBRaWphbVhJTUhmOHhXV1RhMXpnb1lUeE9LcGJ1SnREcGx6MVhyaVRhTWdNQjR3REFZRFZSMFRCQVV3QXdFQi96QU9CZ05WSFE4QkFmOEVCQU1DQVFZd0NnWUlLb1pJemowRUF3TURSd0F3UkFJZ2VtV1FYbk1BZFRhZDJKREpXbmc5VTR1QkJMNW1BN1dJMDVIN29IN2M2aVFDSUhpUnFNak5melVBeWl1OWg2ck9VL0sraVRSMEkvM1kvTlNXc1hIWCthY2MiXSwidHlwIjoiSldUIiwiYWxnIjoiRVMyNTYifQ.eyJkYXRhIjp7ImJ1bmRsZUlkIjoiY29tLmV4YW1wbGUud3JvbmcifSwibm90aWZpY2F0aW9uVVVJRCI6IjlhZDU2YmQyLTBiYzYtNDJlMC1hZjI0LWZkOTk2ZDg3YTFlNiIsIm5vdGlmaWNhdGlvblR5cGUiOiJURVNUIn0.WWE31hTB_mcv2O_lf-xI-MNY3d8txc0MzpqFx4QnYDfFIxB95Lo2Fm3r46YSjLLdL7xCWdEJrJP5bHgRCejAGg";
    const RENEWAL_INFO: &str = "eyJ4NWMiOlsiTUlJQm9EQ0NBVWFnQXdJQkFnSUJEREFLQmdncWhrak9QUVFEQXpCRk1Rc3dDUVlEVlFRR0V3SlZVekVMTUFrR0ExVUVDQXdDUTBFeEVqQVFCZ05WQkFjTUNVTjFjR1Z5ZEdsdWJ6RVZNQk1HQTFVRUNnd01TVzUwWlhKdFpXUnBZWFJsTUI0WERUSXpNREV3TlRJeE16RXpORm9YRFRNek1ERXdNVEl4TXpFek5Gb3dQVEVMTUFrR0ExVUVCaE1DVlZNeEN6QUpCZ05WQkFnTUFrTkJNUkl3RUFZRFZRUUhEQWxEZFhCbGNuUnBibTh4RFRBTEJnTlZCQW9NQkV4bFlXWXdXVEFUQmdjcWhrak9QUUlCQmdncWhrak9QUU1CQndOQ0FBVGl0WUhFYVlWdWM4ZzlBalRPd0VyTXZHeVB5a1BhK3B1dlRJOGhKVEhaWkRMR2FzMnFYMStFcnhnUVRKZ1ZYdjc2bm1MaGhSSkgrajI1QWlBSThpR3NveTh3TFRBSkJnTlZIUk1FQWpBQU1BNEdBMVVkRHdFQi93UUVBd0lIZ0RBUUJnb3Foa2lHOTJOa0Jnc0JCQUlGQURBS0JnZ3Foa2pPUFFRREF3TklBREJGQWlCWDRjK1QwRnA1bko1UVJDbFJmdTVQU0J5UnZOUHR1YVRzazB2UEIzV0FJQUloQU5nYWF1QWovWVA5czBBa0VoeUpoeFFPLzZRMnpvdVorSDFDSU9laG5NelEiLCJNSUlCbnpDQ0FVV2dBd0lCQWdJQkN6QUtCZ2dxaGtqT1BRUURBekEyTVFzd0NRWURWUVFHRXdKVlV6RVRNQkVHQTFVRUNBd0tRMkZzYVdadmNtNXBZVEVTTUJBR0ExVUVCd3dKUTNWd1pYSjBhVzV2TUI0WERUSXpNREV3TlRJeE16RXdOVm9YRFRNek1ERXdNVEl4TXpFd05Wb3dSVEVMTUFrR0ExVUVCaE1DVlZNeEN6QUpCZ05WQkFnTUFrTkJNUkl3RUFZRFZRUUhEQWxEZFhCbGNuUnBibTh4RlRBVEJnTlZCQW9NREVsdWRHVnliV1ZrYVdGMFpUQlpNQk1HQnlxR1NNNDlBZ0VHQ0NxR1NNNDlBd0VIQTBJQUJCVU41VjlyS2pmUmlNQUlvakVBMEF2NU1wMG9GK08wY0w0Z3pyVEYxNzhpblVIdWdqN0V0NDZOcmtRN2hLZ01WbmpvZ3E0NVExck1zK2NNSFZOSUxXcWpOVEF6TUE4R0ExVWRFd1FJTUFZQkFmOENBUUF3RGdZRFZSMFBBUUgvQkFRREFnRUdNQkFHQ2lxR1NJYjNZMlFHQWdFRUFnVUFNQW9HQ0NxR1NNNDlCQU1EQTBnQU1FVUNJUUNtc0lLWXM0MXVsbHNzSFg0clZ2ZVVUMFo3SXM1L2hMSzFsRlBUdHVuM2hBSWdjMisyUkc1K2dOY0ZWY3MrWEplRWw0R1orb2psM1JPT21sbCt5ZTdkeW5RPSIsIk1JSUJnakNDQVNtZ0F3SUJBZ0lKQUxVYzVBTGlINXBiTUFvR0NDcUdTTTQ5QkFNRE1EWXhDekFKQmdOVkJBWVRBbFZUTVJNd0VRWURWUVFJREFwRFlXeHBabTl5Ym1saE1SSXdFQVlEVlFRSERBbERkWEJsY25ScGJtOHdIaGNOTWpNd01UQTFNakV6TURJeVdoY05Nek13TVRBeU1qRXpNREl5V2pBMk1Rc3dDUVlEVlFRR0V3SlZVekVUTUJFR0ExVUVDQXdLUTJGc2FXWnZjbTVwWVRFU01CQUdBMVVFQnd3SlEzVndaWEowYVc1dk1Ga3dFd1lIS29aSXpqMENBUVlJS29aSXpqMERBUWNEUWdBRWMrL0JsK2dvc3BvNnRmOVo3aW81dGRLZHJsTjFZZFZucUVoRURYRFNoemRBSlBRaWphbVhJTUhmOHhXV1RhMXpnb1lUeE9LcGJ1SnREcGx6MVhyaVRhTWdNQjR3REFZRFZSMFRCQVV3QXdFQi96QU9CZ05WSFE4QkFmOEVCQU1DQVFZd0NnWUlLb1pJemowRUF3TURSd0F3UkFJZ2VtV1FYbk1BZFRhZDJKREpXbmc5VTR1QkJMNW1BN1dJMDVIN29IN2M2aVFDSUhpUnFNak5melVBeWl1OWg2ck9VL0sraVRSMEkvM1kvTlNXc1hIWCthY2MiXSwidHlwIjoiSldUIiwiYWxnIjoiRVMyNTYifQ.eyJlbnZpcm9ubWVudCI6IlNhbmRib3giLCJzaWduZWREYXRlIjoxNjcyOTU2MTU0MDAwfQ.FbK2OL-t6l4892W7fzWyus_g9mIl2CzWLbVt7Kgcnt6zzVulF8bzovgpe0v_y490blROGixy8KDoe2dSU53-Xw";
    const TRANSACTION_INFO: &str = "eyJ4NWMiOlsiTUlJQm9EQ0NBVWFnQXdJQkFnSUJDekFLQmdncWhrak9QUVFEQWpCTk1Rc3dDUVlEVlFRR0V3SlZVekVUTUJFR0ExVUVDQXdLUTJGc2FXWnZjbTVwWVRFU01CQUdBMVVFQnd3SlEzVndaWEowYVc1dk1SVXdFd1lEVlFRS0RBeEpiblJsY20xbFpHbGhkR1V3SGhjTk1qTXdNVEEwTVRZek56TXhXaGNOTXpJeE1qTXhNVFl6TnpNeFdqQkZNUXN3Q1FZRFZRUUdFd0pWVXpFVE1CRUdBMVVFQ0F3S1EyRnNhV1p2Y201cFlURVNNQkFHQTFVRUJ3d0pRM1Z3WlhKMGFXNXZNUTB3Q3dZRFZRUUtEQVJNWldGbU1Ga3dFd1lIS29aSXpqMENBUVlJS29aSXpqMERBUWNEUWdBRTRyV0J4R21GYm5QSVBRSTB6c0JLekx4c2o4cEQydnFicjB5UElTVXgyV1F5eG1yTnFsOWZoSzhZRUV5WUZWNysrcDVpNFlVU1Ivbzl1UUlnQ1BJaHJLTWZNQjB3Q1FZRFZSMFRCQUl3QURBUUJnb3Foa2lHOTJOa0Jnc0JCQUlUQURBS0JnZ3Foa2pPUFFRREFnTklBREJGQWlFQWtpRVprb0ZNa2o0Z1huK1E5alhRWk1qWjJnbmpaM2FNOE5ZcmdmVFVpdlFDSURKWVowRmFMZTduU0lVMkxXTFRrNXRYVENjNEU4R0pTWWYvc1lSeEVGaWUiLCJNSUlCbHpDQ0FUMmdBd0lCQWdJQkJqQUtCZ2dxaGtqT1BRUURBakEyTVFzd0NRWURWUVFHRXdKVlV6RVRNQkVHQTFVRUNBd0tRMkZzYVdadmNtNXBZVEVTTUJBR0ExVUVCd3dKUTNWd1pYSjBhVzV2TUI0WERUSXpNREV3TkRFMk1qWXdNVm9YRFRNeU1USXpNVEUyTWpZd01Wb3dUVEVMTUFrR0ExVUVCaE1DVlZNeEV6QVJCZ05WQkFnTUNrTmhiR2xtYjNKdWFXRXhFakFRQmdOVkJBY01DVU4xY0dWeWRHbHViekVWTUJNR0ExVUVDZ3dNU1c1MFpYSnRaV1JwWVhSbE1Ga3dFd1lIS29aSXpqMENBUVlJS29aSXpqMERBUWNEUWdBRUZRM2xYMnNxTjlHSXdBaWlNUURRQy9reW5TZ1g0N1J3dmlET3RNWFh2eUtkUWU2Q1BzUzNqbzJ1UkR1RXFBeFdlT2lDcmpsRFdzeXo1d3dkVTBndGFxTWxNQ013RHdZRFZSMFRCQWd3QmdFQi93SUJBREFRQmdvcWhraUc5Mk5rQmdJQkJBSVRBREFLQmdncWhrak9QUVFEQWdOSUFEQkZBaUVBdm56TWNWMjY4Y1JiMS9GcHlWMUVoVDNXRnZPenJCVVdQNi9Ub1RoRmF2TUNJRmJhNXQ2WUt5MFIySkR0eHF0T2pKeTY2bDZWN2QvUHJBRE5wa21JUFcraSIsIk1JSUJYRENDQVFJQ0NRQ2ZqVFVHTERuUjlqQUtCZ2dxaGtqT1BRUURBekEyTVFzd0NRWURWUVFHRXdKVlV6RVRNQkVHQTFVRUNBd0tRMkZzYVdadmNtNXBZVEVTTUJBR0ExVUVCd3dKUTNWd1pYSjBhVzV2TUI0WERUSXpNREV3TkRFMk1qQXpNbG9YRFRNek1ERXdNVEUyTWpBek1sb3dOakVMTUFrR0ExVUVCaE1DVlZNeEV6QVJCZ05WQkFnTUNrTmhiR2xtYjNKdWFXRXhFakFRQmdOVkJBY01DVU4xY0dWeWRHbHViekJaTUJNR0J5cUdTTTQ5QWdFR0NDcUdTTTQ5QXdFSEEwSUFCSFB2d1pmb0tMS2FPclgvV2U0cU9iWFNuYTVUZFdIVlo2aElSQTF3MG9jM1FDVDBJbzJwbHlEQjMvTVZsazJ0YzRLR0U4VGlxVzdpYlE2WmM5VjY0azB3Q2dZSUtvWkl6ajBFQXdNRFNBQXdSUUloQU1USGhXdGJBUU4waFN4SVhjUDRDS3JEQ0gvZ3N4V3B4NmpUWkxUZVorRlBBaUIzNW53azVxMHpjSXBlZnZZSjBNVS95R0dIU1dlejBicTBwRFlVTy9ubUR3PT0iXSwidHlwIjoiSldUIiwiYWxnIjoiRVMyNTYifQ.eyJlbnZpcm9ubWVudCI6IlNhbmRib3giLCJidW5kbGVJZCI6ImNvbS5leGFtcGxlIiwic2lnbmVkRGF0ZSI6MTY3Mjk1NjE1NDAwMH0.PnHWpeIJZ8f2Q218NSGLo_aR0IBEJvC6PxmxKXh-qfYTrZccx2suGl223OSNAX78e4Ylf2yJCG2N-FfU-NIhZQ";

    #[test]
    fn test_app_store_server_notification_decoding() {
        let verifier = get_payload_verifier();
        let notification = verifier.verify_and_decode_notification(TEST_NOTIFICATION).unwrap();
        assert_eq!(notification.notification_type, NotificationTypeV2::Test);
    }

    #[test]
    fn test_missing_x5c_header() {
        let verifier = get_payload_verifier();
        let result = verifier.verify_and_decode_notification(MISSING_X5C_HEADER_CLAIM);
        assert_eq!(result.err().unwrap(), SignedDataVerifierError::VerificationFailure);
    }

    #[test]
    fn test_wrong_bundle_id_for_server_notification() {
        let verifier = get_payload_verifier();
        let result = verifier.verify_and_decode_notification(WRONG_BUNDLE_ID);
        assert_eq!(result.err().unwrap(), SignedDataVerifierError::InvalidAppIdentifier);
    }

    #[test]
    fn test_wrong_app_apple_id_for_server_notification() {
        let verifier = SignedDataVerifier::new(
            vec![ROOT_CA_BASE64_ENCODED.as_der_bytes().unwrap()],
            Environment::Production,
            "com.example".to_string(),
            Some(1235),
        );
        let result = verifier.verify_and_decode_notification(TEST_NOTIFICATION);
        assert_eq!(result.err().unwrap(), SignedDataVerifierError::InvalidAppIdentifier);
    }

    // #[test]
    // fn test_renewal_info_decoding() {
    //     let verifier = get_payload_verifier();
    //     let notification = verifier.verify_and_decode_renewal_info(RENEWAL_INFO);
    //     assert_eq!(notification.environment, Environment::Sandbox);
    // }

    #[test]
    fn test_transaction_info_decoding() {
        let verifier = get_payload_verifier();
        let notification = verifier.verify_and_decode_signed_transaction(TRANSACTION_INFO).unwrap();
        assert_eq!(notification.environment, Some(Environment::Sandbox));
    }

    #[test]
    fn test_malformed_jwt_with_too_many_parts() {
        let verifier = get_payload_verifier();
        let result = verifier.verify_and_decode_notification("a.b.c.d");
        assert!(result.err().unwrap().to_string().contains("InternalJWTError"));
    }

    #[test]
    fn test_malformed_jwt_with_malformed_data() {
        let verifier = get_payload_verifier();
        let result = verifier.verify_and_decode_notification("a.b.c");
        assert!(result.err().unwrap().to_string().contains("InternalJWTError"));
    }

    fn get_payload_verifier() -> SignedDataVerifier {
        let mut verifier = SignedDataVerifier::new(
            vec![ROOT_CA_BASE64_ENCODED.as_der_bytes().unwrap()],
            Environment::Sandbox,
            "com.example".to_string(),
            None);

        verifier
    }
}