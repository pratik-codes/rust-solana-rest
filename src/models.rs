use serde::{Deserialize, Serialize};

/// Standard API response wrapper for successful responses
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

/// Standard API response wrapper for error responses  
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiErrorResponse {
    pub success: bool,
    pub error: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data,
        }
    }
}

impl ApiErrorResponse {
    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            error: message.to_string(),
        }
    }
}

/// Response for POST /keypair
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeypairResponse {
    pub public_key: String,
    pub secret_key: String,
}

/// Request for POST /token/create
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateTokenRequest {
    #[serde(rename = "mintAuthority")]
    pub mint_authority: String,
    pub mint: String,
    pub decimals: u8,
}

/// Request for POST /token/mint
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MintTokenRequest {
    pub mint: String,
    pub destination: String,
    pub authority: String,
    pub amount: u64,
}

/// Response for token-related endpoints
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenInstructionResponse {
    pub program_id: String,
    pub accounts: Vec<AccountMeta>,
    pub instruction_data: String,
}

/// Account metadata for Solana instructions
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountMeta {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

/// Request for POST /message/sign
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignMessageRequest {
    pub message: String,
    pub secret: String,
}

/// Response for POST /message/sign
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignMessageResponse {
    pub signature: String,
    pub public_key: String,
    pub message: String,
}

/// Request for POST /message/verify
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerifyMessageRequest {
    pub message: String,
    pub signature: String,
    pub public_key: String,
}

/// Response for POST /message/verify
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerifyMessageResponse {
    pub valid: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_api_response_serialization() {
        let response = ApiResponse::success(KeypairResponse {
            public_key: "test_pub_key".to_string(),
            secret_key: "test_secret_key".to_string(),
        });
        
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("success"));
        assert!(json.contains("true"));
        assert!(json.contains("test_pub_key"));
    }

    #[test]  
    fn test_error_response_serialization() {
        let error_response = ApiErrorResponse::error("Test error message");
        
        let json = serde_json::to_string(&error_response).unwrap();
        assert!(json.contains("success"));
        assert!(json.contains("false"));
        assert!(json.contains("Test error message"));
    }

    #[test]
    fn test_create_token_request_deserialization() {
        let json = r#"{"mintAuthority":"test_authority","mint":"test_mint","decimals":9}"#;
        let request: CreateTokenRequest = serde_json::from_str(json).unwrap();
        
        assert_eq!(request.mint_authority, "test_authority");
        assert_eq!(request.mint, "test_mint");
        assert_eq!(request.decimals, 9);
    }
} 
