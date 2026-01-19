use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputRecord {
    pub id: Option<i64>,
    pub timestamp: DateTime<Utc>,
    pub app_name: String,
    pub window_title: Option<String>,
    pub content: String,
    pub key_count: i32,
    pub created_at: Option<DateTime<Utc>>,
}

impl InputRecord {
    pub fn new(app_name: String, window_title: Option<String>, content: String) -> Self {
        Self {
            id: None,
            timestamp: Utc::now(),
            app_name,
            window_title,
            content,
            key_count: 1,
            created_at: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppStats {
    pub app_name: String,
    pub key_count: i64,
    pub record_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyStats {
    pub date: String,
    pub total_keys: i64,
    pub total_records: i64,
    pub app_stats: Vec<AppStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilter {
    pub query: Option<String>,
    pub app_name: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl Default for SearchFilter {
    fn default() -> Self {
        Self {
            query: None,
            app_name: None,
            start_date: None,
            end_date: None,
            limit: Some(100),
            offset: Some(0),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Settings {
    pub excluded_apps: Vec<String>,
    pub merge_interval_ms: u64,
    pub auto_start: bool,
}

impl Settings {
    pub fn default_settings() -> Self {
        Self {
            excluded_apps: vec![],
            merge_interval_ms: 500,
            auto_start: false,
        }
    }
}
