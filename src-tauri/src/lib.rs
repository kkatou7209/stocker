use std::fs::File;
use std::path::Path;
use std::{env, fs};

use tauri::Manager;

use crate::persistence::sqlite::migrate;

mod core;
mod persistence;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

const DB_NAME: &str = "app.db";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let db_path = if tauri::is_dev() {
                let current = env::current_dir()?;
                let path = current.join("db");

                if !path.exists() {
                    fs::create_dir_all(&path)?;
                }

                path.join(DB_NAME)
            } else {
                app.path().app_data_dir()?.join(DB_NAME)
            };

            let db_path = db_path.as_path();

            migrate(db_path.to_string_lossy())?;

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
