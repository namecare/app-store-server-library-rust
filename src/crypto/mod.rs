//! Cryptographic backend abstraction.

#[cfg(feature = "rust_crypto")]
pub mod rust_crypto;

#[cfg(feature = "aws_lc")]
pub mod aws_lc;

#[cfg(feature = "ring")]
pub mod ring;

#[cfg(feature = "receipt-utility")]
pub mod asn1;

pub mod jws;

use std::fmt::Debug;
use std::sync::{Arc, OnceLock};

/// Errors raised by cryptographic primitives.
#[derive(thiserror::Error, Debug)]
pub enum CryptoError {
    #[error("Key error: {0}")]
    KeyError(String),

    #[error("Signing error: {0}")]
    SigningError(String),

    #[error("Verification error: {0}")]
    VerificationError(String),
}

/// ECDSA P-256 signing and verification.
pub trait P256SigningSuite: Send + Sync + Debug {
    /// `pem` is a PKCS#8 PEM private key, as Apple issues them (`.p8`).
    fn private_key(&self, pem: &str) -> Result<Box<dyn P256PrivateKey>, CryptoError>;

    /// `spki_der` is a DER-encoded SubjectPublicKeyInfo.
    fn public_key(&self, spki_der: &[u8]) -> Result<Box<dyn P256PublicKey>, CryptoError>;

    /// Parses a signature from its fixed-width `r‖s` form (JWS/RFC 7515).
    ///
    /// This is the only way to build a signature from bytes: JWS verification
    /// is the only place this library parses one. Certificate signatures are
    /// consumed inside `X509Suite` and never surface here.
    fn signature_from_raw(&self, rs: &[u8; 64]) -> Result<Box<dyn P256Signature>, CryptoError>;
}

pub trait P256PrivateKey: Send + Sync + Debug {
    /// Signs `message` with ECDSA P-256.
    ///
    /// The implementation hashes `message` internally with SHA-256 — callers
    /// pass the raw message, NOT a pre-computed digest.
    fn signature(&self, message: &[u8]) -> Result<Box<dyn P256Signature>, CryptoError>;
}

pub trait P256PublicKey: Send + Sync + Debug {
    /// Returns `Ok(())` when `signature` is valid over `message`.
    ///
    /// `message` is the raw message; the implementation hashes it internally.
    fn is_valid_signature(
        &self,
        signature: &dyn P256Signature,
        message: &[u8],
    ) -> Result<(), CryptoError>;
}

/// An ECDSA P-256 signature, renderable in either standard encoding.
///
/// The signature is the integer pair `(r, s)`; the standards disagree on how
/// to write it down. JWS requires fixed-width `r‖s` (RFC 7515 §3.1); Apple's
/// promotional offers use DER. Both views describe the SAME signature —
/// implementations store one form and derive the other, never sign twice.
pub trait P256Signature: Send + Sync + Debug {
    /// `r‖s`, each zero-padded to 32 bytes. Always exactly 64 bytes.
    fn raw_representation(&self) -> [u8; 64];

    /// `SEQUENCE { INTEGER r, INTEGER s }` — variable length, ~70-72 bytes.
    fn der_representation(&self) -> Result<Vec<u8>, CryptoError>;
}

/// Controls the cryptography used by this library.
///
/// Individual fields can be overridden using struct-update syntax against a
/// backend's `DEFAULT_PROVIDER`:
///
/// ```ignore
/// CryptoProvider { sha256_hasher: &MyHasher, ..DEFAULT_PROVIDER }
///     .install_default()
///     .expect("provider already installed");
/// ```
#[derive(Debug, Clone)]
pub struct CryptoProvider {
    /// ECDSA P-256 signing and verification.
    pub p256_signing: &'static dyn P256SigningSuite,
}

static PROCESS_DEFAULT: OnceLock<Arc<CryptoProvider>> = OnceLock::new();

impl CryptoProvider {
    /// Sets this `CryptoProvider` as the default for this process.
    ///
    /// This can be called successfully at most once in any process execution.
    ///
    /// After calling this, other callers can obtain a reference to the installed
    /// default via [`CryptoProvider::get_default()`].
    pub fn install_default(self) -> Result<(), Arc<Self>> {
        PROCESS_DEFAULT.set(Arc::new(self))
    }

    /// Returns the default `CryptoProvider` for this process.
    ///
    /// This will be `None` if no default has been set yet.
    pub fn get_default() -> Option<&'static Arc<Self>> {
        PROCESS_DEFAULT.get()
    }

    /// The process default provider, installing the crate-feature default if
    /// none has been set.
    pub fn default_provider() -> &'static Arc<Self> {
        PROCESS_DEFAULT.get_or_init(|| Arc::new(Self::from_crate_features()))
    }

    #[allow(unreachable_code)]
    fn from_crate_features() -> Self {
        #[cfg(feature = "rust_crypto")]
        {
            return rust_crypto::DEFAULT_PROVIDER;
        }

        #[cfg(feature = "aws_lc")]
        {
            return aws_lc::DEFAULT_PROVIDER;
        }

        #[cfg(feature = "ring")]
        {
            return ring::DEFAULT_PROVIDER;
        }

        panic!("No crypto backend. Enable 'rust_crypto', 'aws_lc' or 'ring' feature.");
    }
}
