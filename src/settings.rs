use std::sync::Arc;

use arc_swap::ArcSwap;
use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppSettings {
    pub database: Database,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub host: String,
    pub port: Option<i32>,
    pub username: String,
    pub password: String,
    pub database: String,
    pub max_connections: Option<i32>,
}

pub fn init_config(location: &str) -> Result<ArcSwap<AppSettings>, ConfigError> {
    let config = Config::builder().add_source(File::with_name(location)).build()?;
    let app_settings: AppSettings = config.try_deserialize()?;

    Ok(ArcSwap::from(Arc::new(app_settings)))
}
