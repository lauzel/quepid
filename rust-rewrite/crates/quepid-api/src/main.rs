//! Quepid API Server
//!
//! Main entry point for the HTTP API server.

use axum::Router;
use quepid_api::app::create_app;
use quepid_infra::config::AppConfig;
use std::net::SocketAddr;
use tracing::{info, error};

#[tokio::main]
async fn main() {
    // Initialize logging
    init_logging();

    // Load configuration
    let config = match AppConfig::from_env() {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load configuration: {e}");
            std::process::exit(1);
        }
    };

    info!("Starting Quepid API server");
    info!("Configuration: {:?}", config);

    // Create the application
    let app = create_app(&config).await;

    // Bind to socket address
    let addr = SocketAddr::from((config.server.host, config.server.port));
    info!("Listening on {addr}");

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| {
            error!("Failed to bind to {addr}: {e}");
            std::process::exit(1);
        })
        .unwrap();

    axum::serve(listener, app)
        .await
        .map_err(|e| {
            error!("Server error: {e}");
            std::process::exit(1);
        })
        .unwrap();
}

/// Initialize logging with tracing.
fn init_logging() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}
