use std::fmt;
use uuid::Uuid;

/// Validation errors for Advanced Commerce API
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    InvalidCurrencyLength(usize),
    InvalidCurrencyFormat(String),
    EmptyTaxCode,
    EmptyTransactionId,
    EmptyTargetProductId,
    UuidTooLong(usize),
    NegativePrice(i64),
    DescriptionTooLong(usize),
    DisplayNameTooLong(usize),
    SkuTooLong(usize),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::InvalidCurrencyLength(len) => {
                write!(f, "Currency must be a 3-letter ISO 4217 code, got {} characters", len)
            }
            ValidationError::InvalidCurrencyFormat(currency) => {
                write!(f, "Currency must contain only uppercase letters: {}", currency)
            }
            ValidationError::EmptyTaxCode => write!(f, "Tax code cannot be empty"),
            ValidationError::EmptyTransactionId => write!(f, "Transaction ID cannot be empty"),
            ValidationError::EmptyTargetProductId => write!(f, "Target Product ID cannot be empty"),
            ValidationError::UuidTooLong(len) => {
                write!(f, "UUID string representation cannot exceed {} characters, got {}", 
                    MAXIMUM_REQUEST_REFERENCE_ID_LENGTH, len)
            }
            ValidationError::NegativePrice(price) => {
                write!(f, "Price cannot be negative: {}", price)
            }
            ValidationError::DescriptionTooLong(len) => {
                write!(f, "Description length ({}) exceeds maximum allowed ({})", 
                    len, MAXIMUM_DESCRIPTION_LENGTH)
            }
            ValidationError::DisplayNameTooLong(len) => {
                write!(f, "Display name length ({}) exceeds maximum allowed ({})", 
                    len, MAXIMUM_DISPLAY_NAME_LENGTH)
            }
            ValidationError::SkuTooLong(len) => {
                write!(f, "SKU length ({}) exceeds maximum allowed ({})", 
                    len, MAXIMUM_SKU_LENGTH)
            }
        }
    }
}

impl std::error::Error for ValidationError {}

/// Validation constants
pub const CURRENCY_CODE_LENGTH: usize = 3;
pub const MAXIMUM_STOREFRONT_LENGTH: usize = 10;
pub const MAXIMUM_REQUEST_REFERENCE_ID_LENGTH: usize = 36;
pub const MAXIMUM_DESCRIPTION_LENGTH: usize = 45;
pub const MAXIMUM_DISPLAY_NAME_LENGTH: usize = 30;
const MAXIMUM_SKU_LENGTH: usize = 128;

/// Validates currency code according to ISO 4217 standard.
/// 
/// # Arguments
/// * `currency` - The currency code to validate
/// 
/// # Returns
/// * `Ok(String)` - The validated currency code
/// * `Err(ValidationError)` - If validation fails
pub fn validate_currency(currency: &str) -> Result<String, ValidationError> {
    if currency.len() != CURRENCY_CODE_LENGTH {
        return Err(ValidationError::InvalidCurrencyLength(currency.len()));
    }
    
    if !currency.chars().all(|c| c.is_ascii_uppercase()) {
        return Err(ValidationError::InvalidCurrencyFormat(currency.to_string()));
    }
    
    Ok(currency.to_string())
}

/// Validates tax code is not empty.
/// 
/// # Arguments
/// * `tax_code` - The tax code to validate
/// 
/// # Returns
/// * `Ok(String)` - The validated tax code
/// * `Err(ValidationError)` - If validation fails
pub fn validate_tax_code(tax_code: &str) -> Result<String, ValidationError> {
    if tax_code.trim().is_empty() {
        return Err(ValidationError::EmptyTaxCode);
    }
    Ok(tax_code.to_string())
}

/// Validates transaction ID is not empty.
/// 
/// # Arguments
/// * `transaction_id` - The transaction ID to validate
/// 
/// # Returns
/// * `Ok(String)` - The validated transaction ID
/// * `Err(ValidationError)` - If validation fails
pub fn validate_transaction_id(transaction_id: &str) -> Result<String, ValidationError> {
    if transaction_id.trim().is_empty() {
        return Err(ValidationError::EmptyTransactionId);
    }
    Ok(transaction_id.to_string())
}

/// Validates target product ID is not empty.
/// 
/// # Arguments
/// * `target_product_id` - The target product ID to validate
/// 
/// # Returns
/// * `Ok(String)` - The validated target product ID
/// * `Err(ValidationError)` - If validation fails
pub fn validate_target_product_id(target_product_id: &str) -> Result<String, ValidationError> {
    if target_product_id.trim().is_empty() {
        return Err(ValidationError::EmptyTargetProductId);
    }
    Ok(target_product_id.to_string())
}

/// Validates UUID string representation doesn't exceed maximum length.
/// 
/// # Arguments
/// * `uuid` - The UUID to validate
/// 
/// # Returns
/// * `Ok(Uuid)` - The validated UUID
/// * `Err(ValidationError)` - If validation fails
pub fn validate_uuid(uuid: &Uuid) -> Result<Uuid, ValidationError> {
    let uuid_string = uuid.to_string();
    if uuid_string.len() > MAXIMUM_REQUEST_REFERENCE_ID_LENGTH {
        return Err(ValidationError::UuidTooLong(uuid_string.len()));
    }
    Ok(*uuid)
}

