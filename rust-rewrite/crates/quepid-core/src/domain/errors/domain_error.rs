//! Domain Error
//!
//! Base error type for domain layer errors.

use thiserror::Error;

/// Base domain error type that all domain errors derive from.
#[derive(Debug, Error)]
pub enum DomainError {
    /// Validation error for domain logic
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Entity not found
    #[error("Not found: {0}")]
    NotFound(String),

    /// Unauthorized access
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    /// Forbidden operation
    #[error("Forbidden: {0}")]
    Forbidden(String),

    /// Conflict (e.g., duplicate entry)
    #[error("Conflict: {0}")]
    Conflict(String),

    /// Internal domain error
    #[error("Internal domain error: {0}")]
    InternalError(String),
}

impl DomainError {
    /// Create a validation error
    pub fn validation_error(message: impl Into<String>) -> Self {
        Self::ValidationError(message.into())
    }

    /// Create a not found error
    pub fn not_found(entity_type: &str, id: impl std::fmt::Display) -> Self {
        Self::NotFound(format!("{entity_type} with id '{id}' not found"))
    }

    /// Create an unauthorized error
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::Unauthorized(message.into())
    }

    /// Create a forbidden error
    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::Forbidden(message.into())
    }

    /// Create a conflict error
    pub fn conflict(message: impl Into<String>) -> Self {
        Self::Conflict(message.into())
    }

    /// Create an internal error
    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::InternalError(message.into())
    }
}
