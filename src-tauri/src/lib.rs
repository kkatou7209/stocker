mod command;
mod core;
mod persistence;

use std::{env, fs};

use tauri::Manager;

use crate::command::*;
use crate::core::stocker::{Ports, Stocker};
use crate::persistence::sqlite::*;

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

            let stocker = Stocker::plug(Ports {
                for_supply_persistence: SqliteSupplyRepository::new(db_path.to_string_lossy()),
                for_supplier_persistence: SqliteSupplierRepository::new(db_path.to_string_lossy()),
                for_journal_persistence: SqliteJournalRepository::new(db_path.to_string_lossy()),
                for_stocktaking_persistence: SqliteStocktakingRepository::new(
                    db_path.to_string_lossy(),
                ),
            });

            app.manage(stocker);

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            list_all_supplies,
            get_supply_by_id,
            register_supply,
            update_supply,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
