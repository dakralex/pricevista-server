use axum::response::IntoResponse;
use axum::Json;

pub async fn health_handler() -> impl IntoResponse {
    const MESSAGE: &str = "As happy and alive as one could be";

    Json(serde_json::json!({
        "status": "success",
        "message": MESSAGE
    }))
}
