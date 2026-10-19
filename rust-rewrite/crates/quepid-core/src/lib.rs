//! Quepid Core Domain Layer
//!
//! This crate contains the core domain models, value objects, services,
//! and business logic for Quepid. It has no external dependencies on
//! databases, web frameworks, or other infrastructure concerns.
//!
//! # Architecture
//!
//! The domain layer follows Clean Architecture principles:
//!
//! - **Entities**: Core business objects with identity (User, Case, Query, etc.)
//! - **Value Objects**: Immutable objects without identity (EmailAddress, Score, etc.)
//! - **Domain Services**: Business logic that doesn't belong in entities
//! - **Repositories**: Interfaces for persistence (defined here, implemented in infra)
//! - **Errors**: Domain-specific error types

#![warn(missing_docs)]
#![warn(clippy::all)]
#![allow(clippy::module_name_repetitions)]

pub mod domain;
pub mod ports;
pub mod prelude;

/// Re-export commonly used types
pub use domain::*;
pub use ports::*;
