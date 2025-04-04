use serde::Deserialize;
use std::fs;
use std::path::Path;
use tracing::{error, info};

/// Configuration for mdserve with custom styling and layout options
#[derive(Debug, Deserialize, Default, Clone)]
pub struct Config {
    /// Navigation links to display in the header
    pub navigation: Option<Vec<NavLink>>,
}

/// Navigation link structure
#[derive(Debug, Deserialize, Clone)]
pub struct NavLink {
    pub text: String,
    pub url: String,
}

impl Config {
    /// Loads configuration from a file at the given path
    pub fn from_file(path: &Path) -> Self {
        if !path.exists() {
            info!(
                "Config file not found at {}, using defaults",
                path.display()
            );
            return Config::default();
        }

        match fs::read_to_string(path) {
            Ok(content) => match serde_yaml::from_str(&content) {
                Ok(config) => {
                    info!("Successfully loaded config from {}", path.display());
                    config
                }
                Err(e) => {
                    error!("Failed to parse config file {}: {}", path.display(), e);
                    Config::default()
                }
            },
            Err(e) => {
                error!("Failed to read config file {}: {}", path.display(), e);
                Config::default()
            }
        }
    }
}
