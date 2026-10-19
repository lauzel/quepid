//! Value Objects
//!
//! Value objects are immutable objects that represent descriptive aspects
//! of a domain with no conceptual identity. They are defined by their attributes.

pub mod email_address;
pub mod password_hash;
pub mod case_name;
pub mod query_text;
pub mod rating_value;
pub mod score_value;
pub mod percentage;
pub mod pagination;
pub mod sort_order;

pub use email_address::*;
pub use password_hash::*;
pub use case_name::*;
pub use query_text::*;
pub use rating_value::*;
pub use score_value::*;
pub use percentage::*;
pub use pagination::*;
pub use sort_order::*;
