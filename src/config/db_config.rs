use serde::Deserialize;

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub username: String,
    pub password: String,
    pub port: u16,
    pub name: String,
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgresql://{}:{}@{}/{}",
            self.username, self.password, self.host, self.name
        )
    }

    pub fn connection_string_without_db_name(&self) -> String {
        format!(
            "postgresql://{}:{}@{}",
            self.username, self.password, self.host
        )
    }
}
