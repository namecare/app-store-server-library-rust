use crate::chain_verifier::ChainVerificationFailureReason::InvalidCertificate;
use crate::chain_verifier::{ChainVerificationFailureReason, ChainVerifier, ChainVerifierError};
use x509_cert::Certificate;

/// Internal error type for OCSP validation that helps distinguish retryable errors
#[derive(Debug)]
enum OcspError {
    /// Network-related error (connection failure, timeout, etc.)
    NetworkError(String),
    /// HTTP error with non-200 status code
    HttpError(u16),
    /// Failed to read response body
    FetchFailed,
    /// Certificate has been revoked
    CertificateRevoked,
    /// Other validation errors (parsing, certificate issues, etc.)
    ValidationError,
}

impl ChainVerifier {
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
    pub fn check_ocsp_status(&self, leaf: &Certificate, issuer: &Certificate) -> Result<(), ChainVerifierError> {
        match self.check_ocsp_status_internal(leaf, issuer) {
            Ok(()) => Ok(()),
            Err(OcspError::NetworkError(_)) | Err(OcspError::HttpError(_)) | Err(OcspError::FetchFailed) => {
                // Network-related errors are retryable
                Err(ChainVerifierError::VerificationFailure(
                    ChainVerificationFailureReason::RetryableVerificationFailure,
                ))
            }
            Err(OcspError::CertificateRevoked) => {
                // Certificate is revoked - this should fail immediately
                Err(ChainVerifierError::VerificationFailure(
                    ChainVerificationFailureReason::CertificateRevoked,
                ))
            }
            Err(OcspError::ValidationError) => {
                // Other errors are not retryable
                Err(ChainVerifierError::VerificationFailure(InvalidCertificate))
            }
        }
    }

