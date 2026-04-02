//! Favorites endpoints

use axum::{extract::{Query, State}, Json};
use serde::Deserialize;
use crate::AppState;
use crate::models::{ApiResponse, Favorite};

#[derive(Deserialize)]
pub struct ListParams {
    pub user_id: i64,
}

pub async fn list_favorites(
    State(_state): State<AppState>,
    Query(_params): Query<ListParams>,
) -> Json<ApiResponse<Vec<Favorite>>> {
    // TODO: Query from database
    Json(ApiResponse::success(vec![]))
}

#[derive(Deserialize)]
pub struct AddFavoriteRequest {
    pub user_id: i64,
    pub video_id: String,
    pub video_name: String,
    pub video_pic: String,
    pub source_site: String,
}

pub async fn add_favorite(
    State(_state): State<AppState>,
    Json(_req): Json<AddFavoriteRequest>,
) -> Json<ApiResponse<String>> {
    // TODO: Insert into database
    Json(ApiResponse::success("Added".to_string()))
}

#[derive(Deserialize)]
pub struct RemoveFavoriteRequest {
    pub user_id: i64,
    pub video_id: String,
}

pub async fn remove_favorite(
    State(_state): State<AppState>,
    Json(_req): Json<RemoveFavoriteRequest>,
) -> Json<ApiResponse<String>> {
    // TODO: Remove from database
    Json(ApiResponse::success("Removed".to_string()))
}