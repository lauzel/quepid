//! Error Handler Middleware
//!
//! Centralized error handling for the API.

use axum::{
    body::Body,
    extract::State,
    http::{Request, Response, StatusCode},
    middleware::Next,
    Json,
};
use serde_json::json;
use thiserror::Error;
use tracing::{error, debug};

use crate::AppState;

/// Application error type.
///
/// All errors in the application should be converted to this type
/// for consistent error handling.
#[derive(Debug, Error)]
pub enum AppError {
    /// Domain error from the core layer
    #[error("Domain error: {0}")]
    Domain(String),

    /// Repository error from the infrastructure layer
    #[error("Repository error: {0}")]
    Repository(String),

    /// Service error
    #[error("Service error: {0}")]
    Service(String),

    /// Authentication error
    #[error("Authentication error: {0}")]
    Authentication(String),

    /// Authorization error
    #[error("Authorization error: {0}")]
    Authorization(String),

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Not found error
    #[error("Not found")]
    NotFound,

    /// Conflict error
    #[error("Conflict: {0}")]
    Conflict(String),

    /// Internal server error
    #[error("Internal server error")]
    InternalServerError,

    /// Bad request error
    #[error("Bad request: {0}")]
    BadRequest(String),

    /// External service error
    #[error("External service error: {0}")]
    ExternalService(String),
}

impl AppError {
    /// Get the HTTP status code for this error.
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::Domain(_) => StatusCode::BAD_REQUEST,
            AppError::Repository(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Service(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Authentication(_) => StatusCode::UNAUTHORIZED,
            AppError::Authorization(_) => StatusCode::FORBIDDEN,
            AppError::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::ExternalService(_) => StatusCode::BAD_GATEWAY,
        }
    }

    /// Get the error message for this error.
    pub fn error_message(&self) -> String {
        match self {
            AppError::Domain(msg) => msg.clone(),
            AppError::Repository(msg) => msg.clone(),
            AppError::Service(msg) => msg.clone(),
            AppError::Authentication(msg) => msg.clone(),
            AppError::Authorization(msg) => msg.clone(),
            AppError::Validation(msg) => msg.clone(),
            AppError::NotFound => "Not found".to_string(),
            AppError::Conflict(msg) => msg.clone(),
            AppError::InternalServerError => "Internal server error".to_string(),
            AppError::BadRequest(msg) => msg.clone(),
            AppError::ExternalService(msg) => msg.clone(),
        }
    }

    /// Convert to a JSON error response.
    pub fn to_json_response(&self) -> Json<serde_json::Value> {
        let mut response = json!({
            "error": self.error_message(),
            "status": self.status_code().as_u16(),
        });

        // Add additional context for development
        if cfg!(debug_assertions) {
            response["debug"] = json!(format!("{:?}", self));
        }

        Json(response)
    }
}

/// Convert domain errors to AppError.
impl From<quepid_core::domain::errors::DomainError> for AppError {
    fn from(error: quepid_core::domain::errors::DomainError) -> Self {
        match error {
            quepid_core::domain::errors::DomainError::ValidationError(msg) => {
                AppError::Validation(msg)
            }
            quepid_core::domain::errors::DomainError::NotFound(msg) => {
                AppError::NotFound
            }
            quepid_core::domain::errors::DomainError::Unauthorized(msg) => {
                AppError::Authorization(msg)
            }
            quepid_core::domain::errors::DomainError::Forbidden(msg) => {
                AppError::Authorization(msg)
            }
            quepid_core::domain::errors::DomainError::Conflict(msg) => {
                AppError::Conflict(msg)
            }
            quepid_core::domain::errors::DomainError::InternalError(msg) => {
                AppError::InternalServerError
            }
        }
    }
}

/// Error handler middleware.
///
/// This middleware catches all errors and converts them to appropriate
/// HTTP responses.
pub async fn error_handler_middleware<B: Body + Send + 'static>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response<Body>, AppError> {
    let response = next.run(request).await;

    // If the response is already an error, return it
    if response.status().is_server_error() || response.status().is_client_error() {
        return Ok(response);
    }

    Ok(response)
}

/// Convert AppError to Response.
impl Into<Response<Body>> for AppError {
    fn into(self) -> Response<Body> {
        let status = self.status_code();
        let body = Body::from(serde_json::to_vec(&self.to_json_response()).unwrap());

        Response::builder()
            .status(status)
            .header("Content-Type", "application/json")
            .body(body)
            .unwrap()
    }
}

/// Error response for axum handlers.
pub type Result<T> = std::result::Result<T, AppError>;

/// Helper trait to convert various error types to AppError.
pub trait IntoAppError {
    fn into_app_error(self) -> AppError;
}

impl<T: std::fmt::Display> IntoAppError for T {
    default fn into_app_error(self) -> AppError {
        AppError::InternalServerError
    }
}

impl IntoAppError for String {
    fn into_app_error(self) -> AppError {
        AppError::InternalServerError
    }
}

impl IntoAppError for &str {
    fn into_app_error(self) -> AppError {
        AppError::InternalServerError
    }
}
