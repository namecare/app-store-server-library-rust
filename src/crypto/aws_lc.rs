//! AWS-LC backend implementation.

use aws_lc_rs::signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING};

use crate::crypto::{
    CryptoError, CryptoProvider, P256PrivateKey, P256PublicKey, P256Signature, P256SigningSuite,
};

/// Marker type implementing every capability this backend provides.
#[derive(Debug)]
struct AwsLc;

#[derive(Debug)]
struct AwsLcP256PrivateKey {
    key_pair: EcdsaKeyPair,
}

impl P256PrivateKey for AwsLcP256PrivateKey {
    fn signature(&self, message: &[u8]) -> Result<Box<dyn P256Signature>, CryptoError> {
        let rng = aws_lc_rs::rand::SystemRandom::new();
        let sig = self
            .key_pair
            .sign(&rng, message)
            .map_err(|e| CryptoError::SigningError(e.to_string()))?;

        // The key pair was loaded with FIXED_SIGNING, so this is already r‖s.
        let raw: [u8; 64] = sig
            .as_ref()
            .try_into()
            .map_err(|_| CryptoError::SigningError("unexpected signature length".into()))?;

        Ok(Box::new(AwsLcP256Signature { raw }))
    }
}

#[derive(Debug)]
struct AwsLcP256PublicKey {
    spki_der: Vec<u8>,
}

impl P256PublicKey for AwsLcP256PublicKey {
    fn is_valid_signature(
        &self,
        signature: &dyn P256Signature,
        message: &[u8],
    ) -> Result<(), CryptoError> {
        use aws_lc_rs::signature::{UnparsedPublicKey, ECDSA_P256_SHA256_FIXED};

        // SPKI DER wraps the SEC1 point; the last 65 bytes are the uncompressed
        // point aws-lc expects.
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
struct AwsLcP256Signature {
    raw: [u8; 64],
}

impl P256Signature for AwsLcP256Signature {
    fn raw_representation(&self) -> [u8; 64] {
        self.raw
    }

    fn der_representation(&self) -> Result<Vec<u8>, CryptoError> {
        ecdsa_raw_to_der(&self.raw)
            .map_err(|e| CryptoError::SigningError(e.to_string()))
    }
}

impl P256SigningSuite for AwsLc {
    fn private_key(&self, pem: &str) -> Result<Box<dyn P256PrivateKey>, CryptoError> {
        let der = decode_pem(pem).map_err(|e| CryptoError::KeyError(e.to_string()))?;

        // FIXED_SIGNING so `sign` yields r‖s directly — no conversion needed.
        let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &der)
            .map_err(|e| CryptoError::KeyError(e.to_string()))?;

        Ok(Box::new(AwsLcP256PrivateKey { key_pair }))
    }

    fn public_key(&self, spki_der: &[u8]) -> Result<Box<dyn P256PublicKey>, CryptoError> {
        Ok(Box::new(AwsLcP256PublicKey { spki_der: spki_der.to_vec() }))
    }

    fn signature_from_raw(&self, rs: &[u8; 64]) -> Result<Box<dyn P256Signature>, CryptoError> {
        Ok(Box::new(AwsLcP256Signature { raw: *rs }))
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

    BASE64_STANDARD
        .decode(&b64)
        .map_err(|_| "invalid base64")
}

/// Converts a fixed-width `r‖s` ECDSA signature to DER.
///
/// Uses aws-lc's own `ECDSA_SIG` encoder rather than hand-written ASN.1.
fn ecdsa_raw_to_der(rs: &[u8; 64]) -> Result<Vec<u8>, &'static str> {
    use aws_lc_sys::{BN_bin2bn, BN_free, ECDSA_SIG_free, ECDSA_SIG_new, ECDSA_SIG_set0, ECDSA_SIG_to_bytes, OPENSSL_free};
    use std::ptr::null_mut;
    use std::slice;

    unsafe {
        let r = BN_bin2bn(rs[..32].as_ptr(), 32, null_mut());
        if r.is_null() {
            return Err("failed to parse r");
        }
        let s = BN_bin2bn(rs[32..].as_ptr(), 32, null_mut());
        if s.is_null() {
            BN_free(r);
            return Err("failed to parse s");
        }

        let sig = ECDSA_SIG_new();
        if sig.is_null() {
            BN_free(r);
            BN_free(s);
            return Err("failed to allocate ECDSA_SIG");
        }

        // set0 takes ownership of r and s on success.
        if ECDSA_SIG_set0(sig, r, s) != 1 {
            BN_free(r);
            BN_free(s);
            ECDSA_SIG_free(sig);
            return Err("failed to set ECDSA_SIG components");
        }

        let mut out: *mut u8 = null_mut();
        let mut out_len: usize = 0;
        let ok = ECDSA_SIG_to_bytes(&mut out, &mut out_len, sig);
        ECDSA_SIG_free(sig);

        if ok != 1 || out.is_null() {
            return Err("failed to encode ECDSA_SIG");
        }

        let der = slice::from_raw_parts(out, out_len).to_vec();
        OPENSSL_free(out as *mut _);
        Ok(der)
    }
}

#[cfg(test)]
mod signature_tests {
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
}

pub const DEFAULT_PROVIDER: CryptoProvider = CryptoProvider {
    p256_signing: &AwsLc,
};