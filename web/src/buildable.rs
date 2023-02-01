use std::sync::Arc;
use axum::extract::State;
use axum::{extract, Json};
use axum::response::IntoResponse;
use murmelbahn_lib::inventory::Inventory;
use crate::AppState;

pub async fn buildable(
    State(state): State<Arc<AppState>>,
    Json(inventory): extract::Json<Inventory>,
) -> impl IntoResponse {
    let result = state.course_repo.process_all(&state.sets_repo, inventory).await.unwrap();
    Json(result)
}
