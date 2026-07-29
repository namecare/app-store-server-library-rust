//! Cryptographic backend abstraction.

use std::sync::OnceLock;

#[cfg(feature = "rust_crypto")]
pub mod rust_crypto;

#[cfg(feature = "aws_lc")]
pub mod aws_lc;

use crate::chain_verifier::ChainVerifier;

pub type ChainVerifierFactory = fn() -> Box<dyn ChainVerifier>;

use crate::promotional_offer_signature_creator::{PromotionalOfferSignatureCreatorError, PromotionalOfferSigner};

pub type PromotionalOfferSignerFactory =
    fn(&str) -> Result<Box<dyn PromotionalOfferSigner>, PromotionalOfferSignatureCreatorError>;

#[derive(Clone, Copy)]
pub struct CryptoProvider {
    pub chain_verifier: ChainVerifierFactory,
    pub promotional_offer_signer: PromotionalOfferSignerFactory,
}

static PROCESS_DEFAULT: OnceLock<&'static CryptoProvider> = OnceLock::new();

impl CryptoProvider {
    pub fn install_default(&'static self) -> Result<(), &'static Self> {
        PROCESS_DEFAULT.set(self)
    }

    pub fn default_provider() -> &'static Self {
        PROCESS_DEFAULT.get_or_init(Self::from_crate_features)
    }

    #[allow(unreachable_code)]
    fn from_crate_features() -> &'static Self {
        #[cfg(feature = "rust_crypto")]
        {
            return &rust_crypto::DEFAULT_PROVIDER;
        }

        #[cfg(feature = "aws_lc")]
        {
            return &aws_lc::DEFAULT_PROVIDER;
        }

        panic!("No crypto backend. Enable 'rust_crypto' or 'aws_lc' feature.");
    }
}
