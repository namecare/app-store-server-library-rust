//! RustCrypto backend implementation

use p256::ecdsa::signature::Signer;
use p256::ecdsa::SigningKey;
use p256::pkcs8::DecodePrivateKey;

use crate::crypto::{
    CryptoError, CryptoProvider, P256PrivateKey, P256PublicKey, P256Signature,
    P256SigningSuite,
};

/// Marker type implementing every capability this backend provides.
#[derive(Debug)]
struct RustCrypto;

#[derive(Debug)]
struct RustCryptoP256PrivateKey {
    key: SigningKey,
}

impl P256PrivateKey for RustCryptoP256PrivateKey {
    fn signature(&self, message: &[u8]) -> Result<Box<dyn P256Signature>, CryptoError> {
        let sig: p256::ecdsa::Signature = self.key.sign(message);
        Ok(Box::new(RustCryptoP256Signature { sig }))
    }
}

#[derive(Debug)]
struct RustCryptoP256PublicKey {
    key: p256::ecdsa::VerifyingKey,
}

impl P256PublicKey for RustCryptoP256PublicKey {
    fn is_valid_signature(
        &self,
        signature: &dyn P256Signature,
        message: &[u8],
    ) -> Result<(), CryptoError> {
        use signature::Verifier;

        // Rebuild from the raw form so signatures from any backend are accepted.
        let sig = p256::ecdsa::Signature::from_slice(&signature.raw_representation())
            .map_err(|e| CryptoError::VerificationError(e.to_string()))?;

        self.key
            .verify(message, &sig)
            .map_err(|e| CryptoError::VerificationError(e.to_string()))
    }
}

#[derive(Debug)]
struct RustCryptoP256Signature {
    sig: p256::ecdsa::Signature,
}

impl P256Signature for RustCryptoP256Signature {
    fn raw_representation(&self) -> [u8; 64] {
        self.sig.to_bytes().into()
    }

    fn der_representation(&self) -> Result<Vec<u8>, CryptoError> {
        Ok(self.sig.to_der().to_bytes().to_vec())
    }
}

impl P256SigningSuite for RustCrypto {
    fn private_key(&self, pem: &str) -> Result<Box<dyn P256PrivateKey>, CryptoError> {
        let mut buf = [0u8; 2048];
        let (_, der) = pem_rfc7468::decode(pem.as_bytes(), &mut buf)
            .map_err(|e| CryptoError::KeyError(e.to_string()))?;

        let key = SigningKey::from_pkcs8_der(der)
            .map_err(|e| CryptoError::KeyError(e.to_string()))?;

        Ok(Box::new(RustCryptoP256PrivateKey { key }))
    }

    fn public_key(&self, spki_der: &[u8]) -> Result<Box<dyn P256PublicKey>, CryptoError> {
        use p256::pkcs8::DecodePublicKey;

        let key = p256::ecdsa::VerifyingKey::from_public_key_der(spki_der)
            .map_err(|e| CryptoError::KeyError(e.to_string()))?;

        Ok(Box::new(RustCryptoP256PublicKey { key }))
    }

    fn signature_from_raw(&self, rs: &[u8; 64]) -> Result<Box<dyn P256Signature>, CryptoError> {
        let sig = p256::ecdsa::Signature::from_slice(rs)
            .map_err(|e| CryptoError::VerificationError(e.to_string()))?;
        Ok(Box::new(RustCryptoP256Signature { sig }))
    }
}

pub const DEFAULT_PROVIDER: CryptoProvider = CryptoProvider {
    p256_signing: &RustCrypto,
};
