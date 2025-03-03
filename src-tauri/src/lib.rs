use std::fs;
use tokio::sync::Mutex;
use database::db::Database;
use sqlx::SqlitePool;
use tauri::{async_runtime, Manager};
use tauri_plugin_fs::FsExt;

mod database {
    pub mod db;
    pub mod models;
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
                app.manage(Mutex::new(AppState::new(db)));
            });
            Ok(())
        })
            .invoke_handler(tauri::generate_handler![
                commands::crud::insert_student,
                commands::crud::get_students,
                commands::crud::update_student,
                commands::crud::delete_student,
                commands::crud::insert_class,
                commands::crud::get_classes,
                commands::crud::update_class,
                commands::crud::delete_class,
                commands::crud::insert_assignment,
                commands::crud::get_assignments,
                commands::crud::update_assignment,
                commands::crud::delete_assignment,
                commands::crud::insert_grade,
                commands::crud::get_grades,
                commands::crud::update_grade,
                commands::crud::delete_grade,
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
