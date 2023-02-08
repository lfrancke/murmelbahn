use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{extract, Json};
use metrics::increment_counter;
use murmelbahn_lib::inventory::Inventory;
use std::sync::Arc;

pub async fn buildable(
    State(state): State<Arc<AppState>>,
    Json(inventory): extract::Json<Inventory>,
) -> impl IntoResponse {
    increment_counter!("murmelbahn.buildable.requests");
    let result = state
        .course_repo
        .process_all(&state.sets_repo, inventory)
        .await
        .unwrap();
    Json(result)
}
