use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{
    fmt::MakeWriter, prelude::__tracing_subscriber_SubscriberExt, EnvFilter, Registry,
};

pub mod config;
pub mod database;
mod domain;
pub mod dtos;
pub mod routes;
pub mod startup;

pub fn get_subscriber<TSink>(
    name: String,
    level: String,
    sink: TSink,
) -> impl Subscriber + Sync + Send
where
    TSink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&level));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_logger(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to setup logger");
    set_global_default(subscriber).unwrap();
}
