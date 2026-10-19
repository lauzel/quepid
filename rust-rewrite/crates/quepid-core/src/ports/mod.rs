//! Ports Module
//!
//! Defines interfaces (traits) that the domain layer expects from infrastructure.
//! These are implemented in the `quepid-infra` crate.
//!
//! # Ports and Adapters Pattern
//!
//! - **Ports** (this module): Interfaces defined by the domain
//! - **Adapters** (infra): Implementations that adapt external systems to ports
//!
//! This allows the domain to depend on abstractions, not concrete implementations.

pub mod repositories;
pub mod services;

pub use repositories::*;
pub use services::*;
