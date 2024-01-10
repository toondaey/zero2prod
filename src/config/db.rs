use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DatabaseConfiguration {
    pub host: String,
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub name: String,
}

impl DatabaseConfiguration {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgresql://{}:{}@{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.name
        ))
    }

    pub fn connection_string_without_db_name(&self) -> Secret<String> {
        Secret::new(format!(
            "postgresql://{}:{}@{}",
            self.username,
            self.password.expose_secret(),
            self.host
        ))
    }
}
