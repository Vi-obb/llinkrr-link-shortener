use axum::http::StatusCode;
use axum::response::IntoResponse;


// basic function for health check
pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, "All Systems Are Go!")
}

