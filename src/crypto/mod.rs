//! Cryptographic backend abstraction.

#[cfg(feature = "rust_crypto")]
pub mod rust_crypto;

#[cfg(feature = "aws_lc")]
pub mod aws_lc;

#[cfg(feature = "ring")]
pub mod ring;

pub mod jws;

use std::fmt::Debug;
use std::sync::{Arc, OnceLock};

/// Simple PEM decoder - extracts base64 content between BEGIN/END markers
#[cfg(any(feature = "aws_lc", feature = "ring"))]
pub(crate) fn decode_pem(pem: &str) -> Result<Vec<u8>, &'static str> {
    use base64::prelude::*;

    let lines: Vec<&str> = pem.lines().collect();

    // Find BEGIN and END markers
    let start = lines
        .iter()
        .position(|l| l.starts_with("-----BEGIN"))
        .ok_or("missing BEGIN marker")?;
    let end = lines
        .iter()
        .position(|l| l.starts_with("-----END"))
        .ok_or("missing END marker")?;

    if end <= start + 1 {
        return Err("no content between markers");
    }

    // Concatenate base64 lines
    let b64: String = lines[start + 1..end].concat();

    BASE64_STANDARD.decode(&b64).map_err(|_| "invalid base64")
}

/// Converts a fixed-width `r‖s` ECDSA P-256 signature to `SEQUENCE { INTEGER r, INTEGER s }`.
#[cfg(any(feature = "aws_lc", feature = "ring"))]
pub(crate) fn ecdsa_raw_to_der(rs: &[u8; 64]) -> Result<Vec<u8>, CryptoError> {
    use asn1_rs::{Integer, Sequence, ToDer};

    let to_error = |e: asn1_rs::SerializeError| CryptoError::SigningError(e.to_string());

    let r = Integer::from_const_array::<32>(rs[..32].try_into().expect("32 bytes"));
    let s = Integer::from_const_array::<32>(rs[32..].try_into().expect("32 bytes"));

    Sequence::from_iter_to_der([r, s].into_iter())
        .map_err(to_error)?
        .to_der_vec()
        .map_err(to_error)
}

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
}

pub trait P256PrivateKey: Send + Sync + Debug {
    /// Signs `message` with ECDSA P-256, returning signature.
    ///
    /// The implementation hashes `message` internally with SHA-256 — callers
    /// pass the raw message, NOT a pre-computed digest.
    fn signature(&self, message: &[u8]) -> Result<P256Signature, CryptoError>;
}

/// ECDSA P-256 signature: `(raw, der)`.
pub type P256Signature = ([u8; 64], Vec<u8>);

pub trait P256PublicKey: Send + Sync + Debug {
    /// Returns `Ok(())` when `signature` (fixed-width `r‖s`, JWS/RFC 7515) is
    /// valid over `message`.
    ///
    /// `message` is the raw message; the implementation hashes it internally.
    fn is_valid_signature(
        &self,
        signature: &[u8; 64],
        message: &[u8],
    ) -> Result<(), CryptoError>;
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

#[cfg(all(test, any(feature = "aws_lc", feature = "ring")))]
mod der_tests {
    use super::ecdsa_raw_to_der;

    /// The two cases DER's minimal-encoding rules turn on: a high bit that
    /// needs a 0x00 pad, and leading zeros that must be stripped.
    #[test]
    fn der_integers_are_minimally_encoded_and_positive() {
        let mut rs = [0u8; 64];
        rs[0] = 0xFF; // r: high bit set, must be padded
        rs[32 + 31] = 0x01; // s: 31 leading zeros, must be stripped to one byte

        let der = ecdsa_raw_to_der(&rs).expect("encode");

        // SEQUENCE { INTEGER 00 FF 00*31, INTEGER 01 }
        assert_eq!(der[0], 0x30);
        assert_eq!(der[1] as usize, der.len() - 2);
        assert_eq!(der[2], 0x02);
        assert_eq!(der[3], 33, "r must be padded to 33 bytes");
        assert_eq!(der[4], 0x00, "pad byte keeps r positive");
        assert_eq!(der[5], 0xFF);
        assert_eq!(&der[der.len() - 3..], &[0x02, 0x01, 0x01], "s is one byte");
    }

    /// An all-zero component is the one case where stripping must stop short
    /// of emptying the INTEGER.
    #[test]
    fn zero_component_encodes_as_a_single_zero_byte() {
        let der = ecdsa_raw_to_der(&[0u8; 64]).expect("encode");

        // SEQUENCE { INTEGER 00, INTEGER 00 }
        assert_eq!(der, vec![0x30, 0x06, 0x02, 0x01, 0x00, 0x02, 0x01, 0x00]);
    }
}