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
    extract::State,
};
use tower_http::cors::{CorsLayer, Any};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::AppConfig;
use crate::db::Database;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Database>>,
    pub config: AppConfig,
    pub static_dir: String,
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
    tracing::info!("Config loaded: {}:{}", config.host, config.port);

    // Initialize database
    let db = Database::new(&config.db_path).await.expect("Failed to initialize database");
    let db = Arc::new(Mutex::new(db));
    tracing::info!("Database initialized at {}", config.db_path);

    // Get absolute path for static directory
    let exe_path = std::env::current_exe()
        .expect("Failed to get executable path")
        .parent()
        .expect("Failed to get executable parent")
        .to_path_buf();
    let static_dir = exe_path.join("static").to_string_lossy().to_string();
    tracing::info!("Static files directory: {}", static_dir);

    // Create app state
    let state = AppState {
        db,
        config: config.clone(),
        static_dir,
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
        .route("/api/logout", post(api::auth::logout))
        .route("/api/favorites", get(api::favorites::list_favorites))
        .route("/api/favorites", post(api::favorites::add_favorite))
        .route("/api/favorites", delete(api::favorites::remove_favorite))
        .route("/api/playrecords", get(api::playrecords::list_records))
        .route("/api/playrecords", post(api::playrecords::add_record))
        .route("/api/user/preferences", get(api::user::get_preferences))
        .route("/api/user/preferences", post(api::user::set_preferences))
        // Static files
        .route("/", get(serve_index))
        .route("/index.html", get(serve_index))
        .route("/admin.html", get(serve_admin))
        .route("/static/*path", get(serve_static_file))
        .layer(cors)
        .with_state(state);

    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Server listening on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

async fn serve_index(State(state): State<AppState>) -> impl IntoResponse {
    serve_file(state.static_dir.clone(), "index.html".to_string()).await
}

async fn serve_admin(State(state): State<AppState>) -> impl IntoResponse {
    serve_file(state.static_dir.clone(), "admin.html".to_string()).await
}

async fn serve_static_file(
    State(state): State<AppState>,
    axum::extract::Path(path): axum::extract::Path<String>,
) -> impl IntoResponse {
    serve_file(state.static_dir.clone(), path).await
}

async fn serve_file(static_dir: String, file: String) -> impl IntoResponse {
    let path = std::path::Path::new(&static_dir).join(&file);
    
    // Security check: ensure the path is within static directory
    let static_path = std::path::Path::new(&static_dir).canonicalize().unwrap_or_default();
    let file_path = path.canonicalize().unwrap_or_default();
    
    if !file_path.starts_with(&static_path) {
        return (
            StatusCode::FORBIDDEN,
            [("Content-Type", "text/plain")],
            "Forbidden".as_bytes().to_vec(),
        );
    }
    
    match std::fs::read(&path) {
        Ok(content) => {
            let mime = get_mime_type(&file);
            (
                StatusCode::OK,
                [("Content-Type", mime)],
                content
            )
        }
        Err(_) => (
            StatusCode::NOT_FOUND,
            [("Content-Type", "text/html")],
            "<!DOCTYPE html><html><head><title>404 Not Found</title></head><body><h1>404 - File Not Found</h1></body></html>".as_bytes().to_vec(),
        ),
    }
}

fn get_mime_type(file: &str) -> &'static str {
    if file.ends_with(".css") {
        "text/css"
    } else if file.ends_with(".js") {
        "application/javascript"
    } else if file.ends_with(".html") {
        "text/html"
    } else if file.ends_with(".json") {
        "application/json"
    } else if file.ends_with(".png") {
        "image/png"
    } else if file.ends_with(".jpg") || file.ends_with(".jpeg") {
        "image/jpeg"
    } else if file.ends_with(".svg") {
        "image/svg+xml"
    } else if file.ends_with(".ico") {
        "image/x-icon"
    } else if file.ends_with(".woff") || file.ends_with(".woff2") {
        "font/woff2"
    } else {
        "application/octet-stream"
    }
}