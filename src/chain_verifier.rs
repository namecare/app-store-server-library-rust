use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use thiserror::Error;
use crate::chain_verifier::ChainVerificationFailureReason::{CertificateExpired, InvalidCertificate, InvalidChainLength, InvalidEffectiveDate};

use x509_parser::certificate::X509Certificate;
use x509_parser::prelude::{FromDer, ASN1Time};
use x509_parser::der_parser::asn1_rs::oid;
use x509_parser::error::X509Error;

#[derive(Error, Debug, PartialEq)]
pub enum ChainVerifierError {
    #[error("VerificationFailure: [{0}]")]
    VerificationFailure(ChainVerificationFailureReason),

    #[error("InternalX509Error: [{0}]")]
    InternalX509Error(#[from] X509Error),

    #[error("InternalDecodeError: [{0}]")]
    InternalDecodeError(#[from] base64::DecodeError)
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
    CertificateExpired
}

const EXPECTED_CHAIN_LENGTH: usize = 3;

/// Verifies a certificate chain.
///
/// This function verifies a certificate chain consisting of multiple certificates. It performs various
/// checks to ensure the validity and integrity of the chain.
///
/// # Arguments
///
/// * `certificates`: A vector of byte slices containing the certificates in the chain.
/// * `root_certificates`: A vector of byte slices containing the root certificates.
/// * `effective_date`: An optional Unix timestamp representing the effective date for the chain validation.
///
/// # Returns
///
/// * `Ok(Vec<u8>)`: If the certificate chain is valid, it returns the public key data from the leaf certificate.
/// * `Err(ChainVerifierError)`: If the chain verification fails for any reason, it returns a `ChainVerifierError` enum.
///
/// # Example
///
/// ```rust
/// use app_store_server_library::chain_verifier::{verify_chain, ChainVerifierError};
///
/// fn main() {
///     let certificates: Vec<Vec<u8>> = vec![]; // Load your certificates here
///     let root_certificates: Vec<Vec<u8>> = vec![]; // Load your root certificates here
///     let effective_date: Option<u64> = None; // Provide an effective date if needed
///
///     match verify_chain(&certificates, &root_certificates, effective_date) {
///         Ok(public_key) => println!("Certificate chain is valid. Public key: {:?}", public_key),
///         Err(err) => eprintln!("Certificate chain verification failed: {}", err),
///     }
/// }
/// ```
///
/// TODO: Implement issuer checking
pub fn verify_chain(certificates: &Vec<Vec<u8>>, root_certificates: &Vec<Vec<u8>>, effective_date: Option<u64>) -> Result<Vec<u8>, ChainVerifierError> {
    if root_certificates.is_empty() {
        return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
    }

    if certificates.len() != EXPECTED_CHAIN_LENGTH {
        return Err(ChainVerifierError::VerificationFailure(InvalidChainLength));
    }

    let leaf_certificate = &certificates[0];
    let Ok(leaf_certificate) = X509Certificate::from_der(leaf_certificate.as_slice()) else {
        return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
    };
    let leaf_certificate = leaf_certificate.1;

    let Some(_) = leaf_certificate.get_extension_unique(&oid!(1.2.840.113635.100.6.11.1))? else {
        return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
    };

    let intermediate_certificate = &certificates[1];
    let Ok(intermediate_certificate) = X509Certificate::from_der(intermediate_certificate.as_slice()) else {
        return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
    };
    let intermediate_certificate = intermediate_certificate.1;

    let Some(_) = intermediate_certificate.get_extension_unique(&oid!(1.2.840.113635.100.6.2.1))? else {
        return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
    };

    let mut root_certificate: Option<X509Certificate> = None;

    for cert in root_certificates {
        let Ok(cert) = X509Certificate::from_der(&cert) else {
            return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
        };

        match intermediate_certificate.verify_signature(Some(cert.1.public_key())) {
            Ok(_) => (),
            Err(e) => return Err(ChainVerifierError::InternalX509Error(e))
        }

        root_certificate = Some(cert.1)
    }

    let Some(root_certificate) = root_certificate else {
        return Err(ChainVerifierError::VerificationFailure(InvalidCertificate))
    };

    leaf_certificate.verify_signature(Some(intermediate_certificate.public_key()))?;

    if let Some(date) = effective_date {
        let Ok(time) = ASN1Time::from_timestamp(i64::try_from(date).unwrap()) else {
            return Err(ChainVerifierError::VerificationFailure(InvalidEffectiveDate))
        };

        if !(root_certificate.validity.is_valid_at(time) &&
            leaf_certificate.validity.is_valid_at(time) &&
            intermediate_certificate.validity.is_valid_at(time)) {
            return Err(ChainVerifierError::VerificationFailure(CertificateExpired))
        }
    }

    let k = leaf_certificate.public_key().raw.to_vec();
    Ok(k)
}

#[cfg(test)]
mod tests {
    use base64::{DecodeError, Engine};
    use base64::engine::general_purpose::STANDARD;
    use crate::utils::{StringExt};
    use super::*;

    pub fn signed_payload() -> String {
        std::env::var("SIGNED_PAYLOAD").expect("SIGNED_PAYLOAD must be set")
    }

    pub fn apple_root_cert() -> String {
        std::env::var("APPLE_ROOT_BASE64_ENCODED").expect("APPLE_ROOT_BASE64_ENCODED must be set")
    }

    extern crate base64;

    use x509_parser::error::X509Error::SignatureVerificationError;

    const ROOT_CA_BASE64_ENCODED: &str = "MIIBgjCCASmgAwIBAgIJALUc5ALiH5pbMAoGCCqGSM49BAMDMDYxCzAJBgNVBAYTAlVTMRMwEQYDVQQIDApDYWxpZm9ybmlhMRIwEAYDVQQHDAlDdXBlcnRpbm8wHhcNMjMwMTA1MjEzMDIyWhcNMzMwMTAyMjEzMDIyWjA2MQswCQYDVQQGEwJVUzETMBEGA1UECAwKQ2FsaWZvcm5pYTESMBAGA1UEBwwJQ3VwZXJ0aW5vMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEc+/Bl+gospo6tf9Z7io5tdKdrlN1YdVnqEhEDXDShzdAJPQijamXIMHf8xWWTa1zgoYTxOKpbuJtDplz1XriTaMgMB4wDAYDVR0TBAUwAwEB/zAOBgNVHQ8BAf8EBAMCAQYwCgYIKoZIzj0EAwMDRwAwRAIgemWQXnMAdTad2JDJWng9U4uBBL5mA7WI05H7oH7c6iQCIHiRqMjNfzUAyiu9h6rOU/K+iTR0I/3Y/NSWsXHX+acc";
    const INTERMEDIATE_CA_BASE64_ENCODED: &str = "MIIBnzCCAUWgAwIBAgIBCzAKBggqhkjOPQQDAzA2MQswCQYDVQQGEwJVUzETMBEGA1UECAwKQ2FsaWZvcm5pYTESMBAGA1UEBwwJQ3VwZXJ0aW5vMB4XDTIzMDEwNTIxMzEwNVoXDTMzMDEwMTIxMzEwNVowRTELMAkGA1UEBhMCVVMxCzAJBgNVBAgMAkNBMRIwEAYDVQQHDAlDdXBlcnRpbm8xFTATBgNVBAoMDEludGVybWVkaWF0ZTBZMBMGByqGSM49AgEGCCqGSM49AwEHA0IABBUN5V9rKjfRiMAIojEA0Av5Mp0oF+O0cL4gzrTF178inUHugj7Et46NrkQ7hKgMVnjogq45Q1rMs+cMHVNILWqjNTAzMA8GA1UdEwQIMAYBAf8CAQAwDgYDVR0PAQH/BAQDAgEGMBAGCiqGSIb3Y2QGAgEEAgUAMAoGCCqGSM49BAMDA0gAMEUCIQCmsIKYs41ullssHX4rVveUT0Z7Is5/hLK1lFPTtun3hAIgc2+2RG5+gNcFVcs+XJeEl4GZ+ojl3ROOmll+ye7dynQ=";
    const LEAF_CERT_BASE64_ENCODED: &str = "MIIBoDCCAUagAwIBAgIBDDAKBggqhkjOPQQDAzBFMQswCQYDVQQGEwJVUzELMAkGA1UECAwCQ0ExEjAQBgNVBAcMCUN1cGVydGlubzEVMBMGA1UECgwMSW50ZXJtZWRpYXRlMB4XDTIzMDEwNTIxMzEzNFoXDTMzMDEwMTIxMzEzNFowPTELMAkGA1UEBhMCVVMxCzAJBgNVBAgMAkNBMRIwEAYDVQQHDAlDdXBlcnRpbm8xDTALBgNVBAoMBExlYWYwWTATBgcqhkjOPQIBBggqhkjOPQMBBwNCAATitYHEaYVuc8g9AjTOwErMvGyPykPa+puvTI8hJTHZZDLGas2qX1+ErxgQTJgVXv76nmLhhRJH+j25AiAI8iGsoy8wLTAJBgNVHRMEAjAAMA4GA1UdDwEB/wQEAwIHgDAQBgoqhkiG92NkBgsBBAIFADAKBggqhkjOPQQDAwNIADBFAiBX4c+T0Fp5nJ5QRClRfu5PSByRvNPtuaTsk0vPB3WAIAIhANgaauAj/YP9s0AkEhyJhxQO/6Q2zouZ+H1CIOehnMzQ";

    const INTERMEDIATE_CA_INVALID_OID_BASE64_ENCODED: &str = "MIIBnjCCAUWgAwIBAgIBDTAKBggqhkjOPQQDAzA2MQswCQYDVQQGEwJVUzETMBEGA1UECAwKQ2FsaWZvcm5pYTESMBAGA1UEBwwJQ3VwZXJ0aW5vMB4XDTIzMDEwNTIxMzYxNFoXDTMzMDEwMTIxMzYxNFowRTELMAkGA1UEBhMCVVMxCzAJBgNVBAgMAkNBMRIwEAYDVQQHDAlDdXBlcnRpbm8xFTATBgNVBAoMDEludGVybWVkaWF0ZTBZMBMGByqGSM49AgEGCCqGSM49AwEHA0IABBUN5V9rKjfRiMAIojEA0Av5Mp0oF+O0cL4gzrTF178inUHugj7Et46NrkQ7hKgMVnjogq45Q1rMs+cMHVNILWqjNTAzMA8GA1UdEwQIMAYBAf8CAQAwDgYDVR0PAQH/BAQDAgEGMBAGCiqGSIb3Y2QGAgIEAgUAMAoGCCqGSM49BAMDA0cAMEQCIFROtTE+RQpKxNXETFsf7Mc0h+5IAsxxo/X6oCC/c33qAiAmC5rn5yCOOEjTY4R1H1QcQVh+eUwCl13NbQxWCuwxxA==";
    const LEAF_CERT_FOR_INTERMEDIATE_CA_INVALID_OID_BASE64_ENCODED: &str = "MIIBnzCCAUagAwIBAgIBDjAKBggqhkjOPQQDAzBFMQswCQYDVQQGEwJVUzELMAkGA1UECAwCQ0ExEjAQBgNVBAcMCUN1cGVydGlubzEVMBMGA1UECgwMSW50ZXJtZWRpYXRlMB4XDTIzMDEwNTIxMzY1OFoXDTMzMDEwMTIxMzY1OFowPTELMAkGA1UEBhMCVVMxCzAJBgNVBAgMAkNBMRIwEAYDVQQHDAlDdXBlcnRpbm8xDTALBgNVBAoMBExlYWYwWTATBgcqhkjOPQIBBggqhkjOPQMBBwNCAATitYHEaYVuc8g9AjTOwErMvGyPykPa+puvTI8hJTHZZDLGas2qX1+ErxgQTJgVXv76nmLhhRJH+j25AiAI8iGsoy8wLTAJBgNVHRMEAjAAMA4GA1UdDwEB/wQEAwIHgDAQBgoqhkiG92NkBgsBBAIFADAKBggqhkjOPQQDAwNHADBEAiAUAs+gzYOsEXDwQquvHYbcVymyNqDtGw9BnUFp2YLuuAIgXxQ3Ie9YU0cMqkeaFd+lyo0asv9eyzk6stwjeIeOtTU=";
    const LEAF_CERT_INVALID_OID_BASE64_ENCODED: &str = "MIIBoDCCAUagAwIBAgIBDzAKBggqhkjOPQQDAzBFMQswCQYDVQQGEwJVUzELMAkGA1UECAwCQ0ExEjAQBgNVBAcMCUN1cGVydGlubzEVMBMGA1UECgwMSW50ZXJtZWRpYXRlMB4XDTIzMDEwNTIxMzczMVoXDTMzMDEwMTIxMzczMVowPTELMAkGA1UEBhMCVVMxCzAJBgNVBAgMAkNBMRIwEAYDVQQHDAlDdXBlcnRpbm8xDTALBgNVBAoMBExlYWYwWTATBgcqhkjOPQIBBggqhkjOPQMBBwNCAATitYHEaYVuc8g9AjTOwErMvGyPykPa+puvTI8hJTHZZDLGas2qX1+ErxgQTJgVXv76nmLhhRJH+j25AiAI8iGsoy8wLTAJBgNVHRMEAjAAMA4GA1UdDwEB/wQEAwIHgDAQBgoqhkiG92NkBgsCBAIFADAKBggqhkjOPQQDAwNIADBFAiAb+7S3i//bSGy7skJY9+D4VgcQLKFeYfIMSrUCmdrFqwIhAIMVwzD1RrxPRtJyiOCXLyibIvwcY+VS73HYfk0O9lgz";

    const LEAF_CERT_PUBLIC_KEY_BASE64_ENCODED: &str = "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE4rWBxGmFbnPIPQI0zsBKzLxsj8pD2vqbr0yPISUx2WQyxmrNql9fhK8YEEyYFV7++p5i4YUSR/o9uQIgCPIhrA==";

    const REAL_APPLE_ROOT_BASE64_ENCODED: &str = "MIICQzCCAcmgAwIBAgIILcX8iNLFS5UwCgYIKoZIzj0EAwMwZzEbMBkGA1UEAwwSQXBwbGUgUm9vdCBDQSAtIEczMSYwJAYDVQQLDB1BcHBsZSBDZXJ0aWZpY2F0aW9uIEF1dGhvcml0eTETMBEGA1UECgwKQXBwbGUgSW5jLjELMAkGA1UEBhMCVVMwHhcNMTQwNDMwMTgxOTA2WhcNMzkwNDMwMTgxOTA2WjBnMRswGQYDVQQDDBJBcHBsZSBSb290IENBIC0gRzMxJjAkBgNVBAsMHUFwcGxlIENlcnRpZmljYXRpb24gQXV0aG9yaXR5MRMwEQYDVQQKDApBcHBsZSBJbmMuMQswCQYDVQQGEwJVUzB2MBAGByqGSM49AgEGBSuBBAAiA2IABJjpLz1AcqTtkyJygRMc3RCV8cWjTnHcFBbZDuWmBSp3ZHtfTjjTuxxEtX/1H7YyYl3J6YRbTzBPEVoA/VhYDKX1DyxNB0cTddqXl5dvMVztK517IDvYuVTZXpmkOlEKMaNCMEAwHQYDVR0OBBYEFLuw3qFYM4iapIqZ3r6966/ayySrMA8GA1UdEwEB/wQFMAMBAf8wDgYDVR0PAQH/BAQDAgEGMAoGCCqGSM49BAMDA2gAMGUCMQCD6cHEFl4aXTQY2e3v9GwOAEZLuN+yRhHFD/3meoyhpmvOwgPUnPWTxnS4at+qIxUCMG1mihDK1A3UT82NQz60imOlM27jbdoXt2QfyFMm+YhidDkLF1vLUagM6BgD56KyKA==";
    const REAL_APPLE_INTERMEDIATE_BASE64_ENCODED: &str = "MIIDFjCCApygAwIBAgIUIsGhRwp0c2nvU4YSycafPTjzbNcwCgYIKoZIzj0EAwMwZzEbMBkGA1UEAwwSQXBwbGUgUm9vdCBDQSAtIEczMSYwJAYDVQQLDB1BcHBsZSBDZXJ0aWZpY2F0aW9uIEF1dGhvcml0eTETMBEGA1UECgwKQXBwbGUgSW5jLjELMAkGA1UEBhMCVVMwHhcNMjEwMzE3MjAzNzEwWhcNMzYwMzE5MDAwMDAwWjB1MUQwQgYDVQQDDDtBcHBsZSBXb3JsZHdpZGUgRGV2ZWxvcGVyIFJlbGF0aW9ucyBDZXJ0aWZpY2F0aW9uIEF1dGhvcml0eTELMAkGA1UECwwCRzYxEzARBgNVBAoMCkFwcGxlIEluYy4xCzAJBgNVBAYTAlVTMHYwEAYHKoZIzj0CAQYFK4EEACIDYgAEbsQKC94PrlWmZXnXgtxzdVJL8T0SGYngDRGpngn3N6PT8JMEb7FDi4bBmPhCnZ3/sq6PF/cGcKXWsL5vOteRhyJ45x3ASP7cOB+aao90fcpxSv/EZFbniAbNgZGhIhpIo4H6MIH3MBIGA1UdEwEB/wQIMAYBAf8CAQAwHwYDVR0jBBgwFoAUu7DeoVgziJqkipnevr3rr9rLJKswRgYIKwYBBQUHAQEEOjA4MDYGCCsGAQUFBzABhipodHRwOi8vb2NzcC5hcHBsZS5jb20vb2NzcDAzLWFwcGxlcm9vdGNhZzMwNwYDVR0fBDAwLjAsoCqgKIYmaHR0cDovL2NybC5hcHBsZS5jb20vYXBwbGVyb290Y2FnMy5jcmwwHQYDVR0OBBYEFD8vlCNR01DJmig97bB85c+lkGKZMA4GA1UdDwEB/wQEAwIBBjAQBgoqhkiG92NkBgIBBAIFADAKBggqhkjOPQQDAwNoADBlAjBAXhSq5IyKogMCPtw490BaB677CaEGJXufQB/EqZGd6CSjiCtOnuMTbXVXmxxcxfkCMQDTSPxarZXvNrkxU3TkUMI33yzvFVVRT4wxWJC994OsdcZ4+RGNsYDyR5gmdr0nDGg=";
    const REAL_APPLE_SIGNING_CERTIFICATE_BASE64_ENCODED: &str = "MIIEMDCCA7agAwIBAgIQaPoPldvpSoEH0lBrjDPv9jAKBggqhkjOPQQDAzB1MUQwQgYDVQQDDDtBcHBsZSBXb3JsZHdpZGUgRGV2ZWxvcGVyIFJlbGF0aW9ucyBDZXJ0aWZpY2F0aW9uIEF1dGhvcml0eTELMAkGA1UECwwCRzYxEzARBgNVBAoMCkFwcGxlIEluYy4xCzAJBgNVBAYTAlVTMB4XDTIxMDgyNTAyNTAzNFoXDTIzMDkyNDAyNTAzM1owgZIxQDA+BgNVBAMMN1Byb2QgRUNDIE1hYyBBcHAgU3RvcmUgYW5kIGlUdW5lcyBTdG9yZSBSZWNlaXB0IFNpZ25pbmcxLDAqBgNVBAsMI0FwcGxlIFdvcmxkd2lkZSBEZXZlbG9wZXIgUmVsYXRpb25zMRMwEQYDVQQKDApBcHBsZSBJbmMuMQswCQYDVQQGEwJVUzBZMBMGByqGSM49AgEGCCqGSM49AwEHA0IABOoTcaPcpeipNL9eQ06tCu7pUcwdCXdN8vGqaUjd58Z8tLxiUC0dBeA+euMYggh1/5iAk+FMxUFmA2a1r4aCZ8SjggIIMIICBDAMBgNVHRMBAf8EAjAAMB8GA1UdIwQYMBaAFD8vlCNR01DJmig97bB85c+lkGKZMHAGCCsGAQUFBwEBBGQwYjAtBggrBgEFBQcwAoYhaHR0cDovL2NlcnRzLmFwcGxlLmNvbS93d2RyZzYuZGVyMDEGCCsGAQUFBzABhiVodHRwOi8vb2NzcC5hcHBsZS5jb20vb2NzcDAzLXd3ZHJnNjAyMIIBHgYDVR0gBIIBFTCCAREwggENBgoqhkiG92NkBQYBMIH+MIHDBggrBgEFBQcCAjCBtgyBs1JlbGlhbmNlIG9uIHRoaXMgY2VydGlmaWNhdGUgYnkgYW55IHBhcnR5IGFzc3VtZXMgYWNjZXB0YW5jZSBvZiB0aGUgdGhlbiBhcHBsaWNhYmxlIHN0YW5kYXJkIHRlcm1zIGFuZCBjb25kaXRpb25zIG9mIHVzZSwgY2VydGlmaWNhdGUgcG9saWN5IGFuZCBjZXJ0aWZpY2F0aW9uIHByYWN0aWNlIHN0YXRlbWVudHMuMDYGCCsGAQUFBwIBFipodHRwOi8vd3d3LmFwcGxlLmNvbS9jZXJ0aWZpY2F0ZWF1dGhvcml0eS8wHQYDVR0OBBYEFCOCmMBq//1L5imvVmqX1oCYeqrMMA4GA1UdDwEB/wQEAwIHgDAQBgoqhkiG92NkBgsBBAIFADAKBggqhkjOPQQDAwNoADBlAjEAl4JB9GJHixP2nuibyU1k3wri5psGIxPME05sFKq7hQuzvbeyBu82FozzxmbzpogoAjBLSFl0dZWIYl2ejPV+Di5fBnKPu8mymBQtoE/H2bES0qAs8bNueU3CBjjh1lwnDsI=";
    const EFFECTIVE_DATE: u64 = 1681312846;

    #[test]
    fn test_valid_chain_without_ocsp() -> Result<(), ChainVerifierError> {
        let root = ROOT_CA_BASE64_ENCODED.as_der_bytes().unwrap();
        let leaf = LEAF_CERT_BASE64_ENCODED.as_der_bytes().unwrap();
        let intermediate = INTERMEDIATE_CA_BASE64_ENCODED.as_der_bytes().unwrap();
        let chain = vec![leaf.clone(), intermediate, root.clone()];

        let public_key = verify_chain(&chain, &vec![root],  Some(EFFECTIVE_DATE))?;
        assert_eq!(LEAF_CERT_PUBLIC_KEY_BASE64_ENCODED.as_der_bytes().unwrap(), public_key);
        Ok(())
    }

    #[test]
    fn test_valid_chain_invalid_intermediate_oid_without_ocsp() -> Result<(), ChainVerifierError> {
        let root = ROOT_CA_BASE64_ENCODED.as_der_bytes().unwrap();
        let leaf = LEAF_CERT_BASE64_ENCODED.as_der_bytes().unwrap();
        let intermediate = INTERMEDIATE_CA_INVALID_OID_BASE64_ENCODED.as_der_bytes().unwrap();
        let chain = vec![leaf.clone(), intermediate, root.clone()];

        let public_key = verify_chain(&chain, &vec![root],  Some(EFFECTIVE_DATE));
        assert_eq!(public_key.expect_err("Expect error"), ChainVerifierError::VerificationFailure(InvalidCertificate));
        Ok(())
    }

    #[test]
    fn test_valid_chain_invalid_leaf_oid_without_ocsp() -> Result<(), ChainVerifierError> {
        let root = ROOT_CA_BASE64_ENCODED.as_der_bytes().unwrap();
        let leaf = LEAF_CERT_INVALID_OID_BASE64_ENCODED.as_der_bytes().unwrap();
        let intermediate = INTERMEDIATE_CA_BASE64_ENCODED.as_der_bytes().unwrap();
        let chain = vec![leaf.clone(), intermediate, root.clone()];

        let public_key = verify_chain(&chain, &vec![root],  Some(EFFECTIVE_DATE));
        assert_eq!(public_key.expect_err("Expect error"), ChainVerifierError::VerificationFailure(InvalidCertificate));
        Ok(())
    }

    #[test]
    fn test_invalid_chain_length() -> Result<(), ChainVerifierError> {
        let root = ROOT_CA_BASE64_ENCODED.as_der_bytes().unwrap();
        let leaf = LEAF_CERT_BASE64_ENCODED.as_der_bytes().unwrap();
        let intermediate = INTERMEDIATE_CA_INVALID_OID_BASE64_ENCODED.as_der_bytes().unwrap();
        let chain = vec![leaf.clone(), intermediate];

        let public_key = verify_chain(&chain, &vec![root],  Some(EFFECTIVE_DATE));
        assert_eq!(public_key.expect_err("Expect error"), ChainVerifierError::VerificationFailure(InvalidChainLength));
        Ok(())
    }

    #[test]
    fn test_invalid_base64_in_certificate_list() -> Result<(), ChainVerifierError> {
        assert_eq!("abc".as_der_bytes().expect_err("Expect Error"), DecodeError::InvalidPadding);
        Ok(())
    }

    #[test]
    fn test_invalid_data_in_certificate_list() -> Result<(), ChainVerifierError> {
        let root = ROOT_CA_BASE64_ENCODED.as_der_bytes().unwrap();
        let leaf = STANDARD.encode("abc").as_der_bytes().unwrap();
        let intermediate = INTERMEDIATE_CA_BASE64_ENCODED.as_der_bytes().unwrap();
        let chain = vec![leaf.clone(), intermediate, root.clone()];

        let public_key = verify_chain(&chain, &vec![root],  Some(EFFECTIVE_DATE));
        assert_eq!(public_key.expect_err("Expect error"), ChainVerifierError::VerificationFailure(InvalidCertificate));
        Ok(())
    }

    #[test]
    fn test_malformed_root_cert() -> Result<(), ChainVerifierError> {
        let root = ROOT_CA_BASE64_ENCODED.as_der_bytes().unwrap();
        let malformed_root = STANDARD.encode("abc").as_der_bytes().unwrap();
        let leaf = LEAF_CERT_BASE64_ENCODED.as_der_bytes().unwrap();
        let intermediate = INTERMEDIATE_CA_BASE64_ENCODED.as_der_bytes().unwrap();
        let chain = vec![leaf.clone(), intermediate, root.clone()];

        let public_key = verify_chain(&chain, &vec![malformed_root],  Some(EFFECTIVE_DATE));
        assert_eq!(public_key.expect_err("Expect error"), ChainVerifierError::VerificationFailure(InvalidCertificate));
        Ok(())
    }

    #[test]
    fn test_chain_different_than_root_certificate() -> Result<(), ChainVerifierError> {
        let root = ROOT_CA_BASE64_ENCODED.as_der_bytes().unwrap();
        let real_root = REAL_APPLE_ROOT_BASE64_ENCODED.as_der_bytes().unwrap();
        let leaf = LEAF_CERT_BASE64_ENCODED.as_der_bytes().unwrap();
        let intermediate = INTERMEDIATE_CA_BASE64_ENCODED.as_der_bytes().unwrap();
        let chain = vec![leaf.clone(), intermediate, root.clone()];

        let public_key = verify_chain(&chain, &vec![real_root],  Some(EFFECTIVE_DATE));
        assert_eq!(public_key.expect_err("Expect error"), ChainVerifierError::InternalX509Error(SignatureVerificationError));
        Ok(())
    }

    #[test]
    fn test_valid_expired_chain() -> Result<(), ChainVerifierError> {
        let root = ROOT_CA_BASE64_ENCODED.as_der_bytes().unwrap();
        let leaf = LEAF_CERT_BASE64_ENCODED.as_der_bytes().unwrap();
        let intermediate = INTERMEDIATE_CA_BASE64_ENCODED.as_der_bytes().unwrap();
        let chain = vec![leaf.clone(), intermediate, root.clone()];

        let public_key = verify_chain(&chain, &vec![root],  Some(2280946846));
        assert_eq!(public_key.expect_err("Expect error"), ChainVerifierError::VerificationFailure(CertificateExpired));
        Ok(())
    }

    // #[test]
    // fn test_apple_chain_is_valid_with_ocsp_and_strict() -> Result<(), ChainVerifierError> {
    //     // Implement this test case.
    //     todo!();
    //     unimplemented!()
    // }
}

