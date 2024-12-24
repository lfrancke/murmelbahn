use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use metrics::counter;
use murmelbahn_lib::physical::Inventory;
use std::sync::Arc;
use tracing::info;

/// This returns a list of all codes that are buildable with the inventory that is passed in.
/// At the moment, this returns only a list of strings
pub async fn buildable(
    State(state): State<Arc<AppState>>,
    Json(inventory): Json<Inventory>,
) -> impl IntoResponse {
    counter!("murmelbahn.buildable.requests").increment(1);
    let result = state
        .course_repo
        .process_all(&state.sets_repo, inventory)
        .await;

    match result {
        Ok(result) => Json(result).into_response(),
        Err(e) => {
            info!("Error assembling 'buildable' request", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
        },
    }
}
