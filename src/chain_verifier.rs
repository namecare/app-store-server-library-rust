use openssl::x509::{X509, X509StoreContext};
use std::time::{SystemTime, UNIX_EPOCH};
use openssl::stack::{Stack};
use openssl::x509::store::{X509StoreBuilder};
use openssl::x509::verify::X509VerifyParam;

use base64::Engine;
use base64::engine::general_purpose::STANDARD;

// Define your VerificationStatus enum
#[derive(Debug)]
enum VerificationStatus {
    OK,
    VerificationFailure,
    InvalidAppIdentifier,
    InvalidCertificate,
    InvalidChainLength,
    InvalidChain,
    InvalidEnvironment,
}

struct ChainVerifier {
    enable_strict_checks: bool,
    root_certificates: Vec<Vec<u8>>,
}

impl ChainVerifier {
    fn verify_chain(&self, certificates: Vec<String>, effective_date: u64) -> Result<Vec<u8>, VerificationStatus> {
        if self.root_certificates.is_empty() {
            return Err(VerificationStatus::InvalidCertificate);
        }
        if certificates.len() != 3 {
            return Err(VerificationStatus::InvalidChainLength);
        }

        let mut trusted_store_builder = X509StoreBuilder::new().unwrap();

        for trusted_cert_bytes in &self.root_certificates {
            let trusted_cert = X509::from_der(&trusted_cert_bytes).expect("Expect x509");
            trusted_store_builder.add_cert(trusted_cert).expect("Expect added");
        }

        if self.enable_strict_checks {
            trusted_store_builder.set_flags(openssl::x509::verify::X509VerifyFlags::X509_STRICT).expect("Expect added");
        }

        let mut param = X509VerifyParam::new().expect("Expect param");
        param.set_time(effective_date.try_into().unwrap());

        trusted_store_builder.set_param(&param).unwrap();

        let trusted_store = trusted_store_builder.build();

        let leaf_cert_bytes = STANDARD.decode(certificates[0].as_bytes()).expect("Expect bytes");
        let leaf_cert = X509::from_der(&leaf_cert_bytes).expect("Expect x509");

        let intermediate_cert_bytes = STANDARD.decode(certificates[1].as_bytes()).expect("Expect bytes");
        let intermediate_cert = X509::from_der(&intermediate_cert_bytes).expect("Expect x509");

        let mut cert_stack = Stack::new().unwrap();
        cert_stack.push(intermediate_cert).unwrap();

        let mut ctx = X509StoreContext::new().unwrap();

        ctx.init(&trusted_store, &leaf_cert, &cert_stack, |c| c.verify_cert()).expect("Expect init");

        let public_key = leaf_cert.public_key()
            .expect("Expect pubkey")
            .public_key_to_pem()
            .expect("Expect pem");

        Ok(public_key)
    }
}


#[cfg(test)]
mod tests {
    use base64::Engine;
    use base64::engine::general_purpose::STANDARD;
    use jsonwebtoken::{Algorithm, DecodingKey, Validation};
    use crate::primitives::response_body_v2_decoded_payload::ResponseBodyV2DecodedPayload;
    use super::*;

    pub fn signed_payload() -> String {
        std::env::var("SIGNED_PAYLOAD").expect("SIGNED_PAYLOAD must be set")
    }

    pub fn apple_root_cert() -> String {
        std::env::var("REAL_APPLE_ROOT_BASE64_ENCODED").expect("REAL_APPLE_ROOT_BASE64_ENCODED must be set")
    }

    #[test]
    fn text_chain_verification() {
        dotenv::dotenv().ok();

        let payload = signed_payload();
        let token = payload.as_str();
        let header = jsonwebtoken::decode_header(token).expect("Expect header");

        let Some(x5c) = header.x5c else {
            return;
        };

        let root_cert = apple_root_cert();
        let root_cert_der = STANDARD.decode(root_cert).expect("Expect bytes");

        let verifier = ChainVerifier {
            enable_strict_checks: false,
            root_certificates: vec![root_cert_der],
        };

        let effective_date =  SystemTime::now();
        let since_the_epoch = effective_date
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");  // Define the effective date

        let pub_key = verifier.verify_chain(x5c,  since_the_epoch.as_secs()).expect("Expect pub key");

        let decoding_key = DecodingKey::from_ec_pem(pub_key.as_slice()).unwrap();

        let claims: [&str; 0] = [];

        let mut validator = Validation::new(Algorithm::ES256);
        validator.validate_exp = false;
        validator.set_required_spec_claims(&claims);

        let payload = jsonwebtoken::decode::<ResponseBodyV2DecodedPayload>(token, &decoding_key, &validator).expect("Expect Payload");
        println!("{:?}", payload.claims);

    }
}