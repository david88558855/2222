//! Configuration module for MoonTV

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    // Server
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,

    // Database
    #[serde(default = "default_db_path")]
    pub db_path: String,

    // Admin
    #[serde(default = "default_username")]
    pub username: String,
    #[serde(default = "default_password")]
    pub password: String,

    // Site config
    #[serde(default)]
    pub cache_time: u64,
    #[serde(default)]
    pub api_site: std::collections::HashMap<String, ApiSite>,
}

fn default_host() -> String { "0.0.0.0".to_string() }
fn default_port() -> u16 { 3000 }
fn default_db_path() -> String { "moontv.db".to_string() }
fn default_username() -> String { "admin".to_string() }
fn default_password() -> String { "admin".to_string() }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiSite {
    pub api: String,
    pub name: String,
    #[serde(default)]
    pub detail: String,
    #[serde(default)]
    pub is_adult: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        let mut api_site = std::collections::HashMap::new();
        api_site.insert("regular_site".to_string(), ApiSite {
            api: "".to_string(),
            name: "Regular Videos".to_string(),
            detail: "".to_string(),
            is_adult: false,
        });
        
        AppConfig {
            host: default_host(),
            port: default_port(),
            db_path: default_db_path(),
            username: default_username(),
            password: default_password(),
            cache_time: 7200,
            api_site,
        }
    }
}

impl AppConfig {
    pub fn load() -> Self {
        // Try to load from config.json
        if let Ok(content) = fs::read_to_string("config.json") {
            if let Ok(config) = serde_json::from_str(&content) {
                return config;
            }
        }
        
        // Try to load from config dir
        let config_path = Self::config_path();
        if let Ok(content) = fs::read_to_string(&config_path) {
            if let Ok(config) = serde_json::from_str(&content) {
                return config;
            }
        }
        
        tracing::warn!("No config found, using defaults");
        Self::default()
    }

    fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("moontv")
            .join("config.json")
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write("config.json", content)?;
        Ok(())
    }
}