use axum::{
    extract::Json as ExtractJson,
    response::Json,
};
use tracing::{info, error};

use crate::models::{
    ApiResponse, 
    KeypairResponse,
    CreateTokenRequest,
    MintTokenRequest,
    TokenInstructionResponse,
    SignMessageRequest,
    SignMessageResponse,
    VerifyMessageRequest,
    VerifyMessageResponse,
};
use crate::services::solana::SolanaService;
use crate::errors::{AppError, Result};

/// Handler for POST /keypair
/// Generates a new Solana keypair
pub async fn generate_keypair_handler() -> Result<Json<ApiResponse<KeypairResponse>>> {
    info!("Handling keypair generation request");

    let solana_service = SolanaService::new();
    
    match solana_service.generate_keypair() {
        Ok(keypair_response) => {
            info!("Successfully generated new keypair");
            Ok(Json(ApiResponse::success(keypair_response)))
        }
        Err(e) => {
            error!("Failed to generate keypair: {}", e);
            Err(e)
        }
    }
}

/// Handler for POST /token/create
/// Creates an SPL token mint instruction
pub async fn create_token_handler(
    ExtractJson(request): ExtractJson<CreateTokenRequest>,
) -> Result<Json<ApiResponse<TokenInstructionResponse>>> {
    info!("Handling token creation request for mint: {}", request.mint);

    // Validate request
    if request.mint_authority.is_empty() {
        return Err(AppError::ValidationError("mintAuthority is required".to_string()));
    }
    if request.mint.is_empty() {
        return Err(AppError::ValidationError("mint is required".to_string()));
    }

    let solana_service = SolanaService::new();

    // Validate public keys
    if !solana_service.is_valid_pubkey(&request.mint_authority) {
        return Err(AppError::InvalidPublicKey(format!("Invalid mintAuthority: {}", request.mint_authority)));
    }
    if !solana_service.is_valid_pubkey(&request.mint) {
        return Err(AppError::InvalidPublicKey(format!("Invalid mint: {}", request.mint)));
    }

    match solana_service.create_token_mint(
        &request.mint_authority,
        &request.mint,
        request.decimals,
    ) {
        Ok(token_response) => {
            info!("Successfully created token mint instruction for mint: {}", request.mint);
            Ok(Json(ApiResponse::success(token_response)))
        }
        Err(e) => {
            error!("Failed to create token mint instruction: {}", e);
            Err(e)
        }
    }
}

/// Handler for POST /token/mint
/// Creates an SPL token mint_to instruction
pub async fn mint_token_handler(
    ExtractJson(request): ExtractJson<MintTokenRequest>,
) -> Result<Json<ApiResponse<TokenInstructionResponse>>> {
    info!("Handling token minting request for mint: {}", request.mint);

    // Validate request
    if request.mint.is_empty() {
        return Err(AppError::ValidationError("mint is required".to_string()));
    }
    if request.destination.is_empty() {
        return Err(AppError::ValidationError("destination is required".to_string()));
    }
    if request.authority.is_empty() {
        return Err(AppError::ValidationError("authority is required".to_string()));
    }
    if request.amount == 0 {
        return Err(AppError::ValidationError("amount must be greater than 0".to_string()));
    }

    let solana_service = SolanaService::new();

    // Validate public keys
    if !solana_service.is_valid_pubkey(&request.mint) {
        return Err(AppError::InvalidPublicKey(format!("Invalid mint: {}", request.mint)));
    }
    if !solana_service.is_valid_pubkey(&request.destination) {
        return Err(AppError::InvalidPublicKey(format!("Invalid destination: {}", request.destination)));
    }
    if !solana_service.is_valid_pubkey(&request.authority) {
        return Err(AppError::InvalidPublicKey(format!("Invalid authority: {}", request.authority)));
    }

    match solana_service.mint_token(
        &request.mint,
        &request.destination,
        &request.authority,
        request.amount,
    ) {
        Ok(token_response) => {
            info!("Successfully created token mint_to instruction for mint: {}", request.mint);
            Ok(Json(ApiResponse::success(token_response)))
        }
        Err(e) => {
            error!("Failed to create token mint_to instruction: {}", e);
            Err(e)
        }
    }
}

