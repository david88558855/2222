//! Play endpoint

use axum::{extract::{Query, State}, Json};
use serde::Deserialize;
use crate::AppState;
use crate::models::ApiResponse;

#[derive(Deserialize)]
pub struct PlayParams {
    pub id: String,
    pub episode: Option<usize>,
    #[serde(default)]
    pub site: Option<String>,
}

#[derive(serde::Serialize)]
pub struct PlayResponse {
    pub url: String,
    pub play_url: String,
    pub intro_end: u64,
    pub outro_start: u64,
}

pub async fn get_play(
    State(_state): State<AppState>,
    Query(_params): Query<PlayParams>,
) -> Json<ApiResponse<PlayResponse>> {
    // TODO: Fetch play URL from video source
    let response = PlayResponse {
        url: "".to_string(),
        play_url: "".to_string(),
        intro_end: 0,
        outro_start: 0,
    };

    Json(ApiResponse::success(response))
}