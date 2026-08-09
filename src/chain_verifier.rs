use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

use x509_validator::{rfc5280::RFC5280Policy, store::CertificateStore, validator::ChainValidationResultOwned, Certificate, CertificateExt, Oid, PolicyEvaluationResult, PolicyFailureReason, ValidationPolicy};
use x509_validator::unverified_chain::UnverifiedCertificateChain;

#[derive(Error, Debug, PartialEq)]
pub enum ChainVerifierError {
    #[error("VerificationFailure: [{0}]")]
    VerificationFailure(ChainVerificationFailureReason),

    #[error("InternalError: [{0}]")]
    InternalError(String),

    #[error("InternalDecodeError: [{0}]")]
    InternalDecodeError(#[from] base64::DecodeError),
}

#[derive(Error, Debug, PartialEq)]
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
    CertificateExpired,

    #[error("CertificateRevoked")]
    CertificateRevoked,

    #[error("RetryableVerificationFailure")]
    RetryableVerificationFailure,
}

/// Apple's receipt-signing OID, expected on the leaf certificate.
const APPLE_RECEIPT_SIGNER_OID: &str = "1.2.840.113635.100.6.11.1";
/// Apple's WWDR OID, expected on the intermediate certificate.
const APPLE_WWDR_INTERMEDIATE_OID: &str = "1.2.840.113635.100.6.2.1";
/// leaf, intermediate, root.
const EXPECTED_CHAIN_LENGTH: usize = 3;

struct AppStoreOidPolicy {
    wwdr_oid: Oid<'static>,
    receipt_signer_oid: Oid<'static>,
}

impl AppStoreOidPolicy {
    fn new() -> Self {
        Self {
            wwdr_oid: APPLE_WWDR_INTERMEDIATE_OID.parse().expect("valid OID"),
            receipt_signer_oid: APPLE_RECEIPT_SIGNER_OID.parse().expect("valid OID"),
        }
    }

    fn certificate_has_oid(certificate: &Certificate, oid: &Oid<'static>) -> bool {
        certificate
            .tbs_certificate
            .iter_extensions()
            .any(|ext| &ext.oid == oid)
    }
}

impl ValidationPolicy for AppStoreOidPolicy {
    fn verifying_critical_extensions(&self) -> Vec<Oid<'static>> {
        vec![]
    }

    fn chain_meets_policy_requirements(&mut self, chain: &UnverifiedCertificateChain) -> PolicyEvaluationResult {
        if chain.len() != EXPECTED_CHAIN_LENGTH {
            return Err(PolicyFailureReason::new("chain has unexpected length"));
        }

        let leaf = &chain[0];
        let intermediate = &chain[1];

        if !Self::certificate_has_oid(intermediate, &self.wwdr_oid) {
            return Err(PolicyFailureReason::new(
                "intermediate certificate does not contain WWDR OID",
            ));
        }

        if !Self::certificate_has_oid(leaf, &self.receipt_signer_oid) {
            return Err(PolicyFailureReason::new(
                "leaf certificate does not contain Receipt Signing OID",
            ));
        }

        Ok(())
    }
}

/// There are unlikely to be more than a couple of keys at once.
const MAXIMUM_CACHE_SIZE: usize = 32;
/// 15 minutes, in seconds.
const CACHE_TIME_LIMIT: u64 = 15 * 60;

#[derive(PartialEq, Eq, Hash)]
struct CacheKey {
    leaf: Vec<u8>,
    intermediate: Vec<u8>,
}

struct CacheValue {
    expiration_time: u64,
    public_key: Vec<u8>,
}

/// Verifies Apple's certificate chains.
pub struct ChainVerifier {
    root_certificates: Vec<Vec<u8>>,
    verified_public_key_cache: Mutex<HashMap<CacheKey, CacheValue>>,
}

impl ChainVerifier {
    /// Creates a verifier trusting the given DER-encoded root certificates.
    pub fn new(root_certificates: Vec<Vec<u8>>) -> Self {
        Self {
            root_certificates,
            verified_public_key_cache: Mutex::new(HashMap::new()),
        }
    }

