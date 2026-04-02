//! Search endpoint

use axum::{extract::{State, Query}, Json};
use serde::Deserialize;
use crate::AppState;
use crate::models::{ApiResponse, SearchResult};

#[derive(Deserialize, Default)]
pub struct SearchParams {
    pub keyword: Option<String>,
    #[serde(default = "default_page")]
    pub page: usize,
    #[serde(default = "default_page_size")]
    pub page_size: usize,
    #[serde(default)]
    pub site: Option<String>,
}

fn default_page() -> usize { 1 }
fn default_page_size() -> usize { 20 }

pub async fn search(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Json<ApiResponse<SearchResult>> {
    let keyword = params.keyword.unwrap_or_default();
    
    if keyword.is_empty() {
        return Json(ApiResponse::error("keyword is required"));
    }

    // TODO: Query external video sources based on config.api_site
    // For now, return empty results
    let result = SearchResult {
        list: vec![],
        total: 0,
        page: params.page,
        page_size: params.page_size,
    };

    Json(ApiResponse::success(result))
}