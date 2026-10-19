//! Logging Middleware
//!
//! Middleware for request/response logging.

use axum::{
    body::Body,
    extract::State,
    http::{Request, Response},
    middleware::Next,
};
use std::time::Instant;
use tracing::{info, error, Level};

use super::error_handler::AppError;

/// Create logging middleware.
///
/// This middleware logs all requests and responses, including:
/// - Request method and path
/// - Response status code
/// - Request duration
/// - Error information (if any)
pub fn logging_middleware<B: Body + Send + 'static>(
) -> impl axum::middleware::Middleware<B, State = (), Response = Response<Body>> + Clone
where
    B::Data: Send + 'static,
    B::Error: std::error::Error + Send + Sync + 'static,
{
    axum::middleware::from_fn_with_state((), async move |request: Request<B>, next: Next<B>| {
        let start = Instant::now();
        let method = request.method().clone();
        let path = request.uri().path().to_string();
        let query = request.uri().query().map(|q| format!("?{q}")).unwrap_or_default();
        let full_path = format!("{path}{query}");

        // Log request
        info!(
            target: "http",
            method = %method,
            path = %full_path,
            "Request started"
        );

        // Process the request
        let response = match next.run(request).await {
            Ok(response) => response,
            Err(e) => {
                error!(
                    target: "http",
                    method = %method,
                    path = %full_path,
                    duration = ?start.elapsed(),
                    error = %e,
                    "Request failed"
                );
                return Response::builder()
                    .status(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from(format!("Internal server error: {e}")))
                    .unwrap();
            }
        };

        let status = response.status();
        let duration = start.elapsed();

        // Log response
        if status.is_server_error() {
            error!(
                target: "http",
                method = %method,
                path = %full_path,
                status = %status,
                duration = ?duration,
                "Request completed with server error"
            );
        } else if status.is_client_error() {
            info!(
                target: "http",
                method = %method,
                path = %full_path,
                status = %status,
                duration = ?duration,
                "Request completed with client error"
            );
        } else {
            info!(
                target: "http",
                method = %method,
                path = %full_path,
                status = %status,
                duration = ?duration,
                "Request completed"
            );
        }

        response
    })
}

/// Enhanced logging middleware with more details.
pub fn detailed_logging_middleware<B: Body + Send + 'static>(
) -> impl axum::middleware::Middleware<B, State = (), Response = Response<Body>> + Clone
where
    B::Data: Send + 'static,
    B::Error: std::error::Error + Send + Sync + 'static,
{
    axum::middleware::from_fn_with_state((), async move |request: Request<B>, next: Next<B>| {
        let start = Instant::now();
        let method = request.method().clone();
        let path = request.uri().path().to_string();
        let query = request.uri().query().map(|q| format!("?{q}")).unwrap_or_default();
        let full_path = format!("{path}{query}");

        // Extract headers for logging
        let headers: Vec<_> = request
            .headers()
            .iter()
            .filter(|(name, _)| {
                // Filter sensitive headers
                !name.eq_ignore_ascii_case("authorization")
                    && !name.eq_ignore_ascii_case("cookie")
                    && !name.eq_ignore_ascii_case("x-csrf-token")
            })
            .map(|(name, value)| (name.to_string(), value.to_str().unwrap_or("").to_string()))
            .collect();

        // Log request with headers
        info!(
            target: "http.detailed",
            method = %method,
            path = %full_path,
            headers = ?headers,
            "Request started"
        );

        let response = next.run(request).await;
        let status = response.status();
        let duration = start.elapsed();

        // Log response
        if status.is_server_error() {
            error!(
                target: "http.detailed",
                method = %method,
                path = %full_path,
                status = %status,
                duration = ?duration,
                "Request completed with server error"
            );
        } else {
            info!(
                target: "http.detailed",
                method = %method,
                path = %full_path,
                status = %status,
                duration = ?duration,
                "Request completed"
            );
        }

        response
    })
}
