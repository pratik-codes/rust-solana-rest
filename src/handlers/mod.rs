use axum::{
    extract::{Json as ExtractJson, rejection::JsonRejection},
    response::Json,
    async_trait,
    extract::FromRequest,
    http::Request,
};
use tracing::{info, error};
use serde::de::DeserializeOwned;

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
    SendSolRequest,
    SendSolResponse,
    SendTokenRequest,
    SendTokenResponse,
};
use crate::services::solana::SolanaService;
use crate::utils::errors::{AppError, Result};
use crate::utils::validation;

/// Custom JSON extractor that handles deserialization errors properly
pub struct JsonExtractor<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for JsonExtractor<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request<axum::body::Body>, state: &S) -> Result<Self> {
        match ExtractJson::<T>::from_request(req, state).await {
            Ok(json) => Ok(JsonExtractor(json.0)),
            Err(rejection) => {
                let error_message = match rejection {
                    JsonRejection::JsonDataError(err) => {
                        let err_str = err.to_string();
                        if err_str.contains("missing field") || 
                           err_str.contains("missing field `") ||
                           err_str.contains("expected value") ||
                           err_str.contains("Failed to deserialize") {
                            "Missing required fields".to_string()
                        } else {
                            "Invalid JSON data".to_string()
                        }
                    }
                    JsonRejection::JsonSyntaxError(_) => {
                        "Invalid JSON syntax".to_string()
                    }
                    JsonRejection::MissingJsonContentType(_) => {
                        "Missing Content-Type: application/json header".to_string()
                    }
                    _ => "Invalid request body".to_string(),
                };
                Err(AppError::ValidationError(error_message))
            }
        }
    }
}

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
    JsonExtractor(request): JsonExtractor<CreateTokenRequest>,
) -> Result<Json<ApiResponse<TokenInstructionResponse>>> {
    info!("Handling token creation request for mint: {}", request.mint);

    // Comprehensive validation using validation module
    let mint_authority = validation::validate_pubkey(&request.mint_authority, "mintAuthority")?;
    let mint = validation::validate_pubkey(&request.mint, "mint")?;
    let decimals = validation::validate_decimals(request.decimals)?;

    let solana_service = SolanaService::new();

    match solana_service.create_token_mint(
        &mint_authority.to_string(),
        &mint.to_string(),
        decimals,
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
    JsonExtractor(request): JsonExtractor<MintTokenRequest>,
) -> Result<Json<ApiResponse<TokenInstructionResponse>>> {
    info!("Handling token minting request for mint: {}", request.mint);

    // Comprehensive validation using validation module
    let mint = validation::validate_pubkey(&request.mint, "mint")?;
    let destination = validation::validate_pubkey(&request.destination, "destination")?;
    let authority = validation::validate_pubkey(&request.authority, "authority")?;
    let amount = validation::validate_positive_amount(request.amount, "amount")?;

    let solana_service = SolanaService::new();

    match solana_service.mint_token(
        &mint.to_string(),
        &destination.to_string(),
        &authority.to_string(),
        amount,
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
    JsonExtractor(request): JsonExtractor<SignMessageRequest>,
) -> Result<Json<ApiResponse<SignMessageResponse>>> {
    info!("Handling message signing request");

    // Comprehensive validation using validation module
    validation::validate_non_empty_string(&request.message, "message")?;
    validation::validate_secret_key(&request.secret)?;

    let solana_service = SolanaService::new();

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
    JsonExtractor(request): JsonExtractor<VerifyMessageRequest>,
) -> Result<Json<ApiResponse<VerifyMessageResponse>>> {
    info!("Handling message verification request");

    // Comprehensive validation using validation module
    validation::validate_non_empty_string(&request.message, "message")?;
    let _signature_bytes = validation::validate_signature_format(&request.signature)?;
    let pubkey = validation::validate_pubkey(&request.pubkey, "pubkey")?;

    let solana_service = SolanaService::new();

    match solana_service.verify_message(
        &request.message,
        &request.signature,
        &pubkey.to_string(),
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

/// Handler for POST /send/sol
/// Creates a SOL transfer instruction
pub async fn send_sol_handler(
    JsonExtractor(request): JsonExtractor<SendSolRequest>,
) -> Result<Json<ApiResponse<SendSolResponse>>> {
    info!("Handling SOL transfer request from {} to {}", request.from, request.to);

    // Comprehensive validation using validation module
    let from = validation::validate_pubkey(&request.from, "sender")?;
    let to = validation::validate_pubkey(&request.to, "recipient")?;
    let lamports = validation::validate_positive_amount(request.lamports, "lamports")?;

    let solana_service = SolanaService::new();

    match solana_service.send_sol(&from.to_string(), &to.to_string(), lamports) {
        Ok(sol_response) => {
            info!("Successfully created SOL transfer instruction");
            Ok(Json(ApiResponse::success(sol_response)))
        }
        Err(e) => {
            error!("Failed to create SOL transfer instruction: {}", e);
            Err(e)
        }
    }
}

/// Handler for POST /send/token
/// Creates an SPL token transfer instruction
pub async fn send_token_handler(
    JsonExtractor(request): JsonExtractor<SendTokenRequest>,
) -> Result<Json<ApiResponse<SendTokenResponse>>> {
    info!("Handling token transfer request for mint: {}", request.mint);

    // Comprehensive validation using validation module
    let destination = validation::validate_pubkey(&request.destination, "destination")?;
    let mint = validation::validate_pubkey(&request.mint, "mint")?;
    let owner = validation::validate_pubkey(&request.owner, "owner")?;
    let amount = validation::validate_positive_amount(request.amount, "amount")?;

    let solana_service = SolanaService::new();

    match solana_service.send_token(
        &destination.to_string(),
        &mint.to_string(),
        &owner.to_string(),
        amount,
    ) {
        Ok(token_response) => {
            info!("Successfully created token transfer instruction");
            Ok(Json(ApiResponse::success(token_response)))
        }
        Err(e) => {
            error!("Failed to create token transfer instruction: {}", e);
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{CreateTokenRequest, MintTokenRequest, SignMessageRequest, VerifyMessageRequest, SendSolRequest, SendTokenRequest};

    #[tokio::test]
    async fn test_generate_keypair_handler() {
        let result = generate_keypair_handler().await;
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert!(response.0.success);
        assert!(!response.0.data.pubkey.is_empty());
        assert!(!response.0.data.secret.is_empty());
    }

    #[tokio::test]
    async fn test_create_token_handler_validation() {
        let invalid_request = CreateTokenRequest {
            mint_authority: "".to_string(),
            mint: "".to_string(),
            decimals: 9,
        };
        
        let result = create_token_handler(JsonExtractor(invalid_request)).await;
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
        
        let result = mint_token_handler(JsonExtractor(invalid_request)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_sign_message_handler_validation() {
        let invalid_request = SignMessageRequest {
            message: "".to_string(),
            secret: "".to_string(),
        };
        
        let result = sign_message_handler(JsonExtractor(invalid_request)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_verify_message_handler_validation() {
        let invalid_request = VerifyMessageRequest {
            message: "".to_string(),
            signature: "".to_string(),
            pubkey: "".to_string(),
        };
        
        let result = verify_message_handler(JsonExtractor(invalid_request)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_send_sol_handler_validation() {
        let invalid_request = SendSolRequest {
            from: "".to_string(),
            to: "".to_string(),
            lamports: 0,
        };
        
        let result = send_sol_handler(JsonExtractor(invalid_request)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_send_token_handler_validation() {
        let invalid_request = SendTokenRequest {
            destination: "".to_string(),
            mint: "".to_string(),
            owner: "".to_string(),
            amount: 0,
        };
        
        let result = send_token_handler(JsonExtractor(invalid_request)).await;
        assert!(result.is_err());
    }
} 
