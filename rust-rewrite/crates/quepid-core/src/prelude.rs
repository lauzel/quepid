//! Prelude Module
//!
//! Re-exports commonly used types and traits from this crate.
//!
//! Use this to reduce imports in downstream crates:
//!
//! ```rust
//! use quepid_core::prelude::*;
//! ```

pub use crate::domain::{
    entities::*,
    errors::*,
    services::*,
    value_objects::*,
};

pub use crate::ports::*;
