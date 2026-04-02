//! Authentication endpoints

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sha2::{Sha256, Digest};
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
    let db = state.db.lock().await;
    
    // Hash the password
    let mut hasher = Sha256::new();
    hasher.update(req.password.as_bytes());
    let password_hash = format!("{:x}", hasher.finalize());
    
    // Check credentials in database
    match db.find_user_by_username(&req.username).await {
        Ok(Some(user)) => {
            if user.password_hash == password_hash {
                let token = Uuid::new_v4().to_string();
                
                // Create session
                if let Err(e) = db.create_session(token.clone(), user.id, 30 * 24 * 3600).await {
                    tracing::error!("Failed to create session: {}", e);
                }
                
                let response = LoginResponse {
                    token,
                    user_id: user.id,
                    username: user.username,
                    role: user.role,
                };
                return Json(ApiResponse::success(response));
            }
        }
        Ok(None) => {}
        Err(e) => {
            tracing::error!("Database error during login: {}", e);
        }
    }

    Json(ApiResponse::error("Invalid credentials"))
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub confirm_password: Option<String>,
}

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Json<ApiResponse<String>> {
    // Validate input
    if req.username.trim().is_empty() {
        return Json(ApiResponse::error("Username cannot be empty"));
    }
    
    if req.password.len() < 6 {
        return Json(ApiResponse::error("Password must be at least 6 characters"));
    }
    
    if let Some(confirm) = &req.confirm_password {
        if &req.password != confirm {
            return Json(ApiResponse::error("Passwords do not match"));
        }
    }
    
    // Hash the password
    let mut hasher = Sha256::new();
    hasher.update(req.password.as_bytes());
    let password_hash = format!("{:x}", hasher.finalize());
    
    let db = state.db.lock().await;
    
    // Check if username already exists
    match db.find_user_by_username(&req.username).await {
        Ok(Some(_)) => {
            return Json(ApiResponse::error("Username already exists"));
        }
        Ok(None) => {}
        Err(e) => {
            tracing::error!("Database error during registration check: {}", e);
            return Json(ApiResponse::error("Database error"));
        }
    }
    
    // Create new user
    match db.create_user(&req.username, &password_hash, "user").await {
        Ok(_) => {
            Json(ApiResponse::success("Registration successful".to_string()))
        }
        Err(e) => {
            tracing::error!("Failed to create user: {}", e);
            Json(ApiResponse::error("Failed to create user"))
        }
    }
}

#[derive(Deserialize)]
pub struct LogoutRequest {
    pub token: String,
}

pub async fn logout(
    State(state): State<AppState>,
    Json(req): Json<LogoutRequest>,
) -> Json<ApiResponse<String>> {
    let db = state.db.lock().await;
    
    // Invalidate session
    if let Err(e) = db.delete_session(&req.token).await {
        tracing::error!("Failed to delete session: {}", e);
    }
    
    Json(ApiResponse::success("Logged out".to_string()))
}