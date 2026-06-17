use crate::AppState;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use std::sync::Arc;

pub async fn set_list(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Json(state.sets_repo.sets.clone())
}
