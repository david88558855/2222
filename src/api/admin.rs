//! Admin API endpoints

use axum::{extract::State, Json, extract::Path};
use serde::{Deserialize, Serialize};
use crate::{AppState, models::ApiResponse};

// List all users
#[derive(Serialize)]
pub struct UserInfo {
    pub id: i64,
    pub username: String,
    pub role: String,
    pub created_at: String,
}

pub async fn list_users(
    _state: State<AppState>,
) -> Json<ApiResponse<Vec<UserInfo>>> {
    let db = _state.db.lock().await;
    
    match db.list_all_users().await {
        Ok(users) => {
            let user_infos: Vec<UserInfo> = users.into_iter().map(|u| UserInfo {
                id: u.id,
                username: u.username.clone(),
                role: u.role.clone(),
                created_at: u.created_at.to_string(),
            }).collect();
            Json(ApiResponse::success(user_infos))
        }
        Err(e) => Json(ApiResponse::error(&format!("Failed to list users: {}", e))),
    }
}

// Delete a user
pub async fn delete_user(
    _state: State<AppState>,
    Path(id): Path<i64>,
) -> Json<ApiResponse<String>> {
    let db = _state.db.lock().await;
    
    match db.delete_user_by_id(id).await {
        Ok(_) => Json(ApiResponse::success("User deleted".to_string())),
        Err(e) => Json(ApiResponse::error(&format!("Failed to delete user: {}", e))),
    }
}

// List all videos (cached/search results)
#[derive(Serialize)]
pub struct VideoInfo {
    pub id: String,
    pub name: String,
    pub source_site: String,
    pub cached_at: Option<String>,
}

pub async fn list_videos(
    State(_state): State<AppState>,
) -> Json<ApiResponse<Vec<VideoInfo>>> {
    // TODO: Implement video listing from cache/database
    Json(ApiResponse::success(vec![]))
}

// Delete a video
pub async fn delete_video(
    State(_state): State<AppState>,
    Path(_id): Path<String>,
) -> Json<ApiResponse<String>> {
    // TODO: Implement video deletion
    Json(ApiResponse::success("Video deleted".to_string()))
}

// Get system settings
#[derive(Serialize)]
pub struct SystemSettings {
    pub site_name: String,
    pub allow_register: bool,
    pub default_role: String,
    pub max_search_results: i32,
}

pub async fn get_settings(
    State(state): State<AppState>,
) -> Json<ApiResponse<SystemSettings>> {
    let settings = SystemSettings {
        site_name: "MoonTV".to_string(),
        allow_register: true,
        default_role: "user".to_string(),
        max_search_results: 20,
    };
    Json(ApiResponse::success(settings))
}

// Update system settings
#[derive(Deserialize)]
pub struct UpdateSettingsRequest {
    pub site_name: Option<String>,
    pub allow_register: Option<bool>,
    pub max_search_results: Option<i32>,
}

pub async fn update_settings(
    _state: State<AppState>,
    Json(req): Json<UpdateSettingsRequest>,
) -> Json<ApiResponse<SystemSettings>> {
    // TODO: Persist settings to database
    let settings = SystemSettings {
        site_name: req.site_name.unwrap_or_else(|| "MoonTV".to_string()),
        allow_register: req.allow_register.unwrap_or(true),
        default_role: "user".to_string(),
        max_search_results: req.max_search_results.unwrap_or(20),
    };
    Json(ApiResponse::success(settings))
}
