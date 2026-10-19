//! Application
//!
//! Creates and configures the Axum application.

use axum::{Router, routing::get};
use quepid_api::config::AppConfig;
use quepid_api::routes::{api, web};
use quepid_infra::config::DatabaseConfig;
use quepid_infra::database::connection::DatabaseConnection;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use crate::middleware::logging::logging_middleware;

/// Application state shared across all handlers.
#[derive(Debug, Clone)]
pub struct AppState {
    /// Database connection
    pub db: Arc<DatabaseConnection>,
    /// Application configuration
    pub config: Arc<AppConfig>,
}

impl AppState {
    /// Create a new AppState with the given configuration.
    pub async fn new(config: &AppConfig) -> Self {
        let db_config = DatabaseConfig {
            backend: config.database.backend,
            connection_string: config.database.connection_string.clone(),
            max_connections: config.database.max_connections,
            connect_timeout: config.database.connect_timeout,
            query_timeout: config.database.query_timeout,
        };

        let db = Arc::new(DatabaseConnection::new(db_config.connection_string));

        Self {
            db,
            config: Arc::new(config.clone()),
        }
    }
}

/// Create the Axum application router.
///
/// This function sets up all routes, middleware, and error handling.
pub async fn create_app(config: &AppConfig) -> Router {
    let state = AppState::new(config).await;

    // Build the router
    let mut app = Router::new();

    // Add health check endpoint
    app = app.route("/healthcheck", get(health_check));

    // Add API routes
    app = app.nest("/api/v1", api::routes());

    // Add web routes (auth, pages, etc.)
    app = app.nest("/", web::routes());

    // Add static file serving for assets
    if let Some(public_path) = &config.server.public_path {
        app = app.nest_service("/assets", ServeDir::new(public_path));
    }

    // Add middleware
    app = app
        .route_layer(TraceLayer::new_for_http())
        .route_layer(logging_middleware());

    // Add state
    app.with_state(state)
}

/// Health check handler.
///
/// Returns a simple JSON response indicating the server is healthy.
async fn health_check() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