    /// Verifies the chain and returns the leaf's DER-encoded SubjectPublicKeyInfo.
    ///
    /// # Arguments
    /// * `leaf` - DER-encoded leaf certificate
    /// * `intermediate` - DER-encoded intermediate certificate
    /// * `effective_date` - Optional Unix timestamp for validity checks
    /// * `enable_online_checks` - Whether the verified public key is cached,
    ///   matching the official Apple libraries. Revocation checking is not
    ///   performed.
    pub fn verify(
        &self,
        leaf: &[u8],
        intermediate: &[u8],
        effective_date: Option<u64>,
        enable_online_checks: bool,
    ) -> Result<Vec<u8>, ChainVerifierError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        self.verify_at(leaf, intermediate, effective_date, enable_online_checks, now)
    }

    /// [`ChainVerifier::verify`], with the current time injected. Exposed for
    /// deterministic testing of cache expiry.
    pub fn verify_at(
        &self,
        leaf: &[u8],
        intermediate: &[u8],
        effective_date: Option<u64>,
        enable_online_checks: bool,
        now: u64,
    ) -> Result<Vec<u8>, ChainVerifierError> {
        if enable_online_checks {
            if let Some(cached) = self.cached_public_key(leaf, intermediate, now) {
                return Ok(cached);
            }
        }

        let public_key = self.verify_without_caching(leaf, intermediate, effective_date)?;

        if enable_online_checks {
            self.store_public_key(leaf, intermediate, &public_key, now);
        }

        Ok(public_key)
    }

    /// Number of entries currently held. Exposed for testing.
    pub fn cache_len(&self) -> usize {
        self.verified_public_key_cache
            .lock()
            .map(|c| c.len())
            .unwrap_or(0)
    }

    fn verify_without_caching(
        &self,
        leaf: &[u8],
        intermediate: &[u8],
        effective_date: Option<u64>,
    ) -> Result<Vec<u8>, ChainVerifierError> {
        let leaf = parse_certificate(leaf)?;
        let intermediate_der = intermediate.to_vec();

        let mut roots = CertificateStore::new();
        for root_der in &self.root_certificates {
            let root = parse_certificate(root_der)?;
            roots.append(root);
        }

        let validation_time = effective_date
            .map(|d| i64::try_from(d).unwrap_or(i64::MAX))
            .unwrap_or_else(|| {
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_secs() as i64)
                    .unwrap_or(0)
            });

        let policy = x509_validator::policy! {
            RFC5280Policy::new(validation_time);
            AppStoreOidPolicy::new()
        };

        let mut validator = x509_validator::Validator::with_policy(roots, policy);

        match validator.validate(&leaf, std::slice::from_ref(&intermediate_der)) {
            ChainValidationResultOwned::ValidCertificate(chain) => Ok(leaf_spki_der(chain.leaf())),
            ChainValidationResultOwned::CouldNotValidate(reasons) => {
                let expired = reasons
                    .iter()
                    .any(|reason| reason.to_string().contains("expired"));
                if expired {
                    Err(ChainVerifierError::VerificationFailure(
                        ChainVerificationFailureReason::CertificateExpired,
                    ))
                } else {
                    Err(ChainVerifierError::VerificationFailure(
                        ChainVerificationFailureReason::InvalidCertificate,
                    ))
                }
            }
        }
    }

    fn cached_public_key(&self, leaf: &[u8], intermediate: &[u8], now: u64) -> Option<Vec<u8>> {
        let key = CacheKey {
            leaf: leaf.to_vec(),
            intermediate: intermediate.to_vec(),
        };
        let cache = self
            .verified_public_key_cache
            .lock()
            .ok()?;
        let value = cache.get(&key)?;
        if value.expiration_time > now {
            Some(value.public_key.clone())
        } else {
            None
        }
    }

    fn store_public_key(&self, leaf: &[u8], intermediate: &[u8], public_key: &[u8], now: u64) {
        let Ok(mut cache) = self.verified_public_key_cache.lock() else {
            return;
        };

        cache.insert(
            CacheKey {
                leaf: leaf.to_vec(),
                intermediate: intermediate.to_vec(),
            },
            CacheValue {
                expiration_time: now + CACHE_TIME_LIMIT,
                public_key: public_key.to_vec(),
            },
        );

        if cache.len() > MAXIMUM_CACHE_SIZE {
            cache.retain(|_, v| v.expiration_time > now);
        }
    }
}

fn parse_certificate(der: &[u8]) -> Result<Certificate<'_>, ChainVerifierError> {
    Certificate::parse(der).map_err(|_| {
        ChainVerifierError::VerificationFailure(ChainVerificationFailureReason::InvalidCertificate)
    })
}

fn leaf_spki_der(leaf: &Certificate) -> Vec<u8> {
    leaf.tbs_certificate
        .subject_pki
        .raw
        .to_vec()
}
