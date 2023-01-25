use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn set_list(
) -> impl IntoResponse {

    (StatusCode::OK, "nothing is okay")
}
