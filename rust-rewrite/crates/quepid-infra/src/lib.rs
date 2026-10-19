//! Quepid Infrastructure Layer
//!
//! This crate provides implementations for the domain layer's ports (interfaces).
//! It contains:
//!
//! - Database access via SeaORM
//! - External service adapters (search engines, LLM judges, email)
//! - Background job processing
//! - WebSocket server
//!
//! # Architecture
//!
//! This layer implements the traits defined in `quepid-core::ports`.
//! It depends on `quepid-core` but not vice versa.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![allow(clippy::module_name_repetitions)]

pub mod database;
pub mod external;
pub mod jobs;
pub mod websocket;
pub mod config;

pub use database::*;
pub use external::*;
pub use jobs::*;
pub use websocket::*;
pub use config::*;
