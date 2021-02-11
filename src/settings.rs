use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct AppOptions {
    pub listen_address: String,
    pub forward_address: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AllowList {
    pub header_names: Vec<String>,
    pub cookie_names: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub app: AppOptions,
    pub allowlist: AllowList,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut settings = Config::new();
        let env = env::var("ENVIRONMENT").unwrap_or_else(|_| "stage".into());

        let config_path = format!("config/{}/config.toml", env);

        settings.merge(File::with_name(&config_path).required(true))?;
        // freezes settings
        settings.try_into()
    }
}
