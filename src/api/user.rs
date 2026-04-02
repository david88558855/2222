//! User preferences endpoints

use axum::{extract::{Query, State}, Json};
use serde::Deserialize;
use crate::AppState;
use crate::models::{ApiResponse, UserPreferences};

#[derive(Deserialize)]
pub struct GetParams {
    pub user_id: i64,
}

pub async fn get_preferences(
    State(_state): State<AppState>,
    Query(_params): Query<GetParams>,
) -> Json<ApiResponse<UserPreferences>> {
    let prefs = UserPreferences {
        theme: "dark".to_string(),
        adult_filter_enabled: true,
        auto_skip_intro: false,
        auto_skip_outro: false,
        default_quality: "auto".to_string(),
        volume_level: 100,
    };
    Json(ApiResponse::success(prefs))
}

#[derive(Deserialize)]
pub struct SetPreferencesRequest {
    pub user_id: i64,
    pub preferences: UserPreferences,
}

pub async fn set_preferences(
    State(_state): State<AppState>,
    Json(_req): Json<SetPreferencesRequest>,
) -> Json<ApiResponse<String>> {
    // TODO: Update in database
    Json(ApiResponse::success("Saved".to_string()))
}