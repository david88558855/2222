//! MoonTV - Rust implementation
//! Simplified video streaming platform with SQLite storage

mod config;
mod db;
mod models;
mod api;
mod utils;

use std::sync::Arc;
use tokio::sync::Mutex;
use axum::{
    Router,
    routing::{get, post, delete},
    response::IntoResponse,
    http::StatusCode,
};
use tower_http::cors::{CorsLayer, Any};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use rust_embed::RustEmbed;

use crate::config::AppConfig;
use crate::db::Database;

// Embed static files into binary
#[derive(RustEmbed)]
#[folder = "static/"]
struct StaticAssets;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Database>>,
    pub config: AppConfig,
}

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string())
        ))
        .init();

    tracing::info!("MoonTV starting...");

    // Load configuration
    let config = AppConfig::load();
    tracing::info!("Config loaded: {}:{} ", config.host, config.port);

    // Initialize database
    let db = Database::new(&config.db_path).await.expect("Failed to initialize database");
    let db = Arc::new(Mutex::new(db));
    tracing::info!("Database initialized at {}", config.db_path);

    // Create app state
    let state = AppState {
        db,
        config: config.clone(),
    };

    // CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router - API routes first, then static files
    let app = Router::new()
        // API routes
        .route("/api/health", get(api::health::health_check))
        .route("/api/config", get(api::config::get_config))
        .route("/api/search", get(api::search::search))
        .route("/api/detail", get(api::detail::get_detail))
        .route("/api/play", get(api::play::get_play))
        .route("/api/tvbox", get(api::tvbox::serve_tvbox))
        .route("/api/login", post(api::auth::login))
        .route("/api/register", post(api::auth::register))
        .route("/api/logout", post(api::auth::logout))
        .route("/api/favorites", get(api::favorites::list_favorites))
        .route("/api/favorites", post(api::favorites::add_favorite))
        .route("/api/favorites", delete(api::favorites::remove_favorite))
        .route("/api/playrecords", get(api::playrecords::list_records))
        .route("/api/playrecords", post(api::playrecords::add_record))
        .route("/api/user/preferences", get(api::user::get_preferences))
        .route("/api/user/preferences", post(api::user::set_preferences))
        // Admin routes
        .route("/api/admin/users", get(api::admin::list_users))
        .route("/api/admin/users/:id", delete(api::admin::delete_user))
        .route("/api/admin/videos", get(api::admin::list_videos))
        .route("/api/admin/videos/:id", delete(api::admin::delete_video))
        .route("/api/admin/settings", get(api::admin::get_settings))
        .route("/api/admin/settings", post(api::admin::update_settings))
        // Static files (embedded)
        .route("/", get(serve_index))
        .route("/index.html", get(serve_index))
        .route("/admin", get(serve_admin))
        .route("/admin.html", get(serve_admin))
        .route("/static/*path", get(serve_static_file))
        .layer(cors)
        .with_state(state);

    let addr = format!("{}:{} ", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Server listening on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

// Serve embedded static files
async fn serve_index() -> impl IntoResponse {
    serve_embedded("index.html")
}

async fn serve_admin() -> impl IntoResponse {
    serve_embedded("index.html")
}

async fn serve_static_file(path: axum::extract::Path<String>) -> impl IntoResponse {
    serve_embedded(&path.0)
}

fn serve_embedded(path: &str) -> impl IntoResponse {
    match StaticAssets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream().to_string();
            (StatusCode::OK, [("Content-Type", mime.as_str())], content.data.to_vec())
        }
        None => {
            // Try index.html for directory requests
            if path.is_empty() || path.ends_with('/') {
                match StaticAssets::get("index.html") {
                    Some(content) => {
                        (StatusCode::OK, [("Content-Type", "text/html")], content.data.to_vec())
                    }
                    None => (
                        StatusCode::NOT_FOUND,
                        [("Content-Type", "text/html")],
                        "<!DOCTYPE html><html><head><title>404 Not Found</title></head><body><h1>404 - File Not Found</h1></body></html>".as_bytes().to_vec(),
                    )
                }
            } else {
                (
                    StatusCode::NOT_FOUND,
                    [("Content-Type", "text/html")],
                    "<!DOCTYPE html><html><head><title>404 Not Found</title></head><body><h1>404 - File Not Found</h1></body></html>".as_bytes().to_vec(),
                )
            }
        }
    }
}
