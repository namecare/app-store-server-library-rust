//! RustCrypto backend implementation

use p256::ecdsa::signature::Signer;
use p256::ecdsa::SigningKey;
use p256::pkcs8::DecodePrivateKey;

use crate::crypto::{CryptoError, CryptoProvider, P256PrivateKey, P256PublicKey, P256Signature, P256SigningSuite};

#[derive(Debug)]
struct RustCrypto;

impl P256PrivateKey for SigningKey {
    fn signature(&self, message: &[u8]) -> Result<P256Signature, CryptoError> {
        let sig: p256::ecdsa::Signature = self.sign(message);
        let raw: [u8; 64] = sig.to_bytes().into();
        let der = sig.to_der().to_bytes().to_vec();
        Ok((raw, der))
    }
}

impl P256PublicKey for p256::ecdsa::VerifyingKey {
    fn is_valid_signature(&self, signature: &[u8; 64], message: &[u8]) -> Result<(), CryptoError> {
        use signature::Verifier;

        let sig =
            p256::ecdsa::Signature::from_slice(signature).map_err(|e| CryptoError::VerificationError(e.to_string()))?;

        self.verify(message, &sig)
            .map_err(|e| CryptoError::VerificationError(e.to_string()))
    }
}

impl P256SigningSuite for RustCrypto {
    fn private_key(&self, pem: &str) -> Result<Box<dyn P256PrivateKey>, CryptoError> {
        let mut buf = [0u8; 2048];
        let (_, der) =
            pem_rfc7468::decode(pem.as_bytes(), &mut buf).map_err(|e| CryptoError::KeyError(e.to_string()))?;

        let key = SigningKey::from_pkcs8_der(der).map_err(|e| CryptoError::KeyError(e.to_string()))?;

        Ok(Box::new(key))
    }

    fn public_key(&self, spki_der: &[u8]) -> Result<Box<dyn P256PublicKey>, CryptoError> {
        use p256::pkcs8::DecodePublicKey;

        let key = p256::ecdsa::VerifyingKey::from_public_key_der(spki_der)
            .map_err(|e| CryptoError::KeyError(e.to_string()))?;

        Ok(Box::new(key))
    }
}

pub const DEFAULT_PROVIDER: CryptoProvider = CryptoProvider {
    p256_signing: &RustCrypto,
};
