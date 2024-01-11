pub mod app;
pub mod db;
pub mod logger;

use config::{Config, ConfigError, Environment, File, FileFormat, FileSourceFile};
use serde::Deserialize;

use app::AppConfiguration;
use db::DatabaseConfiguration;
use logger::LoggerConfiguration;

#[derive(Deserialize, Debug)]
pub struct Configuration {
    pub environment: String,
    pub database: DatabaseConfiguration,
    pub app: AppConfiguration,
    pub logger: LoggerConfiguration,
}

impl Configuration {
    pub fn new() -> Result<Self, ConfigError> {
        let app_env: AppEnvironment = std::env::var("APP__ENVIRONMENT")
            .unwrap_or("local".to_string())
            .try_into()?;

        let builder = Config::builder()
            .add_source(File::with_name("config/global"))
            .add_source(Environment::with_prefix("app").separator("__"));

        let builder = builder
            .add_source(app_env.get_config())
            .add_source(Environment::with_prefix("app").separator("__"));

        builder.build()?.try_deserialize()
    }
}

#[derive(Debug)]
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

impl AppEnvironment {
    fn get_config(&self) -> File<FileSourceFile, FileFormat> {
        match self {
            AppEnvironment::PRODUCTION => File::with_name("config/production").required(true),
            AppEnvironment::LOCAL => File::with_name("config/local").required(true),
        }
    }
}