/// Handler for POST /message/sign
/// Signs a message with the provided secret key
pub async fn sign_message_handler(
    ExtractJson(request): ExtractJson<SignMessageRequest>,
) -> Result<Json<ApiResponse<SignMessageResponse>>> {
    info!("Handling message signing request");

    // Validate request
    if request.message.is_empty() {
        return Err(AppError::ValidationError("message is required".to_string()));
    }
    if request.secret.is_empty() {
        return Err(AppError::ValidationError("secret is required".to_string()));
    }

    let solana_service = SolanaService::new();

    // Validate secret key format
    if !solana_service.is_valid_secret_key(&request.secret) {
        return Err(AppError::InvalidSecretKey("Invalid secret key format".to_string()));
    }

    match solana_service.sign_message(&request.message, &request.secret) {
        Ok(sign_response) => {
            info!("Successfully signed message");
            Ok(Json(ApiResponse::success(sign_response)))
        }
        Err(e) => {
            error!("Failed to sign message: {}", e);
            Err(e)
        }
    }
}

/// Handler for POST /message/verify
/// Verifies a message signature
pub async fn verify_message_handler(
    ExtractJson(request): ExtractJson<VerifyMessageRequest>,
) -> Result<Json<ApiResponse<VerifyMessageResponse>>> {
    info!("Handling message verification request");

    // Validate request
    if request.message.is_empty() {
        return Err(AppError::ValidationError("message is required".to_string()));
    }
    if request.signature.is_empty() {
        return Err(AppError::ValidationError("signature is required".to_string()));
    }
    if request.public_key.is_empty() {
        return Err(AppError::ValidationError("public_key is required".to_string()));
    }

    let solana_service = SolanaService::new();

    // Validate public key format
    if !solana_service.is_valid_pubkey(&request.public_key) {
        return Err(AppError::InvalidPublicKey(format!("Invalid public_key: {}", request.public_key)));
    }

    match solana_service.verify_message(
        &request.message,
        &request.signature,
        &request.public_key,
    ) {
        Ok(verify_response) => {
            info!("Successfully verified message signature: {}", verify_response.valid);
            Ok(Json(ApiResponse::success(verify_response)))
        }
        Err(e) => {
            error!("Failed to verify message signature: {}", e);
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{CreateTokenRequest, MintTokenRequest, SignMessageRequest, VerifyMessageRequest};

    #[tokio::test]
    async fn test_generate_keypair_handler() {
        let result = generate_keypair_handler().await;
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert!(response.0.success);
        assert!(!response.0.data.public_key.is_empty());
        assert!(!response.0.data.secret_key.is_empty());
    }

    #[tokio::test]
    async fn test_create_token_handler_validation() {
        let invalid_request = CreateTokenRequest {
            mint_authority: "".to_string(),
            mint: "".to_string(),
            decimals: 9,
        };
        
        let result = create_token_handler(ExtractJson(invalid_request)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_mint_token_handler_validation() {
        let invalid_request = MintTokenRequest {
            mint: "".to_string(),
            destination: "".to_string(),
            authority: "".to_string(),
            amount: 0,
        };
        
        let result = mint_token_handler(ExtractJson(invalid_request)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_sign_message_handler_validation() {
        let invalid_request = SignMessageRequest {
            message: "".to_string(),
            secret: "".to_string(),
        };
        
        let result = sign_message_handler(ExtractJson(invalid_request)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_verify_message_handler_validation() {
        let invalid_request = VerifyMessageRequest {
            message: "".to_string(),
            signature: "".to_string(),
            public_key: "".to_string(),
        };
        
        let result = verify_message_handler(ExtractJson(invalid_request)).await;
        assert!(result.is_err());
    }
} 
