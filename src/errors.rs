use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

use crate::models::ApiErrorResponse;

/// Application error types
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid request: {0}")]
    BadRequest(String),
    
    #[error("Invalid keypair: {0}")]
    InvalidKeypair(String),
    
    #[error("Invalid public key: {0}")]
    InvalidPublicKey(String),
    
    #[error("Invalid secret key: {0}")]
    InvalidSecretKey(String),
    
    #[error("Invalid signature: {0}")]
    InvalidSignature(String),
    
    #[error("Signature verification failed")]
    SignatureVerificationFailed,
    
    #[error("Token operation failed: {0}")]
    TokenOperationFailed(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
    
    #[error("Internal server error: {0}")]
    InternalServerError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::InvalidKeypair(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::InvalidPublicKey(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::InvalidSecretKey(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::InvalidSignature(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::SignatureVerificationFailed => (StatusCode::BAD_REQUEST, "Signature verification failed".to_string()),
            AppError::TokenOperationFailed(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::SerializationError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::DeserializationError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        let body = Json(ApiErrorResponse::error(&error_message));
        (status, body).into_response()
    }
}

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, AppError>;

/// Helper function to convert base58 decode errors
pub fn base58_decode_error(err: bs58::decode::Error) -> AppError {
    AppError::DeserializationError(format!("Base58 decode error: {}", err))
}

/// Helper function to convert base64 decode errors  
pub fn base64_decode_error(err: base64::DecodeError) -> AppError {
    AppError::DeserializationError(format!("Base64 decode error: {}", err))
}

/// Helper function to convert serialization errors
pub fn serialization_error(err: impl std::fmt::Display) -> AppError {
    AppError::SerializationError(format!("Serialization error: {}", err))
}

/// Helper function to convert bincode errors
pub fn bincode_error(err: impl std::fmt::Display) -> AppError {
    AppError::SerializationError(format!("Bincode error: {}", err))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_app_error_into_response() {
        let error = AppError::BadRequest("Test error".to_string());
        let response = error.into_response();
        
        // Check that the response has the correct status code
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_error_display() {
        let error = AppError::InvalidPublicKey("test key".to_string());
        assert_eq!(error.to_string(), "Invalid public key: test key");
    }

    #[test]
    fn test_helper_functions() {
        let base58_err = bs58::decode::Error::InvalidCharacter { character: 'x', index: 0 };
        let app_err = base58_decode_error(base58_err);
        assert!(matches!(app_err, AppError::DeserializationError(_)));
    }
}
