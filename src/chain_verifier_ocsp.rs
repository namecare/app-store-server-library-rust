//! OCSP (Online Certificate Status Protocol) verification module.
//!
//! This module provides functionality to verify certificate revocation status using OCSP.
//! It's only compiled when the `ocsp` feature is enabled.

#![cfg(feature = "ocsp")]

use crate::chain_verifier::ChainVerificationFailureReason::InvalidCertificate;
use crate::chain_verifier::{ChainVerificationFailureReason, ChainVerifierError};
use x509_parser::certificate::X509Certificate;
use x509_parser::extensions::{GeneralName, ParsedExtension};
use x509_parser::oid_registry::Oid;

/// Checks the OCSP (Online Certificate Status Protocol) revocation status of a certificate.
///
/// This function performs a real-time check to verify if a certificate has been revoked
/// by contacting the OCSP responder specified in the certificate's Authority Information Access extension.
///
/// # Arguments
///
/// * `leaf` - The certificate to check for revocation status
/// * `issuer` - The issuer certificate that signed the leaf certificate
///
/// # Returns
///
/// * `Ok(())` - If the certificate is valid and not revoked
/// * `Err(ChainVerifierError)` - If the certificate is revoked, status unknown, or an error occurred
///
/// # Errors
///
/// This function will return an error if:
/// - The certificate doesn't contain an OCSP responder URL in its AIA extension
/// - The OCSP request cannot be created or sent
/// - The OCSP responder returns a non-success status
/// - The certificate is marked as revoked in the OCSP response
/// - The certificate status is unknown
/// - Network timeout occurs (5-second timeout is enforced)
///
/// # Example
///
/// ```ignore
/// let leaf_cert = X509Certificate::from_der(&leaf_der)?;
/// let issuer_cert = X509Certificate::from_der(&issuer_der)?;
///
/// match check_ocsp_status(&leaf_cert, &issuer_cert) {
///     Ok(()) => println!("Certificate is valid"),
///     Err(e) => println!("Certificate validation failed: {:?}", e),
/// }
/// ```
pub fn check_ocsp_status(leaf: &X509Certificate<'_>, issuer: &X509Certificate<'_>) -> Result<(), ChainVerifierError> {
    use der::asn1::ObjectIdentifier;
    use der::{asn1::OctetString, Decode, Encode};
    use x509_cert::spki::AlgorithmIdentifierOwned;
    use x509_ocsp::{BasicOcspResponse, CertId, CertStatus, OcspRequest, OcspResponse, Request, TbsRequest};

    let ocsp_url = extract_ocsp_url(leaf)?;

    // Hash the issuer's distinguished name using SHA-1
    let issuer_name_hash = {
        use ring::digest;
        // Get the raw DER encoding of the issuer's distinguished name
        let issuer_name = issuer.subject().as_raw();
        let hash = digest::digest(&digest::SHA1_FOR_LEGACY_USE_ONLY, issuer_name);
        OctetString::new(hash.as_ref()).map_err(|_| ChainVerifierError::VerificationFailure(InvalidCertificate))?
    };

    // Extract and use the issuer's Subject Key Identifier as the key hash
    let issuer_key_data = extract_ski(&issuer)?;
    let issuer_key_hash = OctetString::new(issuer_key_data).unwrap();

    // SHA-1 OID: 1.3.14.3.2.26
    let sha1_oid = ObjectIdentifier::new_unwrap("1.3.14.3.2.26");
    let hash_algorithm = AlgorithmIdentifierOwned {
        oid: sha1_oid,
        parameters: None,
    };

    use x509_cert::serial_number::SerialNumber;
    let serial = SerialNumber::new(&leaf.serial.to_bytes_be())
        .map_err(|_| ChainVerifierError::VerificationFailure(InvalidCertificate))?;

    let cert_id = CertId {
        hash_algorithm,
        issuer_name_hash,
        issuer_key_hash,
        serial_number: serial,
    };

    let request = Request {
        req_cert: cert_id,
        single_request_extensions: None,
    };

    let tbs_request = TbsRequest {
        version: x509_ocsp::Version::V1,
        requestor_name: None,
        request_list: vec![request],
        request_extensions: None,
    };

    let ocsp_request = OcspRequest {
        tbs_request,
        optional_signature: None,
    };

    let request_bytes = ocsp_request
        .to_der()
        .map_err(|_| ChainVerifierError::VerificationFailure(InvalidCertificate))?;

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|_| ChainVerifierError::VerificationFailure(InvalidCertificate))?;

    let response = client
        .post(&ocsp_url)
        .header("Content-Type", "application/ocsp-request")
        .body(request_bytes)
        .send()
        .map_err(|_| ChainVerifierError::VerificationFailure(InvalidCertificate))?;

    if !response.status().is_success() {
        return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
    }

    let response_bytes = response
        .bytes()
        .map_err(|_| ChainVerifierError::VerificationFailure(InvalidCertificate))?;

    let ocsp_response = OcspResponse::from_der(&response_bytes)
        .map_err(|_| ChainVerifierError::VerificationFailure(InvalidCertificate))?;

    use x509_ocsp::OcspResponseStatus;
    match ocsp_response.response_status {
        OcspResponseStatus::Successful => {} // Continue processing
        _ => return Err(ChainVerifierError::VerificationFailure(InvalidCertificate)),
    }

    let response_bytes = ocsp_response
        .response_bytes
        .ok_or(ChainVerifierError::VerificationFailure(InvalidCertificate))?;

    const ID_PKIX_OCSP_BASIC: &str = "1.3.6.1.5.5.7.48.1.1";
    if response_bytes.response_type.to_string() != ID_PKIX_OCSP_BASIC {
        return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
    }

    let basic_response = BasicOcspResponse::from_der(response_bytes.response.as_bytes())
        .map_err(|_| ChainVerifierError::VerificationFailure(InvalidCertificate))?;

    for single_response in &basic_response.tbs_response_data.responses {
        // TODO: Verify the CertId matches our request to ensure this response is for our certificate
        match &single_response.cert_status {
            CertStatus::Good(_) => return Ok(()), // Certificate is valid
            CertStatus::Revoked(_) => {
                // Certificate has been revoked
                return Err(ChainVerifierError::VerificationFailure(
                    ChainVerificationFailureReason::CertificateRevoked,
                ));
            }
            CertStatus::Unknown(_) => {
                // Certificate status unknown - treat as error
                return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
            }
        }
    }

    Err(ChainVerifierError::VerificationFailure(InvalidCertificate))
}

