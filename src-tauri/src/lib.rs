mod command;
mod config;
mod core;
mod persistence;

use std::{env, fs};

use chrono::Local;
use tauri::Manager;
use tauri_plugin_log::log::Level;
use tauri_plugin_log::{log, Target, TargetKind};

use crate::command::*;
use crate::core::stocker::{Ports, Stocker};
use crate::persistence::sqlite::*;

const DB_NAME: &str = "stocker.db";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_process::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .build(),
        )
        .setup(|app| {
            // database path
            let db_path = if tauri::is_dev() {
                env::current_dir()?.join("data")
            } else {
                app.path().app_data_dir()?.join("data")
            };

            if !db_path.exists() {
                fs::create_dir_all(&db_path)?;
            }

            let db_path = db_path.join(DB_NAME);

            // Migrate database
            migrate(db_path.to_string_lossy())?;

            // Plug Stocker with SQLite implementations
            let stocker = Stocker::plug(Ports {
                for_supply_persistence: SqliteSupplyRepository::new(db_path.to_string_lossy()),
                for_supplier_persistence: SqliteSupplierRepository::new(db_path.to_string_lossy()),
                for_journal_persistence: SqliteJournalRepository::new(db_path.to_string_lossy()),
                for_stocktaking_persistence: SqliteStocktakingRepository::new(
                    db_path.to_string_lossy(),
                ),
            });

            // Register application core to state manager
            app.manage(stocker);

            Ok(())
        })
        .plugin(
            tauri_plugin_log::Builder::new()
                // Logs only errors
                .level(log::LevelFilter::Error)
                // Use local timezone for log timestamps
                .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
                // Set max file size to 10 KB
                .max_file_size(10_000)
                .targets([
                    // Error log file named with date
                    Target::new(TargetKind::LogDir {
                        file_name: Some(format!("error-{}", Local::now().format("%Y-%m-%d"))),
                    })
                    .filter(|meta| meta.level() == Level::Error),
                ])
                .build(),
        )
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            // supply comamnds
            list_all_supplies,
            get_supply_by_id,
            register_supply,
            update_supply,
            delete_supply,
            // supplier commands
            list_all_suppliers,
            get_supplier_by_id,
            register_supplier,
            update_supplier,
            search_suppliers,
            delete_supplier,
            // journal commands
            list_all_journals,
            get_journal_by_id,
            get_journal_at,
            record_journal,
            update_journal,
            search_journals,
            delete_journal,
            // stocktaking commands
            list_all_stocktakings,
            get_stocktaking_by_id,
            get_stocktaking_at,
            record_stocktaking,
            update_stocktaking,
            search_stocktakings,
            delete_stocktaking,
            download_stocktaking_csv
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
