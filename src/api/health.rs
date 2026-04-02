//! Health check endpoint

use axum::{extract::State, Json};
use crate::models::ApiResponse;

pub async fn health_check(
    State(_state): State<crate::AppState>,
) -> Json<ApiResponse<String>> {
    Json(ApiResponse::success("OK".to_string()))
}