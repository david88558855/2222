//! Config endpoint

use axum::{extract::State, Json};
use crate::AppState;
use crate::models::ApiResponse;

pub async fn get_config(
    State(state): State<AppState>,
) -> Json<ApiResponse<serde_json::Value>> {
    let config = serde_json::json!({
        "cache_time": state.config.cache_time,
        "api_site": state.config.api_site,
        "enable_register": true,
    });
    Json(ApiResponse::success(config))
}