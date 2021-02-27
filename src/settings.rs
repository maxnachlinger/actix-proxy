use config::{Config, ConfigError, File};
use serde::Deserialize;

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
    pub allow_list: AllowList,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut settings = Config::new();
        let config_path = "config/config.toml";
        settings.merge(File::with_name(&config_path).required(true))?;
        settings.try_into()
    }
}
