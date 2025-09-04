use axum::{Json, extract::Path, response::IntoResponse};
use reqwest::StatusCode;
use serde_json::json;

use crate::services::xray;

#[axum::debug_handler]
pub async fn get_xray_status() -> impl IntoResponse {
    let status_result = xray::get_xray_status();

    (StatusCode::OK, Json(status_result)).into_response()
}

#[axum::debug_handler]
pub async fn toggle_xray(Path(action): Path<String>) -> impl IntoResponse {
    let response = match action.as_str() {
        "on" => match xray::start_xray().await {
            Ok(_) => (
                StatusCode::OK,
                Json(json!({"status": "Xray started successfully"})),
            ),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to start Xray: {}", err)})),
            ),
        },
        "off" => match xray::stop_xray().await {
            Ok(_) => (
                StatusCode::OK,
                Json(json!({"status": "Xray stopped successfully"})),
            ),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to stop Xray: {}", err)})),
            ),
        },
        "restart" => match xray::restart_xray().await {
            Ok(_) => (
                StatusCode::OK,
                Json(json!({"status": "Xray restarted successfully"})),
            ),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to restart Xray: {}", err)})),
            ),
        },
        _ => (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": format!("Unknown action: {}", action)})),
        ),
    };

    response.into_response()
}
