use std::fs;

use database::db::Database;
use sqlx::SqlitePool;
use tauri::{async_runtime, Manager};
use tauri_plugin_fs::FsExt;

mod database {
    pub mod db;
}

mod commands {
    pub mod crud;
}

/// Global state of the application
pub struct AppState {
    pub db: Database,
}

impl AppState {
    pub fn new(db: Database) -> Self {
        AppState { db }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let scope = app.fs_scope();
            let app_dir = app
                .path()
                .app_local_data_dir()
                .expect("failed to get app dir");
            scope
                .allow_directory(&app_dir, true)
                .expect("failed to allow app dir");
            fs::create_dir_all(&app_dir).expect("failed to create app dir");
            let db_path = app_dir.join("database.db");
            if !db_path.exists() {
                std::fs::File::create(&db_path).expect("failed to create database file");
            }
            let db_url = format!("sqlite://{}", db_path.to_string_lossy());
            async_runtime::block_on(async {
                let pool = SqlitePool::connect(&db_url).await.unwrap_or_else(|e| {
                    eprintln!("SQLx connection error: {:?}", e);
                    std::process::exit(1);
                });
                let db = Database::new(pool);
                db.run_migrations().await.unwrap();
                app.manage(AppState::new(db));
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
