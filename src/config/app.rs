use serde::Deserialize;
use serde_aux::prelude::deserialize_number_from_string;

#[derive(Deserialize, Debug)]
pub struct AppConfiguration {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}
