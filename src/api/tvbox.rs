//! TVBox compatible endpoint

use axum::{extract::{Query, State}, Json};
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::models::ApiResponse;

#[derive(Deserialize)]
pub struct TvBoxParams {
    #[serde(default = "default_format")]
    pub format: String,
}

fn default_format() -> String { "json".to_string() }

#[derive(Serialize)]
pub struct TvBoxConfig {
    pub spider: String,
    pub sites: Vec<TvBoxSite>,
    pub lives: Vec<TvBoxLive>,
    pub ijk: Vec<TvBoxIjk>,
}

#[derive(Serialize)]
pub struct TvBoxSite {
    pub key: String,
    pub name: String,
    #[serde(rename = "type")]
    pub site_type: i32,
    pub api: String,
    pub searchable: i32,
    pub quickSearch: i32,
    pub filterable: i32,
}

#[derive(Serialize)]
pub struct TvBoxLive {
    pub name: String,
    #[serde(rename = "type")]
    pub live_type: i32,
    pub url: String,
    pub epg_url: Option<String>,
}

#[derive(Serialize)]
pub struct TvBoxIjk {
    pub group: String,
    pub options: Vec<String>,
}

pub async fn serve_tvbox(
    State(state): State<AppState>,
    Query(params): Query<TvBoxParams>,
) -> impl axum::response::IntoResponse {
    let base_url = format!("http://{}:{}", state.config.host, state.config.port);
    
    let mut sites: Vec<TvBoxSite> = vec![];
    
    for (key, site) in &state.config.api_site {
        sites.push(TvBoxSite {
            key: key.clone(),
            name: site.name.clone(),
            site_type: 1,
            api: site.api.clone(),
            searchable: 1,
            quickSearch: 1,
            filterable: 1,
        });
    }
    
    let config = TvBoxConfig {
        spider: format!("{}/api/spider.js", base_url),
        sites,
        lives: vec![TvBoxLive {
            name: "Live".to_string(),
            live_type: 0,
            url: "".to_string(),
            epg_url: None,
        }],
        ijk: vec![],
    };

    match params.format.as_str() {
        "txt" => axum::response::IntoResponse::into_response(
            axum::http::StatusCode::OK,
            [("Content-Type", "text/plain; charset=utf-8")],
            serde_json::to_string(&config).unwrap_or_default()
        ),
        "json" => axum::response::IntoResponse::into_response(
            axum::http::StatusCode::OK,
            [("Content-Type", "application/json; charset=utf-8")],
            serde_json::to_string(&config).unwrap_or_default()
        ),
        _ => axum::response::IntoResponse::into_response(
            axum::http::StatusCode::BAD_REQUEST,
            "Invalid format"
        ),
    }
}