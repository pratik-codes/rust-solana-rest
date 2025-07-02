use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use bs58;

use super::errors::{AppError, Result};

/// Validates that a string is a valid base58-encoded Solana public key
pub fn validate_pubkey(key: &str, field_name: &str) -> Result<Pubkey> {
    if key.is_empty() {
        return Err(AppError::ValidationError(format!("{} is required", field_name)));
    }
    
    // First check if it's valid base58
    let decoded = bs58::decode(key)
        .into_vec()
        .map_err(|_| {
            if field_name == "sender" {
                AppError::ValidationError("Invalid sender public key".to_string())
            } else {
                AppError::InvalidPublicKey(format!("Invalid {}: not valid base58", field_name))
            }
        })?;
    
    // Solana public keys should be exactly 32 bytes
    if decoded.len() != 32 {
        return Err(if field_name == "sender" {
            AppError::ValidationError("Invalid sender public key".to_string())
        } else {
            AppError::InvalidPublicKey(format!("Invalid {}: incorrect length", field_name))
        });
    }
    
    // Parse as Solana public key
    Pubkey::from_str(key)
        .map_err(|_| {
            if field_name == "sender" {
                AppError::ValidationError("Invalid sender public key".to_string())
            } else {
                AppError::InvalidPublicKey(format!("Invalid {}: {}", field_name, key))
            }
        })
}

/// Validates that an amount is positive (greater than 0)
pub fn validate_positive_amount(amount: u64, field_name: &str) -> Result<u64> {
    if amount == 0 {
        return Err(AppError::ValidationError("Amount must be greater than 0".to_string()));
    }
    Ok(amount)
}

/// Validates that a string is non-empty
pub fn validate_non_empty_string(value: &str, field_name: &str) -> Result<()> {
    if value.is_empty() {
        return Err(AppError::ValidationError(format!("{} is required", field_name)));
    }
    Ok(())
}

/// Validates that a string is a valid base58-encoded secret key (64 bytes when decoded)
pub fn validate_secret_key(secret_key: &str) -> Result<()> {
    if secret_key.is_empty() {
        return Err(AppError::ValidationError("secret is required".to_string()));
    }
    
    let decoded = bs58::decode(secret_key)
        .into_vec()
        .map_err(|_| AppError::InvalidSecretKey("Invalid secret key format".to_string()))?;
    
    if decoded.len() != 64 {
        return Err(AppError::InvalidSecretKey("Invalid secret key length".to_string()));
    }
    
    Ok(())
}

/// Validates that a string is a valid base58-encoded signature
pub fn validate_signature_format(signature: &str) -> Result<Vec<u8>> {
    if signature.is_empty() {
        return Err(AppError::ValidationError("signature is required".to_string()));
    }
    
    let decoded = bs58::decode(signature)
        .into_vec()
        .map_err(|_| AppError::InvalidSignature("Invalid signature format: not valid base58".to_string()))?;
    
    if decoded.len() != 64 {
        return Err(AppError::InvalidSignature("Invalid signature length: must be 64 bytes".to_string()));
    }
    
    Ok(decoded)
}

/// Validates decimals for token creation (0-9 is standard range)
pub fn validate_decimals(decimals: u8) -> Result<u8> {
    if decimals > 9 {
        return Err(AppError::ValidationError("decimals must be between 0 and 9".to_string()));
    }
    Ok(decimals)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_pubkey_valid() {
        let valid_pubkey = "11111111111111111111111111111112";
        let result = validate_pubkey(valid_pubkey, "test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_pubkey_invalid() {
        let invalid_pubkey = "invalid";
        let result = validate_pubkey(invalid_pubkey, "test");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_pubkey_empty() {
        let result = validate_pubkey("", "test");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_positive_amount_valid() {
        let result = validate_positive_amount(1000, "amount");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1000);
    }

    #[test]
    fn test_validate_positive_amount_zero() {
        let result = validate_positive_amount(0, "amount");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_non_empty_string_valid() {
        let result = validate_non_empty_string("test", "message");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_non_empty_string_empty() {
        let result = validate_non_empty_string("", "message");
        assert!(result.is_err());
    }
} 
