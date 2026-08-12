//! ring backend implementation.

use ring::rand::SystemRandom;
use ring::signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING};

use crate::crypto::{
    decode_pem, ecdsa_raw_to_der, CryptoError, CryptoProvider, P256PrivateKey, P256PublicKey, P256Signature,
    P256SigningSuite,
};

#[derive(Debug)]
struct Ring;

impl P256PrivateKey for EcdsaKeyPair {
    fn signature(&self, message: &[u8]) -> Result<P256Signature, CryptoError> {
        let rng = SystemRandom::new();
        let sig = self
            .sign(&rng, message)
            .map_err(|e| CryptoError::SigningError(e.to_string()))?;

        let raw: [u8; 64] = sig
            .as_ref()
            .try_into()
            .map_err(|_| CryptoError::SigningError("unexpected signature length".into()))?;

        let der = ecdsa_raw_to_der(&raw)?;
        Ok((raw, der))
    }
}

impl P256PublicKey for ring::signature::UnparsedPublicKey<Vec<u8>> {
    fn is_valid_signature(&self, signature: &[u8; 64], message: &[u8]) -> Result<(), CryptoError> {
        self.verify(message, signature)
            .map_err(|e| CryptoError::VerificationError(e.to_string()))
    }
}

impl P256SigningSuite for Ring {
    fn private_key(&self, pem: &str) -> Result<Box<dyn P256PrivateKey>, CryptoError> {
        let der = decode_pem(pem).map_err(|e| CryptoError::KeyError(e.to_string()))?;

        let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &der, &SystemRandom::new())
            .map_err(|e| CryptoError::KeyError(e.to_string()))?;

        Ok(Box::new(key_pair))
    }

    fn public_key(&self, spki_der: &[u8]) -> Result<Box<dyn P256PublicKey>, CryptoError> {
        use ring::signature::{UnparsedPublicKey, ECDSA_P256_SHA256_FIXED};

        if spki_der.len() < 65 {
            return Err(CryptoError::KeyError("SPKI too short".into()));
        }
        let point = spki_der[spki_der.len() - 65..].to_vec();

        Ok(Box::new(UnparsedPublicKey::new(
            &ECDSA_P256_SHA256_FIXED,
            point,
        )))
    }
}

pub const DEFAULT_PROVIDER: CryptoProvider = CryptoProvider { p256_signing: &Ring };

#[cfg(test)]
mod signature_tests {
    use crate::crypto::CryptoProvider;

    #[test]
    fn raw_is_64_bytes_and_der_is_well_formed() {
        let pem = include_str!("../../tests/resources/certs/testSigningKey.p8");
        let provider = CryptoProvider::default_provider();

        let key = provider
            .p256_signing
            .private_key(pem)
            .expect("load key");
        let (raw, der) = key
            .signature(b"hello world")
            .expect("sign");

        // FIXED_SIGNING must yield exactly r‖s, never DER.
        assert_eq!(raw.len(), 64);

        assert_eq!(der[0], 0x30, "DER must start with SEQUENCE");
        assert_eq!(der[1] as usize, der.len() - 2, "length byte must match");
        assert!(
            der.len() >= 68 && der.len() <= 72,
            "got {} bytes",
            der.len()
        );
    }

    #[test]
    fn der_and_raw_describe_the_same_signature() {
        let pem = include_str!("../../tests/resources/certs/testSigningKey.p8");
        let provider = CryptoProvider::default_provider();

        let key = provider
            .p256_signing
            .private_key(pem)
            .expect("load key");
        let (raw, der) = key
            .signature(b"same signature")
            .expect("sign");

        // Every byte of r and s must appear in the DER, in order, ignoring
        // ASN.1 framing and any minimal-encoding adjustments.
        let r_trimmed: Vec<u8> = raw[..32]
            .iter()
            .copied()
            .skip_while(|&b| b == 0)
            .collect();
        let s_trimmed: Vec<u8> = raw[32..]
            .iter()
            .copied()
            .skip_while(|&b| b == 0)
            .collect();

        assert!(
            der.windows(r_trimmed.len())
                .any(|w| w == r_trimmed.as_slice()),
            "r not found in DER"
        );
        assert!(
            der.windows(s_trimmed.len())
                .any(|w| w == s_trimmed.as_slice()),
            "s not found in DER"
        );
    }
}