    fn check_ocsp_status_internal(&self, leaf: &Certificate, issuer: &Certificate) -> Result<(), OcspError> {
        use der::asn1::ObjectIdentifier;
        use der::{asn1::OctetString, Decode, Encode};
        use x509_cert::spki::AlgorithmIdentifier;
        use x509_ocsp::{BasicOcspResponse, CertId, CertStatus, OcspRequest, OcspResponse, Request, TbsRequest};

        let ocsp_url = self.extract_ocsp_url(leaf).map_err(|_| OcspError::ValidationError)?;

        // Hash the issuer's distinguished name using SHA-1
        let issuer_name_hash_bytes = {
            use sha1::{Sha1, Digest};
            // Get the raw DER encoding of the issuer's distinguished name
            let issuer_name = issuer.tbs_certificate.subject.to_der()
                .map_err(|_| OcspError::ValidationError)?;
            let hash = Sha1::digest(&issuer_name);
            hash.to_vec()
        };

        // Extract and use the issuer's Subject Key Identifier as the key hash
        let issuer_key_data = self.extract_ski(&issuer).map_err(|_| OcspError::ValidationError)?;

        // SHA-1 OID: 1.3.14.3.2.26
        let sha1_oid = ObjectIdentifier::new_unwrap("1.3.14.3.2.26");

        // Convert to owned types for x509-ocsp compatibility
        let hash_algorithm = AlgorithmIdentifier {
            oid: sha1_oid,
            parameters: None,
        };

        let serial_bytes = leaf.tbs_certificate.serial_number.as_bytes();

        let issuer_name_hash = OctetString::new(issuer_name_hash_bytes)
            .map_err(|_| OcspError::ValidationError)?;
        let issuer_key_hash = OctetString::new(issuer_key_data)
            .map_err(|_| OcspError::ValidationError)?;

        // Use the SerialNumber from x509-cert
        use x509_cert::serial_number::SerialNumber;
        let serial_number = SerialNumber::new(serial_bytes)
            .map_err(|_| OcspError::ValidationError)?;

        let cert_id = CertId {
            hash_algorithm,
            issuer_name_hash,
            issuer_key_hash,
            serial_number,
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

        // Encode the OCSP request
        let request_bytes = ocsp_request
            .to_der()
            .map_err(|_| OcspError::ValidationError)?;

        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .map_err(|e| OcspError::NetworkError(format!("Failed to build HTTP client: {}", e)))?;

        let response = client
            .post(&ocsp_url)
            .header("Content-Type", "application/ocsp-request")
            .body(request_bytes)
            .send()
            .map_err(|e| {
                // reqwest errors can be network-related (timeout, connection failure, etc.)
                OcspError::NetworkError(format!("OCSP request failed: {}", e))
            })?;

        // Check HTTP status code
        let status = response.status();
        if !status.is_success() {
            return Err(OcspError::HttpError(status.as_u16()));
        }

        let response_bytes = response
            .bytes()
            .map_err(|_| OcspError::FetchFailed)?;

        let ocsp_response = OcspResponse::from_der(&response_bytes)
            .map_err(|_| OcspError::ValidationError)?;

        use x509_ocsp::OcspResponseStatus;
        match ocsp_response.response_status {
            OcspResponseStatus::Successful => {} // Continue processing
            _ => return Err(OcspError::ValidationError),
        }

        let response_bytes = ocsp_response
            .response_bytes
            .ok_or_else(|| OcspError::ValidationError)?;

        const ID_PKIX_OCSP_BASIC: &str = "1.3.6.1.5.5.7.48.1.1";
        if response_bytes.response_type.to_string() != ID_PKIX_OCSP_BASIC {
            return Err(OcspError::ValidationError);
        }

        let basic_response = BasicOcspResponse::from_der(response_bytes.response.as_bytes())
            .map_err(|_| OcspError::ValidationError)?;

        for single_response in &basic_response.tbs_response_data.responses {
            // TODO: Verify the CertId matches our request to ensure this response is for our certificate
            match &single_response.cert_status {
                CertStatus::Good(_) => return Ok(()), // Certificate is valid
                CertStatus::Revoked(_) => {
                    // Certificate has been revoked
                    return Err(OcspError::CertificateRevoked);
                }
                CertStatus::Unknown(_) => {
                    // Certificate status unknown - treat as validation error
                    return Err(OcspError::ValidationError);
                }
            }
        }

        Err(OcspError::ValidationError)
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
    fn extract_ski(&self, issuer: &Certificate) -> Result<Vec<u8>, ChainVerifierError> {
        use const_oid::ObjectIdentifier;
        use der::Decode;

        // Subject Key Identifier OID: 2.5.29.14
        let ski_oid = ObjectIdentifier::new_unwrap("2.5.29.14");

        let Some(extensions) = &issuer.tbs_certificate.extensions else {
            return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
        };

        for ext in extensions {
            if ext.extn_id == ski_oid {
                // The extension value is an OCTET STRING containing the key identifier
                let octet_string = der::asn1::OctetString::from_der(ext.extn_value.as_bytes())
                    .map_err(|_| ChainVerifierError::VerificationFailure(InvalidCertificate))?;
                return Ok(octet_string.as_bytes().to_vec());
            }
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
    fn extract_ocsp_url(&self, cert: &Certificate) -> Result<String, ChainVerifierError> {
        use const_oid::ObjectIdentifier;

        // AIA extension OID: 1.3.6.1.5.5.7.1.1
        let aia_oid = ObjectIdentifier::new_unwrap("1.3.6.1.5.5.7.1.1");

        let Some(extensions) = &cert.tbs_certificate.extensions else {
            return Err(ChainVerifierError::VerificationFailure(InvalidCertificate));
        };

        // OCSP OID: 1.3.6.1.5.5.7.48.1
        let ocsp_oid = ObjectIdentifier::new_unwrap("1.3.6.1.5.5.7.48.1");

        for ext in extensions {
            if ext.extn_id == aia_oid {
                // Try to extract OCSP URL from the extension
                // This is a simplified parser - in production, you'd want more robust parsing
                if let Ok(url) = self.parse_aia_for_ocsp(ext.extn_value.as_bytes(), &ocsp_oid) {
                    return Ok(url);
                }
            }
        }

        Err(ChainVerifierError::VerificationFailure(InvalidCertificate))
    }

    /// Helper function to parse AIA extension and extract OCSP URL
    fn parse_aia_for_ocsp(&self, aia_bytes: &[u8], ocsp_oid: &const_oid::ObjectIdentifier) -> Result<String, ChainVerifierError> {
        use crate::asn1::asn1_basics::{read_sequence, read_oid, read_tlv};

        // AIA is a SEQUENCE of AccessDescription
        // Each AccessDescription is a SEQUENCE of { accessMethod OID, accessLocation GeneralName }
        // GeneralName for URI is [6] IMPLICIT IA5String

        let (mut offset, length) = read_sequence(aia_bytes, 0)
            .map_err(|e| ChainVerifierError::InternalX509Error(e.to_string()))?;

        let end_offset = offset + length;

        while offset < end_offset {
            // Read AccessDescription SEQUENCE
            let (desc_offset, desc_length) = read_sequence(aia_bytes, offset)
                .map_err(|e| ChainVerifierError::InternalX509Error(e.to_string()))?;

            let desc_end = desc_offset + desc_length;

            // Read accessMethod OID
            let (oid_offset, oid_length) = read_oid(aia_bytes, desc_offset)
                .map_err(|e| ChainVerifierError::InternalX509Error(e.to_string()))?;

            // Check if this is the OCSP OID
            let oid_bytes = &aia_bytes[oid_offset..oid_offset + oid_length];

            // OCSP OID bytes: 1.3.6.1.5.5.7.48.1 = 2B 06 01 05 05 07 30 01
            let expected_ocsp_oid = [0x2B, 0x06, 0x01, 0x05, 0x05, 0x07, 0x30, 0x01];

            if oid_bytes == expected_ocsp_oid {
                // Read the accessLocation - should be [6] IMPLICIT IA5String (URI)
                let location_offset = oid_offset + oid_length;
                let (tag, uri_length, uri_offset) = read_tlv(aia_bytes, location_offset)
                    .map_err(|e| ChainVerifierError::InternalX509Error(e.to_string()))?;

                // Tag [6] for uniformResourceIdentifier is 0x86
                if tag == 0x86 {
                    let uri_bytes = &aia_bytes[uri_offset..uri_offset + uri_length];
                    let uri = std::str::from_utf8(uri_bytes)
                        .map_err(|_| ChainVerifierError::VerificationFailure(InvalidCertificate))?;
                    return Ok(uri.to_string());
                }
            }

            // Move to next AccessDescription
            offset = desc_end;
        }

        Err(ChainVerifierError::VerificationFailure(InvalidCertificate))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::x509::x509;

    #[test]
    fn test_extract_ocsp_url_missing_aia() {
        // Create a minimal certificate without AIA extension
        let cert_der = include_bytes!("../tests/resources/certs/testCA.der");
        let cert = x509::parse_certificate(cert_der).unwrap();

        let verifier = ChainVerifier::new(vec![]);
        let result = verifier.extract_ocsp_url(&cert);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_ocsp_url_with_aia() {
        // This test would need a certificate with an AIA extension
        // For now, we'll test the error case
        let cert_der = include_bytes!("../tests/resources/certs/testCA.der");
        let cert = x509::parse_certificate(cert_der).unwrap();

        let verifier = ChainVerifier::new(vec![]);
        let result = verifier.extract_ocsp_url(&cert);
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
        assert_eq!(
            cert_id
                .issuer_name_hash
                .as_bytes()
                .len(),
            20
        );
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
