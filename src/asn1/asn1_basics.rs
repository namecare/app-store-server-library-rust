/// ASN.1 decoding error
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ASN1Error {
    #[error("ASN1DecodeError: [{0}]")]
    ASN1DecodeError(String)
}

// ASN.1 Universal Tags
pub const TAG_INTEGER: u8 = 0x02;
pub const TAG_BIT_STRING: u8 = 0x03;
pub const TAG_OCTET_STRING: u8 = 0x04;
pub const TAG_OID: u8 = 0x06;
pub const TAG_UTF8_STRING: u8 = 0x0C;
pub const TAG_SEQUENCE: u8 = 0x30;
pub const TAG_SET: u8 = 0x31;
pub const TAG_CONTEXT_SPECIFIC_0: u8 = 0xA0;
pub const TAG_CONTEXT_SPECIFIC_CONSTRUCTED_4: u8 = 0x24;

/// Reads ASN.1 TLV (Tag-Length-Value) structure from data
/// 
/// Returns: (tag, length, next_offset)
pub fn read_tlv(data: &[u8], offset: usize) -> Result<(u8, usize, usize), ASN1Error> {
    if offset >= data.len() {
        return Err(ASN1Error::ASN1DecodeError("Unexpected end of data".to_string()));
    }

    let tag = data[offset];
    let mut current_offset = offset + 1;

    if current_offset >= data.len() {
        return Err(ASN1Error::ASN1DecodeError("Unexpected end of data".to_string()));
    }

    let first_length_byte = data[current_offset];
    current_offset += 1;

    let length = if first_length_byte == 0x80 {
        // Indefinite length
        usize::MAX
    } else if first_length_byte & 0x80 != 0 {
        // Long form
        let num_octets = (first_length_byte & 0x7F) as usize;
        if current_offset + num_octets > data.len() {
            return Err(ASN1Error::ASN1DecodeError("Invalid length encoding".to_string()));
        }
        let mut len = 0usize;
        for i in 0..num_octets {
            len = (len << 8) | (data[current_offset + i] as usize);
        }
        current_offset += num_octets;
        len
    } else {
        // Short form
        first_length_byte as usize
    };

    Ok((tag, length, current_offset))
}

/// Skips a TLV element and returns the offset after it
pub fn skip(data: &[u8], offset: usize) -> Result<usize, ASN1Error> {
    let (_, length, content_offset) = read_tlv(data, offset)?;
    if length == usize::MAX {
        // Indefinite length - find end-of-contents
        find_end_of_contents(data, content_offset)
    } else {
        Ok(content_offset + length)
    }
}

/// Reads a SEQUENCE and returns its content offset and length
pub fn read_sequence(data: &[u8], offset: usize) -> Result<(usize, usize), ASN1Error> {
    let (tag, length, content_offset) = read_tlv(data, offset)?;
    if tag != TAG_SEQUENCE {
        return Err(ASN1Error::ASN1DecodeError(format!("Expected SEQUENCE (0x30), got 0x{:02x}", tag)));
    }
    Ok((content_offset, length))
}

/// Reads a SET and returns its content offset and length
pub fn read_set(data: &[u8], offset: usize) -> Result<(usize, usize), ASN1Error> {
    let (tag, length, content_offset) = read_tlv(data, offset)?;
    if tag != TAG_SET {
        return Err(ASN1Error::ASN1DecodeError(format!("Expected SET (0x31), got 0x{:02x}", tag)));
    }
    Ok((content_offset, length))
}

/// Reads an OCTET STRING and returns its content offset and length
pub fn read_octet_string(data: &[u8], offset: usize) -> Result<(usize, usize), ASN1Error> {
    let (tag, length, content_offset) = read_tlv(data, offset)?;
    if tag != TAG_OCTET_STRING {
        return Err(ASN1Error::ASN1DecodeError(format!("Expected OCTET STRING (0x04), got 0x{:02x}", tag)));
    }
    Ok((content_offset, length))
}

/// Reads a BIT STRING and returns its content offset and length
pub fn read_bit_string(data: &[u8], offset: usize) -> Result<(usize, usize), ASN1Error> {
    let (tag, length, content_offset) = read_tlv(data, offset)?;
    if tag != TAG_BIT_STRING && tag != TAG_CONTEXT_SPECIFIC_CONSTRUCTED_4 {
        return Err(ASN1Error::ASN1DecodeError(format!("Expected BIT STRING (0x03 or 0x24), got 0x{:02x}", tag)));
    }
    Ok((content_offset, length))
}

