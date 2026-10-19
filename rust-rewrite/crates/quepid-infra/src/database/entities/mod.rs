//! Database Entities
//!
//! SeaORM entity definitions generated from the database schema.
//!
//! These are typically generated using `sea-orm-cli` but can also be
//! written manually for more control.

// Generated entities will be placed here
// For now, we'll create them manually to match the Ruby models

pub mod user;
pub mod team;
pub mod case;
pub mod query;
pub mod rating;
pub mod score;
pub mod scorer;
pub mod try_;
pub mod search_endpoint;
pub mod snapshot;
pub mod book;
pub mod query_doc_pair;
pub mod judgement;
pub mod annotation;
pub mod api_key;
pub mod announcement;
pub mod ai_judge;

pub use user::*;
pub use team::*;
pub use case::*;
pub use query::*;
pub use rating::*;
pub use score::*;
pub use scorer::*;
pub use try_::*;
pub use search_endpoint::*;
pub use snapshot::*;
pub use book::*;
pub use query_doc_pair::*;
pub use judgement::*;
pub use annotation::*;
pub use api_key::*;
pub use announcement::*;
pub use ai_judge::*;
