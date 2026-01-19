mod database;
mod keyboard;
mod models;
mod window;

use database::Database;
use models::{DailyStats, InputRecord, SearchFilter, Settings};
use once_cell::sync::OnceCell;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::Manager;

static DATABASE: OnceCell<Arc<Database>> = OnceCell::new();

fn get_db() -> &'static Arc<Database> {
    DATABASE.get().expect("Database not initialized")
}

#[tauri::command]
fn get_records(filter: SearchFilter) -> Result<Vec<InputRecord>, String> {
    get_db()
        .get_records(&filter)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_today_stats() -> Result<DailyStats, String> {
    get_db()
        .get_today_stats()
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_app_list() -> Result<Vec<String>, String> {
    get_db()
        .get_app_list()
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_old_records(before_date: String) -> Result<usize, String> {
    get_db()
        .delete_records_before(&before_date)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_settings() -> Result<Settings, String> {
    let db = get_db();

    let excluded_apps: Vec<String> = db
        .get_setting("excluded_apps")
        .map_err(|e| e.to_string())?
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();

    let merge_interval_ms: u64 = db
        .get_setting("merge_interval_ms")
        .map_err(|e| e.to_string())?
        .and_then(|s| s.parse().ok())
        .unwrap_or(500);

    let auto_start: bool = db
        .get_setting("auto_start")
        .map_err(|e| e.to_string())?
        .and_then(|s| s.parse().ok())
        .unwrap_or(false);

    Ok(Settings {
        excluded_apps,
        merge_interval_ms,
        auto_start,
    })
}

#[tauri::command]
fn save_settings(settings: Settings) -> Result<(), String> {
    let db = get_db();

    let excluded_json = serde_json::to_string(&settings.excluded_apps)
        .map_err(|e| e.to_string())?;
    db.set_setting("excluded_apps", &excluded_json)
        .map_err(|e| e.to_string())?;

    db.set_setting("merge_interval_ms", &settings.merge_interval_ms.to_string())
        .map_err(|e| e.to_string())?;

    db.set_setting("auto_start", &settings.auto_start.to_string())
        .map_err(|e| e.to_string())?;

    // Update excluded apps in keyboard listener
    keyboard::set_excluded_apps(settings.excluded_apps);

    Ok(())
}

#[tauri::command]
fn export_records(filter: SearchFilter) -> Result<String, String> {
    let records = get_db()
        .get_records(&filter)
        .map_err(|e| e.to_string())?;

    serde_json::to_string_pretty(&records)
        .map_err(|e| e.to_string())
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Initialize database
            let app_dir = app.path().app_data_dir().expect("Failed to get app data dir");
            std::fs::create_dir_all(&app_dir).expect("Failed to create app data dir");
            let db_path: PathBuf = app_dir.join("arkinput.db");

            let db = Arc::new(
                Database::new(db_path).expect("Failed to initialize database")
            );

            if DATABASE.set(db.clone()).is_err() {
                panic!("Failed to set database");
            }

            // Initialize keyboard listener with database
            keyboard::init_database(db);

            // Load settings and apply excluded apps
            if let Ok(settings) = get_settings() {
                keyboard::set_excluded_apps(settings.excluded_apps);
            }

            // Start keyboard listener
            keyboard::start_keyboard_listener();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_records,
            get_today_stats,
            get_app_list,
            delete_old_records,
            get_settings,
            save_settings,
            export_records,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
