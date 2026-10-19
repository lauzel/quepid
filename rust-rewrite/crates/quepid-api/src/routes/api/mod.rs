//! API Routes
//!
//! Defines all API routes under /api/v1.

pub mod cases;
pub mod queries;
pub mod ratings;
pub mod scores;
pub mod scorers;
pub mod books;
pub mod query_doc_pairs;
pub mod judgements;
pub mod snapshots;
pub mod search_endpoints;
pub mod tries;
pub mod users;
pub mod teams;
pub mod ai_judges;
pub mod annotations;

use axum::Router;

/// Create all API routes.
pub fn routes() -> Router {
    Router::new()
        .nest("/cases", cases::routes())
        .nest("/queries", queries::routes())
        .nest("/ratings", ratings::routes())
        .nest("/scores", scores::routes())
        .nest("/scorers", scorers::routes())
        .nest("/books", books::routes())
        .nest("/query_doc_pairs", query_doc_pairs::routes())
        .nest("/judgements", judgements::routes())
        .nest("/snapshots", snapshots::routes())
        .nest("/search_endpoints", search_endpoints::routes())
        .nest("/tries", tries::routes())
        .nest("/users", users::routes())
        .nest("/teams", teams::routes())
        .nest("/ai_judges", ai_judges::routes())
        .nest("/annotations", annotations::routes())
}
