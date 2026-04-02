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
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
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

    // List all users (admin function)
    pub async fn list_all_users(&self) -> Result<Vec<crate::models::User>, rusqlite::Error> {
        let mut stmt = self.conn.prepare("SELECT id, username, role, created_at FROM users ORDER BY id")?;
        let user_iter = stmt.query_map([], |row| {
            Ok(crate::models::User {
                id: row.get(0)?,
                username: row.get(1)?,
                password_hash: String::new(), // Don't return password hash
                role: row.get(2)?,
                created_at: row.get::<_, i64>(3).map(|ts| ts.to_string()).unwrap_or_default(),
            })
        })?;

        let mut users = Vec::new();
        for user in user_iter {
            if let Ok(u) = user {
                users.push(u);
            }
        }
        Ok(users)
    }

    // Delete user by ID (admin function)
    pub async fn delete_user_by_id(&self, id: i64) -> Result<(), rusqlite::Error> {
        self.conn.execute("DELETE FROM users WHERE id = ?", [id])?;
        Ok(())
    }

    // Find user by username
    pub async fn find_user_by_username(&self, username: &str) -> Result<Option<crate::models::User>, rusqlite::Error> {
        let mut stmt = self.conn.prepare("SELECT id, username, password_hash, role, created_at FROM users WHERE username = ?")?;
        let mut rows = stmt.query([username])?;
        
        if let Some(row) = rows.next()? {
            Ok(Some(crate::models::User {
                id: row.get(0)?,
                username: row.get(1)?,
                password_hash: row.get(2)?,
                role: row.get(3)?,
                created_at: row.get::<_, i64>(4).map(|ts| ts.to_string()).unwrap_or_default(),
            }))
        } else {
            Ok(None)
        }
    }

    // Create new user
    pub async fn create_user(&self, username: &str, password_hash: &str, role: &str) -> Result<i64, rusqlite::Error> {
        let mut stmt = self.conn.prepare("INSERT INTO users (username, password_hash, role) VALUES (?, ?, ?)")?;
        let id = stmt.insert((username, password_hash, role))?;
        
        // Create default preferences for new user
        let mut prefs_stmt = self.conn.prepare("INSERT OR IGNORE INTO user_preferences (user_id) VALUES (?)")?;
        let _ = prefs_stmt.insert([id]);
        
        Ok(id)
    }

    // Create session
    pub async fn create_session(&self, token: String, user_id: i64, expires_in_secs: i64) -> Result<(), rusqlite::Error> {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let expires_at = now + expires_in_secs;
        
        let mut stmt = self.conn.prepare("INSERT INTO sessions (id, user_id, expires_at) VALUES (?, ?, ?)")?;
        stmt.insert((token, user_id, expires_at))?;
        Ok(())
    }

    // Delete session
    pub async fn delete_session(&self, token: &str) -> Result<(), rusqlite::Error> {
        self.conn.execute("DELETE FROM sessions WHERE id = ?", [token])?;
        Ok(())
    }
}