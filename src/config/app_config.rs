use config::{Config, Environment, File};
use serde::Deserialize;

use super::{db_config::DatabaseConfig, logger_config::LoggerConfig};

#[derive(Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub app_port: u16,
    pub logger: LoggerConfig,
}

impl AppConfig {
    pub fn new() -> Result<Self, config::ConfigError> {
        let builder = Config::builder()
            .add_source(File::with_name("config"))
            .add_source(Environment::with_prefix("app").separator("_"))
            .build()?;

        builder.try_deserialize()
    }
}
