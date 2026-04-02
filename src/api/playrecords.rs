//! Play records endpoints

use axum::{extract::{Query, State}, Json};
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::models::{ApiResponse, PlayRecord};

#[derive(Deserialize)]
pub struct ListParams {
    pub user_id: i64,
}

pub async fn list_records(
    State(_state): State<AppState>,
    Query(_params): Query<ListParams>,
) -> Json<ApiResponse<Vec<PlayRecord>>> {
    // TODO: Query from database
    Json(ApiResponse::success(vec![]))
}

#[derive(Deserialize, Serialize)]
pub struct AddRecordRequest {
    pub user_id: i64,
    pub video_id: String,
    pub video_name: String,
    pub episode_index: i32,
    pub position_seconds: i64,
    pub duration_seconds: i64,
    pub source_site: String,
}

pub async fn add_record(
    State(_state): State<AppState>,
    Json(_req): Json<AddRecordRequest>,
) -> Json<ApiResponse<String>> {
    // TODO: Insert or update in database
    Json(ApiResponse::success("Saved".to_string()))
}