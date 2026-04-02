//! Detail endpoint

use axum::{extract::{Query, State}, Json};
use serde::Deserialize;
use crate::AppState;
use crate::models::{ApiResponse, VideoDetail, Episode};

#[derive(Deserialize)]
pub struct DetailParams {
    pub id: String,
    #[serde(default)]
    pub site: Option<String>,
}

pub async fn get_detail(
    State(_state): State<AppState>,
    Query(params): Query<DetailParams>,
) -> Json<ApiResponse<VideoDetail>> {
    // TODO: Fetch video details from external API
    let detail = VideoDetail {
        id: params.id,
        name: "Sample Video".to_string(),
        pic: "".to_string(),
        detail: "".to_string(),
        source_site: params.site.unwrap_or_default(),
        episodes: vec![],
    };

    Json(ApiResponse::success(detail))
}