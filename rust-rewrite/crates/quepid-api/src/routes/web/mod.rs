//! Web Routes
//!
//! Defines all web routes (non-API).

pub mod sessions;
pub mod accounts;
pub mod pages;
pub mod admin;
pub mod dropdown;
pub mod mapper_wizard;
pub mod bulk_judge;
pub mod proxy;

use axum::Router;

/// Create all web routes.
pub fn routes() -> Router {
    Router::new()
        .nest("/users", sessions::routes())
        .nest("/account", accounts::routes())
        .nest("/admin", admin::routes())
        .nest("/dropdown", dropdown::routes())
        .nest("/search_endpoints/mapper_wizard", mapper_wizard::routes())
        .nest("/bulk_judge", bulk_judge::routes())
        .nest("/proxy", proxy::routes())
        .merge(pages::routes())
}