/// Reads an OID and returns its content offset and length
pub fn read_oid(data: &[u8], offset: usize) -> Result<(usize, usize), ASN1Error> {
    let (tag, length, content_offset) = read_tlv(data, offset)?;
    if tag != TAG_OID {
        return Err(ASN1Error::ASN1DecodeError(format!("Expected OID (0x06), got 0x{:02x}", tag)));
    }
    Ok((content_offset, length))
}

/// Reads a context-specific [0] and returns its content offset and length
pub fn read_context_specific_0(data: &[u8], offset: usize) -> Result<(usize, usize), ASN1Error> {
    let (tag, length, content_offset) = read_tlv(data, offset)?;
    if tag != TAG_CONTEXT_SPECIFIC_0 {
        return Err(ASN1Error::ASN1DecodeError(format!("Expected context-specific [0] (0xA0), got 0x{:02x}", tag)));
    }
    Ok((content_offset, length))
}

/// Reads a UTF8String and returns the string
pub fn read_utf8_string(data: &[u8], offset: usize) -> Result<String, ASN1Error> {
    let (tag, length, content_offset) = read_tlv(data, offset)?;
    if tag != TAG_UTF8_STRING {
        return Err(ASN1Error::ASN1DecodeError(format!("Expected UTF8String (0x0C), got 0x{:02x}", tag)));
    }
    
    let utf8_bytes = &data[content_offset..content_offset + length];
    std::str::from_utf8(utf8_bytes)
        .map(|s| s.to_string())
        .map_err(|e| ASN1Error::ASN1DecodeError(format!("Invalid UTF-8: {}", e)))
}

/// Reads an ASN.1 INTEGER value as u64
pub fn read_integer(data: &[u8], offset: usize) -> Result<u64, ASN1Error> {
    let (tag, length, content_offset) = read_tlv(data, offset)?;
    if tag != TAG_INTEGER {
        return Err(ASN1Error::ASN1DecodeError("Expected INTEGER".to_string()));
    }

    if length == 0 {
        return Ok(0);
    }

    if length > 8 {
        return Err(ASN1Error::ASN1DecodeError("Integer too large for u64".to_string()));
    }

    let mut result = 0u64;
    for i in 0..length {
        result = (result << 8) | (data[content_offset + i] as u64);
    }

    Ok(result)
}

/// Gets the content of an element, handling indefinite length
pub fn get_content<'a>(data: &'a [u8], content_offset: usize, length: usize) -> Result<&'a [u8], ASN1Error> {
    if length == usize::MAX {
        // Indefinite length - find end-of-contents
        let end_offset = find_end_of_contents(data, content_offset)?;
        // The end_offset points to after the end-of-contents marker
        // We need to find where the actual content ends (before the 0x00 0x00)
        if end_offset >= 2 && data[end_offset - 2] == 0x00 && data[end_offset - 1] == 0x00 {
            Ok(&data[content_offset..end_offset - 2])
        } else {
            // Search backwards for the end-of-contents marker
            let mut actual_end = content_offset;
            while actual_end + 1 < data.len() {
                if data[actual_end] == 0x00 && data[actual_end + 1] == 0x00 {
                    break;
                }
                actual_end += 1;
            }
            Ok(&data[content_offset..actual_end])
        }
    } else {
        if content_offset + length > data.len() {
            return Err(ASN1Error::ASN1DecodeError("Data too short for specified length".to_string()));
        }
        Ok(&data[content_offset..content_offset + length])
    }
}

