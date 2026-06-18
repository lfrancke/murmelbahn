mod buildable;
mod course;
mod set;

use crate::AppState;
use axum::Router;
use axum::routing::{get, post};
use std::sync::Arc;

use buildable::buildable;
use course::{course_bom, course_dump, course_raw_download};
use set::set_list;

/// Builds the `/api` router (mounted with `nest("/api", ...)` in main).
pub fn router(state: Arc<AppState>) -> Router {
    let course_routes = Router::new()
        .route("/{id}/dump", get(course_dump))
        .route("/{id}/bom", get(course_bom))
        .route("/{id}/raw", get(course_raw_download))
        .with_state(state.clone());

    let set_routes = Router::new()
        .route("/list", get(set_list))
        .with_state(state.clone());

    Router::new()
        .route("/buildable", post(buildable))
        .with_state(state)
        .nest("/course", course_routes)
        .nest("/set", set_routes)
}
