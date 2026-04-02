//! Database operations

use rusqlite::Connection;
use std::path::Path;
use std::sync::Arc;

pub struct Database {
    conn: Arc<Connection>,
}

unsafe impl Send for Database {}
unsafe impl Sync for Database {}

impl Database {
    pub async fn new(path: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let db_path = Path::new(path);
        if let Some(parent) = db_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        
        let conn = Connection::open(path)?;
        let db = Database { conn: Arc::new(conn) };
        db.init_tables()?;
        Ok(db)
    }

    fn init_tables(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.conn.execute_batch(
            r#"
            -- Users table
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                role TEXT DEFAULT 'user',
                created_at INTEGER DEFAULT (strftime('%s', 'now'))
            );

            -- Sessions table
            CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY,
                user_id INTEGER,
                created_at INTEGER DEFAULT (strftime('%s', 'now')),
                expires_at INTEGER,
                FOREIGN KEY (user_id) REFERENCES users(id)
            );

            -- Favorites table
            CREATE TABLE IF NOT EXISTS favorites (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER NOT NULL,
                video_id TEXT NOT NULL,
                video_name TEXT,
                video_pic TEXT,
                source_site TEXT,
                created_at INTEGER DEFAULT (strftime('%s', 'now')),
                UNIQUE(user_id, video_id),
                FOREIGN KEY (user_id) REFERENCES users(id)
            );

            -- Play records table
            CREATE TABLE IF NOT EXISTS play_records (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER NOT NULL,
                video_id TEXT NOT NULL,
                video_name TEXT,
                episode_index INTEGER DEFAULT 0,
                position_seconds INTEGER DEFAULT 0,
                duration_seconds INTEGER DEFAULT 0,
                source_site TEXT,
                updated_at INTEGER DEFAULT (strftime('%s', 'now')),
                UNIQUE(user_id, video_id),
                FOREIGN KEY (user_id) REFERENCES users(id)
            );

            -- User preferences
            CREATE TABLE IF NOT EXISTS user_preferences (
                user_id INTEGER PRIMARY KEY,
                theme TEXT DEFAULT 'dark',
                adult_filter_enabled INTEGER DEFAULT 1,
                auto_skip_intro INTEGER DEFAULT 0,
                auto_skip_outro INTEGER DEFAULT 0,
                default_quality TEXT DEFAULT 'auto',
                volume_level INTEGER DEFAULT 100,
                FOREIGN KEY (user_id) REFERENCES users(id)
            );

            -- Video skip settings (intro/outro times)
            CREATE TABLE IF NOT EXISTS video_skip_settings (
                video_id TEXT PRIMARY KEY,
                intro_end INTEGER DEFAULT 0,
                outro_start INTEGER DEFAULT 0
            );

            -- Cache table for API responses
            CREATE TABLE IF NOT EXISTS api_cache (
                key TEXT PRIMARY KEY,
                data TEXT NOT NULL,
                created_at INTEGER DEFAULT (strftime('%s', 'now')),
                expires_at INTEGER NOT NULL
            );

            -- Create indexes
            CREATE INDEX IF NOT EXISTS idx_favorites_user ON favorites(user_id);
            CREATE INDEX IF NOT EXISTS idx_play_records_user ON play_records(user_id);
            CREATE INDEX IF NOT EXISTS idx_sessions_user ON sessions(user_id);
            CREATE INDEX IF NOT EXISTS idx_cache_expires ON api_cache(expires_at);
            "#
        )?;
        Ok(())
    }

    pub fn get_conn(&self) -> &Connection {
        &self.conn
    }
}
