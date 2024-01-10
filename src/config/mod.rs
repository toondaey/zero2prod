pub mod app;
pub mod db;
pub mod logger;


use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

use app::AppConfiguration;
use db::DatabaseConfiguration;
use logger::LoggerConfiguration;

#[derive(Deserialize)]
pub struct Configuration {
    pub database: DatabaseConfiguration,
    #[serde(rename = "app-config")]
    pub app: AppConfiguration,
    pub logger: LoggerConfiguration,
}

impl Configuration {
    pub fn new() -> Result<Self, ConfigError> {
        let builder = Config::builder()
            .add_source(File::with_name("config/global"))
            .add_source(Environment::with_prefix("app").separator("_"));

        let app_env: AppEnvironment = std::env::var("APP_ENVIRONMENT")
            .unwrap_or("local".to_string())
            .try_into()?;

        let builder = builder.add_source(
            match app_env {
                AppEnvironment::PRODUCTION => File::with_name("config/local").required(true),
                AppEnvironment::LOCAL => File::with_name("config/production").required(true),
            }
        );

        builder.build()?.try_deserialize()
    }
}

enum AppEnvironment {
    PRODUCTION,
    LOCAL,
}

impl TryFrom<String> for AppEnvironment {
    type Error = ConfigError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(AppEnvironment::LOCAL),
            "production" => Ok(AppEnvironment::PRODUCTION),
            x => Err(ConfigError::Message(format!(
                "Unable to process environment specified: {x}"
            ))),
        }
    }
}
