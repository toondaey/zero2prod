use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use serde_aux::prelude::deserialize_number_from_string;
use sqlx::{
    postgres::{PgConnectOptions, PgSslMode},
    ConnectOptions,
};

#[derive(Deserialize, Debug)]
pub struct DatabaseConfiguration {
    pub host: Secret<String>,
    pub username: Secret<String>,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub name: Secret<String>,
    pub require_ssl: bool,
}

impl DatabaseConfiguration {
    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db()
            .database(self.name.expose_secret())
            .application_name("zero2prod")
            .log_statements(tracing_log::log::LevelFilter::Trace)
    }

    pub fn without_db(&self) -> PgConnectOptions {
        let require_ssl = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(self.host.expose_secret())
            .username(self.username.expose_secret())
            .password(self.password.expose_secret())
            .ssl_mode(require_ssl)
    }
}
