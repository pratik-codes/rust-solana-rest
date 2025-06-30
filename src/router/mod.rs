use axum::{
    routing::post,
    Router,
    middleware::{self, Next},
    response::Response,
    http::Request,
    body::Body,
};
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::handlers::{
    generate_keypair_handler,
    create_token_handler,
    mint_token_handler,
    sign_message_handler,
    verify_message_handler,
    send_sol_handler,
    send_token_handler,
};

/// Middleware to log all incoming requests and outgoing responses
async fn logging_middleware(
    req: Request<Body>,
    next: Next,
) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let headers = req.headers().clone();
    
    // Log the incoming request
    info!(
        "REQUEST_INCOMING: {} {} - Headers: {:?}",
        method,
        uri,
        headers
    );

    // Call the next middleware/handler
    let response = next.run(req).await;
    
    // Log the outgoing response
    let status = response.status();
    let response_headers = response.headers().clone();
    
    info!(
        "RESPONSE_DELIVERED: {} {} - Status: {} - Headers: {:?}",
        method,
        uri,
        status,
        response_headers
    );

    response
}

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
        // POST /send/sol - Create SOL transfer instruction
        .route("/send/sol", post(send_sol_handler))
        // POST /send/token - Create SPL token transfer instruction
        .route("/send/token", post(send_token_handler))
        // Add logging middleware
        .layer(middleware::from_fn(logging_middleware))
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