/// Finds the end-of-contents marker for indefinite length encoding
/// 
/// Returns the offset after the end-of-contents marker
pub fn find_end_of_contents(data: &[u8], start_offset: usize) -> Result<usize, ASN1Error> {
    let mut offset = start_offset;
    let mut depth = 1;

    while offset < data.len() && depth > 0 {
        let (tag, length, content_offset) = read_tlv(data, offset)?;

        if tag == 0x00 && length == 0 {
            // End-of-contents marker
            depth -= 1;
            if depth == 0 {
                return Ok(content_offset);
            }
            offset = content_offset;
        } else if length == usize::MAX {
            // Another indefinite length item
            depth += 1;
            offset = content_offset;
        } else {
            // Definite length item
            offset = content_offset + length;
        }
    }

    Err(ASN1Error::ASN1DecodeError("Missing end-of-contents marker".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_tlv_short_form() {
        let data = vec![0x02, 0x03, 0x01, 0x02, 0x03];
        let (tag, length, offset) = read_tlv(&data, 0).unwrap();
        assert_eq!(tag, 0x02);
        assert_eq!(length, 3);
        assert_eq!(offset, 2);
    }

    #[test]
    fn test_read_tlv_long_form_single_octet() {
        let data = vec![0x02, 0x81, 0x80, /* 128 bytes of content */];
        let (tag, length, offset) = read_tlv(&data, 0).unwrap();
        assert_eq!(tag, 0x02);
        assert_eq!(length, 0x80);
        assert_eq!(offset, 3);
    }

    #[test]
    fn test_read_tlv_long_form_two_octets() {
        let data = vec![0x02, 0x82, 0x01, 0x00, /* 256 bytes of content */];
        let (tag, length, offset) = read_tlv(&data, 0).unwrap();
        assert_eq!(tag, 0x02);
        assert_eq!(length, 0x0100);
        assert_eq!(offset, 4);
    }

    #[test]
    fn test_read_tlv_indefinite_length() {
        let data = vec![0x02, 0x80, /* content */];
        let (tag, length, offset) = read_tlv(&data, 0).unwrap();
        assert_eq!(tag, 0x02);
        assert_eq!(length, usize::MAX);
        assert_eq!(offset, 2);
    }

    #[test]
    fn test_read_tlv_offset_out_of_bounds() {
        let data = vec![0x02, 0x03, 0x01, 0x02, 0x03];
        let result = read_tlv(&data, 10);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ASN1Error::ASN1DecodeError("Unexpected end of data".to_string()));
    }

    #[test]
    fn test_read_tlv_incomplete_data() {
        let data = vec![0x02];
        let result = read_tlv(&data, 0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ASN1Error::ASN1DecodeError("Unexpected end of data".to_string()));
    }

    #[test]
    fn test_read_tlv_invalid_long_form() {
        let data = vec![0x02, 0x82, 0x01]; // Missing second length octet
        let result = read_tlv(&data, 0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ASN1Error::ASN1DecodeError("Invalid length encoding".to_string()));
    }

    #[test]
    fn test_read_integer_small_value() {
        let data = vec![0x02, 0x01, 0x05];
        let result = read_integer(&data, 0).unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn test_read_integer_zero_length() {
        let data = vec![0x02, 0x00];
        let result = read_integer(&data, 0).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_read_integer_multi_byte() {
        let data = vec![0x02, 0x02, 0x01, 0x00];
        let result = read_integer(&data, 0).unwrap();
        assert_eq!(result, 256);
    }

    #[test]
    fn test_read_integer_max_u64_bytes() {
        let data = vec![0x02, 0x08, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        let result = read_integer(&data, 0).unwrap();
        assert_eq!(result, u64::MAX);
    }

    #[test]
    fn test_read_integer_wrong_tag() {
        let data = vec![0x03, 0x01, 0x05]; // Wrong tag (BIT STRING instead of INTEGER)
        let result = read_integer(&data, 0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ASN1Error::ASN1DecodeError("Expected INTEGER".to_string()));
    }

    #[test]
    fn test_read_integer_too_large() {
        let data = vec![0x02, 0x09, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let result = read_integer(&data, 0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ASN1Error::ASN1DecodeError("Integer too large for u64".to_string()));
    }

    #[test]
    fn test_find_end_of_contents_simple() {
        let data = vec![
            0x30, 0x80, // SEQUENCE with indefinite length
            0x02, 0x01, 0x05, // INTEGER 5
            0x00, 0x00, // End-of-contents
        ];
        let result = find_end_of_contents(&data, 2).unwrap();
        assert_eq!(result, 7);
    }

    #[test]
    fn test_find_end_of_contents_nested() {
        let data = vec![
            0x30, 0x80, // Outer SEQUENCE with indefinite length
            0x30, 0x80, // Inner SEQUENCE with indefinite length
            0x02, 0x01, 0x05, // INTEGER 5
            0x00, 0x00, // End-of-contents for inner
            0x00, 0x00, // End-of-contents for outer
        ];
        let result = find_end_of_contents(&data, 2).unwrap();
        assert_eq!(result, 11);
    }

    #[test]
    fn test_find_end_of_contents_mixed_lengths() {
        let data = vec![
            0x30, 0x80, // SEQUENCE with indefinite length
            0x02, 0x01, 0x05, // INTEGER 5 (definite length)
            0x30, 0x03, 0x02, 0x01, 0x06, // SEQUENCE with definite length containing INTEGER 6
            0x00, 0x00, // End-of-contents
        ];
        let result = find_end_of_contents(&data, 2).unwrap();
        assert_eq!(result, 12);
    }

    #[test]
    fn test_find_end_of_contents_missing_marker() {
        let data = vec![
            0x30, 0x80, // SEQUENCE with indefinite length
            0x02, 0x01, 0x05, // INTEGER 5
            // Missing end-of-contents marker
        ];
        let result = find_end_of_contents(&data, 2);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ASN1Error::ASN1DecodeError("Missing end-of-contents marker".to_string()));
    }

    #[test]
    fn test_asn1_error_display() {
        let error = ASN1Error::ASN1DecodeError("Test error".to_string());
        assert_eq!(error.to_string(), "ASN1DecodeError: [Test error]");
    }

    #[test]
    fn test_skip() {
        let data = vec![0x02, 0x03, 0x01, 0x02, 0x03, 0x04, 0x01, 0x05];
        let next_offset = skip(&data, 0).unwrap();
        assert_eq!(next_offset, 5);
        
        // Verify we can read the next element
        let (tag, length, _) = read_tlv(&data, next_offset).unwrap();
        assert_eq!(tag, 0x04);
        assert_eq!(length, 1);
    }

    #[test]
    fn test_skip_indefinite_length() {
        let data = vec![
            0x30, 0x80, // SEQUENCE with indefinite length
            0x02, 0x01, 0x05, // INTEGER 5
            0x00, 0x00, // End-of-contents
            0x04, 0x01, 0x06, // OCTET STRING
        ];
        let next_offset = skip(&data, 0).unwrap();
        assert_eq!(next_offset, 7);
        
        // Verify we can read the next element
        let (tag, _, _) = read_tlv(&data, next_offset).unwrap();
        assert_eq!(tag, 0x04);
    }

    #[test]
    fn test_read_sequence() {
        let data = vec![0x30, 0x03, 0x02, 0x01, 0x05];
        let (content_offset, length) = read_sequence(&data, 0).unwrap();
        assert_eq!(content_offset, 2);
        assert_eq!(length, 3);
    }

    #[test]
    fn test_read_sequence_wrong_tag() {
        let data = vec![0x31, 0x03, 0x02, 0x01, 0x05]; // SET instead of SEQUENCE
        let result = read_sequence(&data, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_set() {
        let data = vec![0x31, 0x03, 0x02, 0x01, 0x05];
        let (content_offset, length) = read_set(&data, 0).unwrap();
        assert_eq!(content_offset, 2);
        assert_eq!(length, 3);
    }

    #[test]
    fn test_read_octet_string() {
        let data = vec![0x04, 0x05, 0x48, 0x65, 0x6c, 0x6c, 0x6f]; // "Hello"
        let (content_offset, length) = read_octet_string(&data, 0).unwrap();
        assert_eq!(content_offset, 2);
        assert_eq!(length, 5);
    }

    #[test]
    fn test_read_bit_string() {
        let data = vec![0x03, 0x02, 0x00, 0xFF];
        let (content_offset, length) = read_bit_string(&data, 0).unwrap();
        assert_eq!(content_offset, 2);
        assert_eq!(length, 2);
    }

    #[test]
    fn test_read_bit_string_context_specific() {
        let data = vec![0x24, 0x80]; // Context-specific constructed 4 with indefinite length
        let (content_offset, length) = read_bit_string(&data, 0).unwrap();
        assert_eq!(content_offset, 2);
        assert_eq!(length, usize::MAX);
    }

    #[test]
    fn test_read_oid() {
        let data = vec![0x06, 0x09, 0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x07, 0x02];
        let (content_offset, length) = read_oid(&data, 0).unwrap();
        assert_eq!(content_offset, 2);
        assert_eq!(length, 9);
    }

    #[test]
    fn test_read_context_specific_0() {
        let data = vec![0xA0, 0x03, 0x02, 0x01, 0x05];
        let (content_offset, length) = read_context_specific_0(&data, 0).unwrap();
        assert_eq!(content_offset, 2);
        assert_eq!(length, 3);
    }

    #[test]
    fn test_read_utf8_string() {
        let data = vec![0x0C, 0x05, 0x48, 0x65, 0x6c, 0x6c, 0x6f]; // "Hello"
        let result = read_utf8_string(&data, 0).unwrap();
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_read_utf8_string_invalid_utf8() {
        let data = vec![0x0C, 0x02, 0xFF, 0xFE]; // Invalid UTF-8
        let result = read_utf8_string(&data, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_content_definite_length() {
        let data = vec![0x04, 0x05, 0x48, 0x65, 0x6c, 0x6c, 0x6f]; // OCTET STRING "Hello"
        let (_, length, content_offset) = read_tlv(&data, 0).unwrap();
        let content = get_content(&data, content_offset, length).unwrap();
        assert_eq!(content, b"Hello");
    }

    #[test]
    fn test_get_content_indefinite_length() {
        let data = vec![
            0x24, 0x80, // BIT STRING with indefinite length
            0x04, 0x05, 0x48, 0x65, 0x6c, 0x6c, 0x6f, // OCTET STRING "Hello"
            0x00, 0x00, // End-of-contents
        ];
        let (_, length, content_offset) = read_tlv(&data, 0).unwrap();
        let content = get_content(&data, content_offset, length).unwrap();
        assert_eq!(content, &data[2..9]);
    }
}