/// Validates price is non-negative.
/// 
/// # Arguments
/// * `price` - The price to validate
/// 
/// # Returns
/// * `Ok(i64)` - The validated price
/// * `Err(ValidationError)` - If validation fails
pub fn validate_price(price: i64) -> Result<i64, ValidationError> {
    if price < 0 {
        return Err(ValidationError::NegativePrice(price));
    }
    Ok(price)
}

/// Validates description does not exceed maximum length.
/// 
/// # Arguments
/// * `description` - The description to validate
/// 
/// # Returns
/// * `Ok(String)` - The validated description
/// * `Err(ValidationError)` - If validation fails
pub fn validate_description(description: &str) -> Result<String, ValidationError> {
    if description.len() > MAXIMUM_DESCRIPTION_LENGTH {
        return Err(ValidationError::DescriptionTooLong(description.len()));
    }
    Ok(description.to_string())
}

/// Validates display name does not exceed maximum length.
/// 
/// # Arguments
/// * `display_name` - The display name to validate
/// 
/// # Returns
/// * `Ok(String)` - The validated display name
/// * `Err(ValidationError)` - If validation fails
pub fn validate_display_name(display_name: &str) -> Result<String, ValidationError> {
    if display_name.len() > MAXIMUM_DISPLAY_NAME_LENGTH {
        return Err(ValidationError::DisplayNameTooLong(display_name.len()));
    }
    Ok(display_name.to_string())
}

/// Validates SKU does not exceed maximum length.
/// 
/// # Arguments
/// * `sku` - The SKU to validate
/// 
/// # Returns
/// * `Ok(String)` - The validated SKU
/// * `Err(ValidationError)` - If validation fails
pub fn validate_sku(sku: &str) -> Result<String, ValidationError> {
    if sku.len() > MAXIMUM_SKU_LENGTH {
        return Err(ValidationError::SkuTooLong(sku.len()));
    }
    Ok(sku.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_currency_valid() {
        assert_eq!(validate_currency("USD").unwrap(), "USD");
        assert_eq!(validate_currency("EUR").unwrap(), "EUR");
        assert_eq!(validate_currency("GBP").unwrap(), "GBP");
    }

    #[test]
    fn test_validate_currency_invalid_length() {
        assert!(matches!(
            validate_currency("US"),
            Err(ValidationError::InvalidCurrencyLength(2))
        ));
        assert!(matches!(
            validate_currency("USDD"),
            Err(ValidationError::InvalidCurrencyLength(4))
        ));
    }

    #[test]
    fn test_validate_currency_invalid_format() {
        assert!(matches!(
            validate_currency("usd"),
            Err(ValidationError::InvalidCurrencyFormat(_))
        ));
        assert!(matches!(
            validate_currency("US1"),
            Err(ValidationError::InvalidCurrencyFormat(_))
        ));
    }

    #[test]
    fn test_validate_price_valid() {
        assert_eq!(validate_price(0).unwrap(), 0);
        assert_eq!(validate_price(100).unwrap(), 100);
        assert_eq!(validate_price(999999).unwrap(), 999999);
    }

    #[test]
    fn test_validate_price_invalid() {
        assert!(matches!(
            validate_price(-1),
            Err(ValidationError::NegativePrice(-1))
        ));
        assert!(matches!(
            validate_price(-100),
            Err(ValidationError::NegativePrice(-100))
        ));
    }

    #[test]
    fn test_validate_empty_strings() {
        assert!(matches!(
            validate_tax_code(""),
            Err(ValidationError::EmptyTaxCode)
        ));
        assert!(matches!(
            validate_tax_code("  "),
            Err(ValidationError::EmptyTaxCode)
        ));
        assert!(validate_tax_code("ABC123").is_ok());
    }

    #[test]
    fn test_validate_lengths() {
        let long_description = "a".repeat(46);
        assert!(matches!(
            validate_description(&long_description),
            Err(ValidationError::DescriptionTooLong(46))
        ));
        
        let ok_description = "a".repeat(45);
        assert!(validate_description(&ok_description).is_ok());
        
        let long_display_name = "a".repeat(31);
        assert!(matches!(
            validate_display_name(&long_display_name),
            Err(ValidationError::DisplayNameTooLong(31))
        ));
        
        let ok_display_name = "a".repeat(30);
        assert!(validate_display_name(&ok_display_name).is_ok());
        
        let long_sku = "a".repeat(129);
        assert!(matches!(
            validate_sku(&long_sku),
            Err(ValidationError::SkuTooLong(129))
        ));
        
        let ok_sku = "a".repeat(128);
        assert!(validate_sku(&ok_sku).is_ok());
    }

    #[test]
    fn test_validate_uuid() {
        let uuid = Uuid::new_v4();
        assert_eq!(validate_uuid(&uuid).unwrap(), uuid);
    }
}