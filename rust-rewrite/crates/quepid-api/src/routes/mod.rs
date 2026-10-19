//! API Routes
//!
//! Defines all HTTP routes for the API.

pub mod api;
pub mod web;
pub mod health;

pub use api::*;
pub use web::*;
pub use health::*;