/// Extracts the Subject Key Identifier (SKI) from an issuer certificate.
///
/// # Arguments
///
/// * `issuer` - The issuer certificate from which to extract the SKI
///
/// # Returns
///
/// * `Ok(Vec<u8>)` - The raw bytes of the Subject Key Identifier
/// * `Err(ChainVerifierError)` - If the SKI extension is not present or cannot be parsed
///
/// # Details
///
/// The Subject Key Identifier extension (OID: 2.5.29.14) contains a key identifier
/// derived from the public key. This function extracts and returns the raw identifier bytes.
fn extract_ski(issuer: &X509Certificate<'_>) -> Result<Vec<u8>, ChainVerifierError> {
    // Subject Key Identifier OID: 2.5.29.14
    let ski_oid = Oid::from(&[2, 5, 29, 14]).unwrap();
    let ski_ext = issuer
        .get_extension_unique(&ski_oid)
        .ok()
        .flatten()
        .ok_or(ChainVerifierError::VerificationFailure(InvalidCertificate))?;

    if let ParsedExtension::SubjectKeyIdentifier(ski) = ski_ext.parsed_extension() {
        return Ok(ski.0.to_vec());
    }

    Err(ChainVerifierError::VerificationFailure(InvalidCertificate))
}

/// Extracts the OCSP responder URL from a certificate's Authority Information Access extension.
///
/// # Arguments
///
/// * `cert` - The certificate from which to extract the OCSP URL
///
/// # Returns
///
/// * `Ok(String)` - The OCSP responder URL
/// * `Err(ChainVerifierError)` - If the AIA extension is missing or doesn't contain an OCSP URL
///
/// # Details
///
/// This function looks for the Authority Information Access (AIA) extension (OID: 1.3.6.1.5.5.7.1.1)
/// and searches for an access descriptor with the OCSP access method (OID: 1.3.6.1.5.5.7.48.1).
/// The first OCSP URL found is returned.
fn extract_ocsp_url(cert: &X509Certificate<'_>) -> Result<String, ChainVerifierError> {
    // AIA extension OID: 1.3.6.1.5.5.7.1.1
    let aia_oid = Oid::from(&[1, 3, 6, 1, 5, 5, 7, 1, 1]).unwrap();
    let aia_ext = cert
        .get_extension_unique(&aia_oid)
        .ok()
        .flatten()
        .ok_or(ChainVerifierError::VerificationFailure(InvalidCertificate))?;

    // Parse AIA extension to find OCSP URL
    if let ParsedExtension::AuthorityInfoAccess(aia) = aia_ext.parsed_extension() {
        // OCSP OID: 1.3.6.1.5.5.7.48.1
        let ocsp_oid = Oid::from(&[1, 3, 6, 1, 5, 5, 7, 48, 1]).unwrap();
        for access_desc in &aia.accessdescs {
            if access_desc.access_method == ocsp_oid {
                if let GeneralName::URI(uri) = &access_desc.access_location {
                    return Ok(uri.to_string());
                }
            }
        }
    }

    Err(ChainVerifierError::VerificationFailure(InvalidCertificate))
}

