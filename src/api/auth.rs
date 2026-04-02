//! Authentication endpoints

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::AppState;
use crate::models::ApiResponse;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: i64,
    pub username: String,
    pub role: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Json<ApiResponse<LoginResponse>> {
    // Check credentials
    if req.username == state.config.username && req.password == state.config.password {
        let token = Uuid::new_v4().to_string();
        let response = LoginResponse {
            token,
            user_id: 1,
            username: req.username,
            role: "admin".to_string(),
        };
        return Json(ApiResponse::success(response));
    }

    Json(ApiResponse::error("Invalid credentials"))
}

#[derive(Deserialize)]
pub struct LogoutRequest {
    pub token: String,
}

pub async fn logout(
    State(_state): State<AppState>,
    Json(_req): Json<LogoutRequest>,
) -> Json<ApiResponse<String>> {
    // TODO: Invalidate session in database
    Json(ApiResponse::success("Logged out".to_string()))
}