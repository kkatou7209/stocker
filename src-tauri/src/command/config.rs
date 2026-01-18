use std::fs;
use std::path::PathBuf;

use tauri::{AppHandle, Manager};

use crate::config::{Config, Theme, CONFIG_FILE_NAME};

/// Initialize configuration file if it does not exist
pub fn init_config(app: &AppHandle) -> Result<PathBuf, String> {
    let path = app
        .path()
        .config_dir()
        .map_err(|e| format!("Failed to get config directory: {}", e))?;

    fs::create_dir_all(&path).map_err(|e| format!("Failed to create config directory: {}", e))?;

    let config_path = path.join(CONFIG_FILE_NAME);

    if !config_path.exists() {
        let default_config = Config::default();

        let config_data =
            serde_json::to_string_pretty(&default_config).map_err(|e| format!("{}", e))?;

        fs::write(&config_path, config_data)
            .map_err(|e| format!("Failed to write default config file: {}", e))?;
    }

    Ok(config_path)
}

/// Command to get the current application theme
#[tauri::command]
pub fn get_current_theme(app: AppHandle) -> Result<Theme, String> {
    let config_path = init_config(&app)?;

    let config_data = fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    let config: crate::config::Config = serde_json::from_str(&config_data)
        .map_err(|e| format!("Failed to parse config file: {}", e))?;

    Ok(config.theme)
}

/// Command to set the application theme
#[tauri::command]
pub fn set_theme(app: AppHandle, theme: Theme) -> Result<(), String> {
    let config_path = init_config(&app)?;

    let mut config_data = fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    let mut config: crate::config::Config = serde_json::from_str(&config_data)
        .map_err(|e| format!("Failed to parse config file: {}", e))?;

    config.theme = theme;

    config_data = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    fs::write(&config_path, config_data)
        .map_err(|e| format!("Failed to write config file: {}", e))?;

    Ok(())
}