#[cfg(test)]
mod tests {
    use super::*;
    use x509_parser::prelude::*;

    #[test]
    fn test_extract_ocsp_url_missing_aia() {
        // Create a minimal certificate without AIA extension
        let cert_der = include_bytes!("../resources/certs/testCA.der");
        let cert = X509Certificate::from_der(cert_der).unwrap().1;

        let result = extract_ocsp_url(&cert);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_ocsp_url_with_aia() {
        // This test would need a certificate with an AIA extension
        // For now, we'll test the error case
        let cert_der = include_bytes!("../resources/certs/testCA.der");
        let cert = X509Certificate::from_der(cert_der).unwrap().1;

        let result = extract_ocsp_url(&cert);
        // Most test certificates don't have OCSP URLs
        assert!(result.is_err());
    }

    #[test]
    fn test_ocsp_response_parsing() {
        use der::{Decode, Encode};
        use x509_ocsp::{OcspResponse, OcspResponseStatus};

        // Create a minimal OCSP response for testing
        let response = OcspResponse {
            response_status: OcspResponseStatus::Successful,
            response_bytes: None,
        };

        let encoded = response.to_der().unwrap();
        let decoded = OcspResponse::from_der(&encoded).unwrap();

        assert_eq!(decoded.response_status, OcspResponseStatus::Successful);
    }

    #[test]
    fn test_cert_id_creation() {
        use der::asn1::{ObjectIdentifier, OctetString};
        use x509_cert::serial_number::SerialNumber;
        use x509_cert::spki::AlgorithmIdentifierOwned;
        use x509_ocsp::CertId;

        // SHA-1 OID
        let sha1_oid = ObjectIdentifier::new_unwrap("1.3.14.3.2.26");
        let hash_algorithm = AlgorithmIdentifierOwned {
            oid: sha1_oid,
            parameters: None,
        };

        // SHA-1 produces 20-byte hashes
        let issuer_name_hash = OctetString::new(&[0u8; 20]).unwrap();
        let issuer_key_hash = OctetString::new(&[0u8; 20]).unwrap();
        let serial = SerialNumber::new(&[1, 2, 3]).unwrap();

        let cert_id = CertId {
            hash_algorithm,
            issuer_name_hash,
            issuer_key_hash,
            serial_number: serial,
        };

        // Basic sanity check for SHA-1 hash length
        assert_eq!(cert_id.issuer_name_hash.as_bytes().len(), 20);
        assert_eq!(cert_id.issuer_key_hash.as_bytes().len(), 20);
    }

    #[test]
    fn test_ocsp_request_creation() {
        use der::asn1::{ObjectIdentifier, OctetString};
        use der::Encode;
        use x509_cert::serial_number::SerialNumber;
        use x509_cert::spki::AlgorithmIdentifierOwned;
        use x509_ocsp::{CertId, OcspRequest, Request, TbsRequest};

        let sha1_oid = ObjectIdentifier::new_unwrap("1.3.14.3.2.26");
        let hash_algorithm = AlgorithmIdentifierOwned {
            oid: sha1_oid,
            parameters: None,
        };

        let cert_id = CertId {
            hash_algorithm,
            issuer_name_hash: OctetString::new(&[0u8; 20]).unwrap(),
            issuer_key_hash: OctetString::new(&[0u8; 20]).unwrap(),
            serial_number: SerialNumber::new(&[1]).unwrap(),
        };

        let request = Request {
            req_cert: cert_id,
            single_request_extensions: None,
        };

        let tbs_request = TbsRequest {
            version: x509_ocsp::Version::V1,
            requestor_name: None,
            request_list: vec![request],
            request_extensions: None,
        };

        let ocsp_request = OcspRequest {
            tbs_request,
            optional_signature: None,
        };

        // Test that we can encode the request
        let encoded = ocsp_request.to_der();
        assert!(encoded.is_ok());
    }
}
