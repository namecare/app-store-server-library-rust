//! ring backend implementation.

use ring::rand::SystemRandom;
use ring::signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING};

use crate::crypto::{
    CryptoError, CryptoProvider, P256PrivateKey, P256PublicKey, P256Signature, P256SigningSuite,
};

/// Marker type implementing every capability this backend provides.
#[derive(Debug)]
struct Ring;

#[derive(Debug)]
struct RingP256PrivateKey {
    key_pair: EcdsaKeyPair,
}

impl P256PrivateKey for RingP256PrivateKey {
    fn signature(&self, message: &[u8]) -> Result<Box<dyn P256Signature>, CryptoError> {
        let rng = SystemRandom::new();
        let sig = self
            .key_pair
            .sign(&rng, message)
            .map_err(|e| CryptoError::SigningError(e.to_string()))?;

        // The key pair was loaded with FIXED_SIGNING, so this is already r‖s.
        let raw: [u8; 64] = sig
            .as_ref()
            .try_into()
            .map_err(|_| CryptoError::SigningError("unexpected signature length".into()))?;

        Ok(Box::new(RingP256Signature { raw }))
    }
}

#[derive(Debug)]
struct RingP256PublicKey {
    spki_der: Vec<u8>,
}

impl P256PublicKey for RingP256PublicKey {
    fn is_valid_signature(
        &self,
        signature: &dyn P256Signature,
        message: &[u8],
    ) -> Result<(), CryptoError> {
        use ring::signature::{UnparsedPublicKey, ECDSA_P256_SHA256_FIXED};

        // SPKI DER wraps the SEC1 point; the last 65 bytes are the uncompressed
        // point ring expects.
        if self.spki_der.len() < 65 {
            return Err(CryptoError::KeyError("SPKI too short".into()));
        }
        let point = &self.spki_der[self.spki_der.len() - 65..];

        UnparsedPublicKey::new(&ECDSA_P256_SHA256_FIXED, point)
            .verify(message, &signature.raw_representation())
            .map_err(|e| CryptoError::VerificationError(e.to_string()))
    }
}

#[derive(Debug)]
struct RingP256Signature {
    raw: [u8; 64],
}

impl P256Signature for RingP256Signature {
    fn raw_representation(&self) -> [u8; 64] {
        self.raw
    }

    fn der_representation(&self) -> Result<Vec<u8>, CryptoError> {
        Ok(ecdsa_raw_to_der(&self.raw))
    }
}

impl P256SigningSuite for Ring {
    fn private_key(&self, pem: &str) -> Result<Box<dyn P256PrivateKey>, CryptoError> {
        let der = decode_pem(pem).map_err(|e| CryptoError::KeyError(e.to_string()))?;

        // FIXED_SIGNING so `sign` yields r‖s directly — no conversion needed.
        let key_pair =
            EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &der, &SystemRandom::new())
                .map_err(|e| CryptoError::KeyError(e.to_string()))?;

        Ok(Box::new(RingP256PrivateKey { key_pair }))
    }

    fn public_key(&self, spki_der: &[u8]) -> Result<Box<dyn P256PublicKey>, CryptoError> {
        Ok(Box::new(RingP256PublicKey {
            spki_der: spki_der.to_vec(),
        }))
    }

    fn signature_from_raw(&self, rs: &[u8; 64]) -> Result<Box<dyn P256Signature>, CryptoError> {
        Ok(Box::new(RingP256Signature { raw: *rs }))
    }
}

/// Simple PEM decoder - extracts base64 content between BEGIN/END markers
fn decode_pem(pem: &str) -> Result<Vec<u8>, &'static str> {
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

/// Converts a fixed-width `r‖s` ECDSA signature to DER.
///
/// ring exposes no signature re-encoder, so the `SEQUENCE { INTEGER r,
/// INTEGER s }` is written directly. Both integers are at most 32 bytes, so
/// the SEQUENCE always fits the short-form length octet.
fn ecdsa_raw_to_der(rs: &[u8; 64]) -> Vec<u8> {
    let r = der_integer(&rs[..32]);
    let s = der_integer(&rs[32..]);

    let mut der = Vec::with_capacity(2 + r.len() + s.len());
    der.push(0x30);
    der.push((r.len() + s.len()) as u8);
    der.extend_from_slice(&r);
    der.extend_from_slice(&s);
    der
}

