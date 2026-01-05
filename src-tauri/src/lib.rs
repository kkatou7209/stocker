mod command;
mod core;
mod persistence;

use std::{env, fs};

use tauri::Manager;

use crate::command::*;
use crate::core::stocker::{Ports, Stocker};
use crate::persistence::sqlite::*;

const DB_NAME: &str = "stocker.db";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
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
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // supply comamnds
            list_all_supplies,
            get_supply_by_id,
            register_supply,
            update_supply,
            // supplier commands
            list_all_suppliers,
            get_supplier_by_id,
            register_supplier,
            update_supplier,
            search_suppliers,
            // journal commands
            list_all_journals,
            get_journal_by_id,
            get_journal_at,
            record_journal,
            update_journal,
            search_journals,
            // stocktaking commands
            list_all_stocktakings,
            get_stocktaking_by_id,
            get_stocktaking_at,
            record_stocktaking,
            update_stocktaking,
            search_stocktakings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
