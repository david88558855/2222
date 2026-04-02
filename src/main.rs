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
    response::{IntoResponse, Response},
    http::{StatusCode, header::HeaderName},
    body::Body,
};
use tower_http::cors::{CorsLayer, Any};
use tower_http::fs::ServeFileSystem;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::AppConfig;
use crate::db::Database;

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
    tracing::info!("Config loaded: {}:{}", config.host, config.port);

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

    // Build router
    let app = Router::new()
        .route("/", get(serve_index))
        .route("/index.html", get(serve_index))
        .route("/admin.html", get(serve_admin))
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
        .route("/static/:file", get(serve_static))
        .layer(cors)
        .with_state(state);

    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Server listening on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

async fn serve_index() -> impl IntoResponse {
    redirect("/index.html")
}

async fn serve_admin() -> impl IntoResponse {
    redirect("/index.html")
}

async fn serve_static(
    axum::extract::Path(file): axum::extract::Path<String>,
) -> impl IntoResponse {
    let path = format!("static/{}", file);
    match std::fs::read(&path) {
        Ok(content) => {
            let mime = if file.ends_with(".css") {
                "text/css"
            } else if file.ends_with(".js") {
                "application/javascript"
            } else if file.ends_with(".html") {
                "text/html"
            } else if file.ends_with(".png") {
                "image/png"
            } else if file.ends_with(".jpg") || file.ends_with(".jpeg") {
                "image/jpeg"
            } else {
                "application/octet-stream"
            };
            (
                StatusCode::OK,
                [(HeaderName::from_static("content-type"), mime)],
                content
            )
        }
        Err(_) => (
            StatusCode::NOT_FOUND,
            "Not found"
        ),
    }
}

fn redirect(uri: &str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header("Location", uri)
        .body(Body::empty())
        .unwrap()
}