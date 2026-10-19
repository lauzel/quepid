//! Repository Ports
//!
//! Defines repository traits (interfaces) that the domain layer expects.
//! These are implemented in the `quepid-infra` crate.

pub mod user_repository;
pub mod team_repository;
pub mod case_repository;
pub mod query_repository;
pub mod rating_repository;
pub mod score_repository;
pub mod scorer_repository;
pub mod try_repository;
pub mod search_endpoint_repository;
pub mod snapshot_repository;
pub mod book_repository;
pub mod query_doc_pair_repository;
pub mod judgement_repository;
pub mod annotation_repository;
pub mod api_key_repository;
pub mod announcement_repository;
pub mod ai_judge_repository;

pub use user_repository::*;
pub use team_repository::*;
pub use case_repository::*;
pub use query_repository::*;
pub use rating_repository::*;
pub use score_repository::*;
pub use scorer_repository::*;
pub use try_repository::*;
pub use search_endpoint_repository::*;
pub use snapshot_repository::*;
pub use book_repository::*;
pub use query_doc_pair_repository::*;
pub use judgement_repository::*;
pub use annotation_repository::*;
pub use api_key_repository::*;
pub use announcement_repository::*;
pub use ai_judge_repository::*;