/// Encodes one unsigned big-endian component as a DER INTEGER: leading zero
/// bytes are stripped (DER requires the minimal encoding), and a single 0x00
/// is prepended when the high bit is set so the value stays positive.
fn der_integer(component: &[u8]) -> Vec<u8> {
    let value: &[u8] = match component.iter().position(|&b| b != 0) {
        Some(first) => &component[first..],
        None => &[0],
    };

    let needs_pad = value[0] & 0x80 != 0;
    let len = value.len() + usize::from(needs_pad);

    let mut out = Vec::with_capacity(2 + len);
    out.push(0x02);
    out.push(len as u8);
    if needs_pad {
        out.push(0x00);
    }
    out.extend_from_slice(value);
    out
}

#[cfg(test)]
mod signature_tests {
    use super::ecdsa_raw_to_der;
    use crate::crypto::CryptoProvider;

    #[test]
    fn raw_is_64_bytes_and_der_is_well_formed() {
        let pem = include_str!("../../../tests/resources/certs/testSigningKey.p8");
        let provider = CryptoProvider::default_provider();

        let key = provider.p256_signing.private_key(pem).expect("load key");
        let sig = key.signature(b"hello world").expect("sign");

        // FIXED_SIGNING must yield exactly r‖s, never DER.
        let raw = sig.raw_representation();
        assert_eq!(raw.len(), 64);

        let der = sig.der_representation().expect("convert to DER");
        assert_eq!(der[0], 0x30, "DER must start with SEQUENCE");
        assert_eq!(der[1] as usize, der.len() - 2, "length byte must match");
        assert!(der.len() >= 68 && der.len() <= 72, "got {} bytes", der.len());
    }

    #[test]
    fn der_and_raw_describe_the_same_signature() {
        let pem = include_str!("../../../tests/resources/certs/testSigningKey.p8");
        let provider = CryptoProvider::default_provider();

        let key = provider.p256_signing.private_key(pem).expect("load key");
        let sig = key.signature(b"same signature").expect("sign");

        let raw = sig.raw_representation();
        let der = sig.der_representation().expect("convert to DER");

        // Every byte of r and s must appear in the DER, in order, ignoring
        // ASN.1 framing and any minimal-encoding adjustments.
        let r_trimmed: Vec<u8> = raw[..32].iter().copied().skip_while(|&b| b == 0).collect();
        let s_trimmed: Vec<u8> = raw[32..].iter().copied().skip_while(|&b| b == 0).collect();

        assert!(
            der.windows(r_trimmed.len()).any(|w| w == r_trimmed.as_slice()),
            "r not found in DER"
        );
        assert!(
            der.windows(s_trimmed.len()).any(|w| w == s_trimmed.as_slice()),
            "s not found in DER"
        );
    }

    /// The hand-rolled encoder has to handle the two cases ring's absent
    /// encoder would: a high bit that needs a 0x00 pad, and leading zeros
    /// that DER requires stripped.
    #[test]
    fn der_integers_are_minimally_encoded_and_positive() {
        let mut rs = [0u8; 64];
        rs[0] = 0xFF; // r: high bit set, must be padded
        rs[32 + 31] = 0x01; // s: 31 leading zeros, must be stripped to one byte

        let der = ecdsa_raw_to_der(&rs);

        // SEQUENCE { INTEGER 00 FF 00*31, INTEGER 01 }
        assert_eq!(der[0], 0x30);
        assert_eq!(der[1] as usize, der.len() - 2);
        assert_eq!(der[2], 0x02);
        assert_eq!(der[3], 33, "r must be padded to 33 bytes");
        assert_eq!(der[4], 0x00, "pad byte keeps r positive");
        assert_eq!(der[5], 0xFF);
        assert_eq!(&der[der.len() - 3..], &[0x02, 0x01, 0x01], "s is one byte");
    }
}

pub const DEFAULT_PROVIDER: CryptoProvider = CryptoProvider {
    p256_signing: &Ring,
};
