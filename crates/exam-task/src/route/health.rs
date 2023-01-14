use axum::{response::IntoResponse, routing::get, Router};

pub fn health() -> Router {
    async fn handler() -> impl IntoResponse {
        "Healthy"
    }

    Router::new().route("/health", get(handler))
}
