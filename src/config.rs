use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::PathBuf;
use tracing::debug;

#[derive(Debug, Deserialize)]
pub struct Config {
    /// Override default shell detection
    pub shell: Option<String>,

    /// Custom path for the aliases JSON file
    pub aliases_path: Option<PathBuf>,

    /// Log level: error, warn, info, debug, trace
    #[serde(default = "default_log_level")]
    pub log_level: String,
}

fn default_log_level() -> String {
    "error".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            shell: None,
            aliases_path: None,
            log_level: default_log_level(),
        }
    }
}

impl Config {
    pub fn path() -> Result<PathBuf> {
        let home =
            dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Cannot determine home directory"))?;
        Ok(home.join(".akash").join("config.toml"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::path()?;

        if !path.exists() {
            debug!("No config file found at {}, using defaults", path.display());
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read {}", path.display()))?;

        let config: Self = toml::from_str(&content)
            .with_context(|| format!("Failed to parse {}", path.display()))?;

        debug!("Loaded config from {}", path.display());
        Ok(config)
    }

    /// Parse log_level string into tracing::Level
    pub fn tracing_level(&self) -> tracing::Level {
        match self.log_level.to_lowercase().as_str() {
            "trace" => tracing::Level::TRACE,
            "debug" => tracing::Level::DEBUG,
            "info" => tracing::Level::INFO,
            "warn" => tracing::Level::WARN,
            _ => tracing::Level::ERROR,
        }
    }
}
