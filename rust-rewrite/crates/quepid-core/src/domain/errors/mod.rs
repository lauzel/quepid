//! Domain Errors
//!
//! Custom error types for the domain layer.

pub mod authentication;
pub mod authorization;
pub mod validation;
pub mod not_found;
pub mod conflict;
pub mod domain_error;

pub use authentication::*;
pub use authorization::*;
pub use validation::*;
pub use not_found::*;
pub use conflict::*;
pub use domain_error::*;
