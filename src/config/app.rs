use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfiguration {
    pub host: String,
    pub port: u16
}
