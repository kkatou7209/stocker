use serde::{Deserialize, Serialize};

pub const CONFIG_FILE_NAME: &str = "stocker-config.json";

/// Application theme
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Theme {
    Dark,
    Light,
}

/// Application configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    /// Application theme
    pub theme: Theme,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            theme: Theme::Light,
        }
    }
}
