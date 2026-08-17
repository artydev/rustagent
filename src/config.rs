//! API key configuration.
//!
//! The Albert API key can be provided through two channels, in order of
//! precedence:
//!
//! 1. The `ALBERT_API_KEY` environment variable (highest priority).
//! 2. A config file at the platform config directory
//!    (`~/.config/rustagent/config.toml` on Linux, `%APPDATA%\rustagent\config.toml`
//!    on Windows, `~/Library/Application Support/rustagent/config.toml` on macOS).
//!
//! This module also validates the key and can persist it to the config file so
//! that a first-run setup can write it once and have it remembered.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// The environment variable that holds the API key (highest priority).
pub const API_KEY_ENV: &str = "ALBERT_API_KEY";

/// The name of the config file inside the platform config directory.
pub const CONFIG_FILE_NAME: &str = "config.toml";

/// Where the API key came from, used for user-facing messages.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeySource {
    /// Read from the `ALBERT_API_KEY` environment variable.
    Environment,
    /// Read from the config file.
    ConfigFile,
    /// No key was found anywhere.
    Missing,
}

/// The resolved API key configuration.
#[derive(Clone, Debug)]
pub struct ApiKeyConfig {
    /// The API key, or an empty string if none was found.
    pub key: String,
    /// Where the key came from.
    #[allow(dead_code)] // used for future user-facing source reporting
    pub source: KeySource,
}

/// The on-disk representation of the config file.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FileConfig {
    /// The Albert API key.
    pub api_key: Option<String>,
}

impl ApiKeyConfig {
    /// Load the API key from the environment variable first, then the config
    /// file. Returns a config with an empty key and `KeySource::Missing` if
    /// neither source provides one.
    pub fn load() -> ApiKeyConfig {
        // 1. Environment variable (highest priority).
        if let Ok(key) = std::env::var(API_KEY_ENV) {
            let key = key.trim().to_string();
            if !key.is_empty() {
                return ApiKeyConfig {
                    key,
                    source: KeySource::Environment,
                };
            }
        }

        // 2. Config file.
        if let Some(key) = Self::load_from_file() {
            let key = key.trim().to_string();
            if !key.is_empty() {
                return ApiKeyConfig {
                    key,
                    source: KeySource::ConfigFile,
                };
            }
        }

        // 3. Nothing found.
        ApiKeyConfig {
            key: String::new(),
            source: KeySource::Missing,
        }
    }

    /// Read the API key from the config file, if present.
    fn load_from_file() -> Option<String> {
        let path = Self::config_file_path();
        let content = std::fs::read_to_string(&path).ok()?;
        let config: FileConfig = toml::from_str(&content).ok()?;
        config.api_key
    }

    /// The absolute path to the config file.
    pub fn config_file_path() -> PathBuf {
        crate::platform::config_dir().join(CONFIG_FILE_NAME)
    }

    /// Persist the API key to the config file, creating the directory if
    /// needed. Returns an error message on failure.
    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_file_path();
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }
        let config = FileConfig {
            api_key: Some(self.key.clone()),
        };
        let content = toml::to_string_pretty(&config)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        std::fs::write(&path, content).map_err(|e| format!("Failed to write config file: {}", e))
    }

    /// Validate the API key. Returns `Ok(())` if the key looks usable, or an
    /// `Err` with a human-readable explanation otherwise.
    pub fn validate(&self) -> Result<(), String> {
        if self.key.trim().is_empty() {
            return Err(format!(
                "No Albert API key found. Set the {} environment variable or \
                 create a config file at {}.",
                API_KEY_ENV,
                Self::config_file_path().display()
            ));
        }
        // A minimal sanity check: keys are typically non-whitespace tokens.
        if self.key.chars().any(char::is_whitespace) {
            return Err("The Albert API key contains whitespace and looks invalid.".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_rejects_empty() {
        let cfg = ApiKeyConfig {
            key: String::new(),
            source: KeySource::Missing,
        };
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn validate_rejects_whitespace() {
        let cfg = ApiKeyConfig {
            key: "abc def".to_string(),
            source: KeySource::Environment,
        };
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn validate_accepts_valid_key() {
        let cfg = ApiKeyConfig {
            key: "sk-1234567890abcdef".to_string(),
            source: KeySource::Environment,
        };
        assert!(cfg.validate().is_ok());
    }
}
