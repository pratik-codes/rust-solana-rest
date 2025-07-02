use std::net::SocketAddr;
use tokio;
use tracing::{info, warn};
use tracing_subscriber;

mod router;
mod handlers;
mod services;
mod models;
mod utils;

use router::create_router;

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into())
        )
        .init();

    // Create the application router
    let app = create_router();

    // Define the server address
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Starting Solana HTTP server on {}", addr);

    // Create a TCP listener
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| {
            warn!("Failed to bind to address {}: {}", addr, e);
            std::process::exit(1);
        });

    info!("Server listening on http://{}", addr);
    info!("Available endpoints:");
    info!("  POST /keypair         - Generate new Solana keypair");
    info!("  POST /token/create    - Create SPL token mint instruction");
    info!("  POST /token/mint      - Create SPL token mint_to instruction");
    info!("  POST /message/sign    - Sign message with secret key");
    info!("  POST /message/verify  - Verify message signature");
    info!("  POST /send/sol        - Create SOL transfer instruction");
    info!("  POST /send/token      - Create SPL token transfer instruction");

    // Start serving the application
    axum::serve(listener, app)
        .await
        .unwrap_or_else(|e| {
            warn!("Server error: {}", e);
            std::process::exit(1);
        });
} 
