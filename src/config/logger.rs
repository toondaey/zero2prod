use serde::Deserialize;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, EnvFilter, Registry};

#[derive(Deserialize, Debug)]
pub struct LoggerConfiguration {
    pub name: String,
    pub level: String,
}

impl LoggerConfiguration {
    pub fn new_subscriber(&self) -> impl Subscriber + Send + Sync {
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&self.level));
        let formatting_layer = BunyanFormattingLayer::new(self.name.to_owned(), std::io::stdout);
        Registry::default()
            .with(env_filter)
            .with(JsonStorageLayer)
            .with(formatting_layer)
    }
}
