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
use axum::body::{to_bytes, Body as AxumBody};
use bytes::Bytes;

use crate::handlers::{
    generate_keypair_handler,
    create_token_handler,
    mint_token_handler,
    sign_message_handler,
    verify_message_handler,
    send_sol_handler,
    send_token_handler,
};

/// Generate a curl command from the request details
fn generate_curl_command(
    method: &str,
    uri: &str,
    headers: &axum::http::HeaderMap,
    body: &str,
    host: &str,
) -> String {
    let mut curl_cmd = format!("curl -X {} http://{}{}", method, host, uri);
    
    // Add headers
    for (name, value) in headers.iter() {
        if let Ok(value_str) = value.to_str() {
            // Skip some headers that curl adds automatically or aren't needed for replication
            let header_name = name.as_str().to_lowercase();
            if !matches!(header_name.as_str(), 
                "host" | "user-agent" | "accept-encoding" | "x-forwarded-for" | 
                "x-forwarded-host" | "x-forwarded-proto" | "content-length"
            ) {
                curl_cmd.push_str(&format!(" -H \"{}: {}\"", name, value_str));
            }
        }
    }
    
    // Add body if present
    if !body.is_empty() && method != "GET" {
        curl_cmd.push_str(&format!(" -d '{}'", body.replace("'", "'\\''")));
    }
    
    curl_cmd
}

/// Middleware to log all incoming requests and outgoing responses
async fn logging_middleware(
    req: Request<Body>,
    next: Next,
) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let headers = req.headers().clone();
    
    // Extract the body for curl logging
    let (parts, body) = req.into_parts();
    let body_bytes = match to_bytes(body, usize::MAX).await {
        Ok(bytes) => bytes,
        Err(_) => Bytes::new(),
    };
    
    let body_str = String::from_utf8_lossy(&body_bytes);
    let host = headers.get("host")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("localhost:3000");
    
    // Generate and log the curl command
    let curl_command = generate_curl_command(
        method.as_str(),
        uri.path(),
        &headers,
        &body_str,
        host,
    );
    
    info!("CURL {}", curl_command);
    
    // Log the incoming request (existing functionality)
    info!(
        "REQUEST_INCOMING: {} {} - Headers: {:?}",
        method,
        uri,
        headers
    );

    // Reconstruct the request with the body
    let reconstructed_req = Request::from_parts(parts, AxumBody::from(body_bytes));
    
    // Call the next middleware/handler
    let response = next.run(reconstructed_req).await;
    
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
