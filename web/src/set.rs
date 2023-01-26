use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use crate::AppState;

pub async fn set_list(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    Json(state.sets_repo.sets.clone())
}
