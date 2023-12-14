use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub port: usize,
    pub start_at_login: bool,
    pub start_minimised: bool,
    pub theme_path: Option<String>,
    pub sensor_poll_rate_ms: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            port: 2137,
            start_at_login: false,
            start_minimised: false,
            theme_path: None,
            sensor_poll_rate_ms: 1000,
        }
    }
}

impl AppConfig {
    pub fn load() -> Self {
        confy::load("zefirs-flashy-cooler", Some("AppConfig")).unwrap_or_default()
    }

    pub fn save(&self) -> anyhow::Result<()> {
        confy::store("zefirs-flashy-cooler", Some("AppConfig"), self.clone())
            .context("Failed to store config!")
    }
}
