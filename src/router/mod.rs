use axum::{
    routing::post,
    Router,
};
use tower_http::cors::CorsLayer;

use crate::handlers::{
    generate_keypair_handler,
    create_token_handler,
    mint_token_handler,
    sign_message_handler,
    verify_message_handler,
};

/// Creates and configures the main application router
pub fn create_router() -> Router {
    Router::new()
        // POST /keypair - Generate new Solana keypair
        .route("/keypair", post(generate_keypair_handler))
        // POST /token/create - Create SPL token mint instruction
        .route("/token/create", post(create_token_handler))
        // POST /token/mint - Create SPL token mint_to instruction
        .route("/token/mint", post(mint_token_handler))
        // POST /message/sign - Sign a message with secret key
        .route("/message/sign", post(sign_message_handler))
        // POST /message/verify - Verify a message signature
        .route("/message/verify", post(verify_message_handler))
        // Add CORS middleware to allow cross-origin requests
        .layer(CorsLayer::permissive())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router_creation() {
        let _router = create_router();
        // Basic test to ensure router can be created without panicking
        assert!(true);
    }
} 
