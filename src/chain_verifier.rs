use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use thiserror::Error;
use crate::chain_verifier::ChainVerificationFailureReason::{CertificateExpired, InvalidCertificate, InvalidChainLength, InvalidEffectiveDate};

#[cfg(feature = "x509-parser")]
use {
    x509_parser::certificate::X509Certificate,
    x509_parser::prelude::{FromDer, ASN1Time},
    x509_parser::der_parser::asn1_rs::oid,
    x509_parser::error::X509Error,
};

#[cfg(feature = "openssl")]
use {
    openssl::stack::Stack,
    openssl::x509::store::X509StoreBuilder,
    openssl::x509::verify::X509VerifyParam,
    openssl::x509::{X509, X509StoreContext},
};

#[derive(Error, Debug)]
pub enum ChainVerifierError {
    #[error("VerificationFailure: [{0}]")]
    VerificationFailure(ChainVerificationFailureReason),

    #[cfg(feature = "x509-parser")]
    #[error("InternalX509Error: [{0}]")]
    InternalX509Error(#[from] X509Error)
}

#[derive(Error, Debug)]
pub enum ChainVerificationFailureReason {
    #[error("InvalidAppIdentifier")]
    InvalidAppIdentifier,

    #[error("InvalidIssuer")]
    InvalidIssuer,

    #[error("InvalidCertificate")]
    InvalidCertificate,

    #[error("InvalidChainLength")]
    InvalidChainLength,

    #[error("InvalidChain")]
    InvalidChain,

    #[error("InvalidEnvironment")]
    InvalidEffectiveDate,

    #[error("CertificateExpired")]
    CertificateExpired
}

const EXPECTED_CHAIN_LENGTH: usize = 3;

#[derive(Clone, Debug)]
pub struct ChainVerifier {
    root_certificates: Vec<Vec<u8>>,
}

impl ChainVerifier {
    pub fn new(enable_strict_checks: bool, root_certificates: Vec<Vec<u8>>) -> Self {
        ChainVerifier {
            root_certificates
        }
    }
}

#[cfg(feature = "x509-parser")]
impl ChainVerifier {

