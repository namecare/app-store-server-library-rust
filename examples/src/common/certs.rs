//! Root certificate sources for the examples.
//!
//! Production traffic verifies against Apple's four public root CAs, which are
//! embedded below. `DEMO_MODE` swaps in the repository's self-signed test CA so
//! the bundled `testNotification` fixture verifies offline.

use std::path::{Path, PathBuf};

use base64::engine::general_purpose::STANDARD;
use base64::Engine;

/// Apple's four public root certificates, base64-encoded DER.
///
/// Copied from `tests/common/mod.rs::REAL_APPLE_MULTI_ROOT_BASE64_ENCODED`.
/// In order: Apple Root CA, Apple Computer Root CA, Apple Root CA - G2,
/// Apple Root CA - G3.
const APPLE_ROOTS_BASE64: [&str; 4] = [
    "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",
];

/// The repository's self-signed test CA. Verifies the bundled fixture only.
const DEMO_ROOT_DER: &[u8] = include_bytes!("../../assets/testCA.der");

/// Where a verifier's trusted roots come from.
#[derive(Debug, Clone)]
pub enum RootSource {
    /// Apple's four embedded production roots. The default.
    AppleProduction,
    /// The bundled self-signed test CA. Fixtures only.
    Demo,
    /// Every `.cer`/`.der` file in a directory.
    Directory(PathBuf),
}

/// Apple's four public root certificates as DER bytes.
pub fn apple_production_roots() -> Vec<Vec<u8>> {
    APPLE_ROOTS_BASE64
        .iter()
        .map(|b64| {
            STANDARD
                .decode(b64)
                .expect("embedded Apple root is valid base64")
        })
        .collect()
}

/// The bundled self-signed test CA as DER bytes.
pub fn demo_roots() -> Vec<Vec<u8>> {
    vec![DEMO_ROOT_DER.to_vec()]
}

/// Reads every `.cer`/`.der` file in `dir` as DER bytes.
pub fn roots_from_dir(dir: &Path) -> std::io::Result<Vec<Vec<u8>>> {
    let mut roots = Vec::new();

    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        let is_cert = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.eq_ignore_ascii_case("cer") || e.eq_ignore_ascii_case("der"))
            .unwrap_or(false);

        if is_cert {
            roots.push(std::fs::read(&path)?);
        }
    }

    Ok(roots)
}

/// Resolves a [`RootSource`] to DER-encoded certificates, logging which set is
/// in use so a misconfigured deployment is obvious in the logs.
pub fn load_roots(source: &RootSource) -> std::io::Result<Vec<Vec<u8>>> {
    match source {
        RootSource::AppleProduction => {
            println!("[certs] using Apple's embedded production root CAs");
            Ok(apple_production_roots())
        }
        RootSource::Demo => {
            println!(
                "[certs] WARNING: DEMO MODE - trusting the bundled self-signed test CA. \
                 This verifies only this repository's test fixtures and MUST NOT be used \
                 against real App Store traffic."
            );
            Ok(demo_roots())
        }
        RootSource::Directory(dir) => {
            let roots = roots_from_dir(dir)?;
            println!(
                "[certs] loaded {} root certificate(s) from {}",
                roots.len(),
                dir.display()
            );
            Ok(roots)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apple_production_roots_decode_to_four_certs() {
        let roots = apple_production_roots();
        assert_eq!(roots.len(), 4);
        // Every root must be non-empty DER starting with a SEQUENCE tag.
        for root in &roots {
            assert!(!root.is_empty());
            assert_eq!(root[0], 0x30);
        }
    }

    #[test]
    fn demo_roots_contain_the_bundled_test_ca() {
        let roots = demo_roots();
        assert_eq!(roots.len(), 1);
        assert_eq!(roots[0][0], 0x30);
    }
}
