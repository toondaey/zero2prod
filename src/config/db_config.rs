use serde::Deserialize;
use secrecy::{Secret, ExposeSecret};

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub name: String,
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgresql://{}:{}@{}/{}",
            self.username, self.password.expose_secret(), self.host, self.name
        ))
    }

    pub fn connection_string_without_db_name(&self) -> Secret<String> {
        Secret::new(format!(
            "postgresql://{}:{}@{}",
            self.username, self.password.expose_secret(), self.host
        ))
    }
}