    pub fn verify(&self, certificates: &Vec<Vec<u8>>, effective_date: Option<u64>) -> Result<Vec<u8>, ChainVerifierError> {
        if self.root_certificates.is_empty() {
            return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
        }

        if certificates.len() != EXPECTED_CHAIN_LENGTH {
            return Err(ChainVerifierError::VerificationFailure(InvalidChainLength));
        }

        let leaf_certificate = &certificates[0];
        let Ok(leaf_certificate) = X509Certificate::from_der(leaf_certificate.as_slice()) else {
            return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
        };
        let leaf_certificate = leaf_certificate.1;

        let Some(_) = leaf_certificate.get_extension_unique(&oid!(1.2.840.113635.100.6.11.1))? else {
            return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
        };

        let intermediate_certificate = &certificates[1];
        let Ok(intermediate_certificate) = X509Certificate::from_der(intermediate_certificate.as_slice()) else {
            return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
        };
        let intermediate_certificate = intermediate_certificate.1;

        let Some(_) = intermediate_certificate.get_extension_unique(&oid!(1.2.840.113635.100.6.2.1))? else {
            return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
        };

        let mut root_certificate: Option<X509Certificate> = None;

        for cert in &self.root_certificates {
            let Ok(cert) = X509Certificate::from_der(&cert) else {
                return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
            };

            let cert = cert.1;

            if !cert.validity.is_valid() {
                return Err(ChainVerifierError::VerificationFailure(InvalidCertificate))
            }

            // TODO: Implement issuer checking
            // if intermediate_certificate.issuer != cert.issuer {
            //     return Err(ChainVerifierError::VerificationFailure(InvalidIssuer))
            // }

            match intermediate_certificate.verify_signature(Some(cert.public_key())) {
                Ok(_) => (),
                Err(e) => return Err(ChainVerifierError::InternalX509Error(e))
            }

            root_certificate = Some(cert)
        }

        let Some(root_certificate) = root_certificate else {
            return Err(ChainVerifierError::VerificationFailure(InvalidCertificate))
        };

        leaf_certificate.verify_signature(Some(intermediate_certificate.public_key()))?;

        // TODO: Properly validate issuer
        // if leaf_certificate.issuer != intermediate_certificate.issuer {
        //     return Err(ChainVerifierError::VerificationFailure(InvalidIssuer))
        // }

        if let Some(date) = effective_date {
            let Ok(time) = ASN1Time::from_timestamp(i64::try_from(date).unwrap()) else {
                return Err(ChainVerifierError::VerificationFailure(InvalidEffectiveDate))
            };

            if !(root_certificate.validity.is_valid_at(time) &&
                leaf_certificate.validity.is_valid_at(time) &&
                intermediate_certificate.validity.is_valid_at(time)) {
                return Err(ChainVerifierError::VerificationFailure(CertificateExpired))
            }
        }

        let k = leaf_certificate.public_key().subject_public_key.data.to_vec();
        Ok(k)
    }
}

#[cfg(feature = "openssl")]
impl ChainVerifier {
    pub fn verify(&self, certificates: &Vec<Vec<u8>>, effective_date: Option<u64>) -> Result<Vec<u8>, ChainVerifierError> {
        if self.root_certificates.is_empty() {
            return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
        }

        if certificates.len() != EXPECTED_CHAIN_LENGTH {
            return Err(ChainVerifierError::VerificationFailure(InvalidChainLength));
        }

        let mut trusted_store_builder = X509StoreBuilder::new().unwrap();

        for trusted_cert_bytes in &self.root_certificates {
            let trusted_cert = X509::from_der(&trusted_cert_bytes).expect("Expect x509");
            trusted_store_builder.add_cert(trusted_cert).unwrap();
        }

        trusted_store_builder.set_flags(openssl::x509::verify::X509VerifyFlags::X509_STRICT).unwrap();

        if let Some(effective_date) = effective_date {
            let mut param = X509VerifyParam::new().unwrap();
            param.set_time(effective_date.try_into().unwrap());

            trusted_store_builder.set_param(&param).unwrap();
        }

        let trusted_store = trusted_store_builder.build();

        let leaf_cert = X509::from_der(&certificates[0]).expect("Expect x509");
        let intermediate_cert = X509::from_der(&certificates[1]).expect("Expect x509");

        let mut cert_stack = Stack::new().unwrap();
        cert_stack.push(intermediate_cert).unwrap();

        let mut ctx = X509StoreContext::new().unwrap();
        ctx.init(&trusted_store, &leaf_cert, &cert_stack, |c| c.verify_cert()).unwrap();

        let public_key: Vec<u8> = leaf_cert.public_key()
            .expect("Expect pubkey")
            .public_key_to_der()
            .expect("Expect pem");

        // Take the last 65 bytes
        let public_key = &public_key[public_key.len() - 65..];

        Ok(public_key.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use base64::Engine;
    use base64::engine::general_purpose::STANDARD;
    use jsonwebtoken::{Algorithm, DecodingKey, Validation};
    use std::time::{SystemTime, UNIX_EPOCH};
    use crate::primitives::response_body_v2_decoded_payload::ResponseBodyV2DecodedPayload;
    use super::*;

    pub fn signed_payload() -> String {
        std::env::var("SIGNED_PAYLOAD").expect("SIGNED_PAYLOAD must be set")
    }

    pub fn apple_root_cert() -> String {
        std::env::var("APPLE_ROOT_BASE64_ENCODED").expect("APPLE_ROOT_BASE64_ENCODED must be set")
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
        let x5c = x5c.iter().map(|c| STANDARD.decode(c).expect("Expect bytes")).collect();

        let root_cert = apple_root_cert();
        let root_cert_der = STANDARD.decode(root_cert).expect("Expect bytes");

        let verifier = ChainVerifier {
            root_certificates: vec![root_cert_der],
        };

        let effective_date =  SystemTime::now();
        let since_the_epoch = effective_date
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");  // Define the effective date

        let pub_key = verifier.verify(&x5c,  Some(since_the_epoch.as_secs())).expect("Expect pub key");

        let decoding_key = DecodingKey::from_ec_der(pub_key.as_slice());
        let claims: [&str; 0] = [];

        let mut validator = Validation::new(Algorithm::ES256);
        validator.validate_exp = false;
        validator.set_required_spec_claims(&claims);

        let payload = jsonwebtoken::decode::<ResponseBodyV2DecodedPayload>(token, &decoding_key, &validator).expect("Expect Payload");
        println!("{:?}", payload.claims);

    }
}