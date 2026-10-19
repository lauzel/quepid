//! Quepid API Library
//!
//! This crate provides the HTTP API server for Quepid.
//! It depends on `quepid-core` and `quepid-infra`.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![allow(clippy::module_name_repetitions)]

pub mod app;
pub mod config;
pub mod handlers;
pub mod middleware;
pub mod routes;

pub use app::*;
pub use config::*;
pub use handlers::*;
pub use middleware::*;
pub use routes::*;
