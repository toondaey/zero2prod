use std::net::TcpListener;

use secrecy::ExposeSecret;
// use env_logger::Env;
use sqlx::PgPool;
// use tracing::subscriber::set_global_default;
// use tracing_log::LogTracer;
// use tracing::dispatcher::set_global_default;
// use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
// use tracing_log::LogTracer;
// use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, EnvFilter, Registry};
use zero2prod::{config::app_config::AppConfig, init_logger, get_subscriber};

#[tokio::main]
async fn main() -> ::std::io::Result<()> {
    let config = AppConfig::new().expect("Failed to load configuration");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.app_port))
        .expect("Port is already assigned");

    let connection_pool = PgPool::connect(config.database.connection_string().expose_secret())
        .await
        .expect("Could not connect to the database");

    let subscriber = get_subscriber(config.logger.name, config.logger.level, std::io::stdout);
    init_logger(subscriber);
        // let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    // let formatting_layer = BunyanFormattingLayer::new("zero2prod".to_owned(), std::io::stdout);
    // let log_subscriber = Registry::default()
    //     .with(env_filter)
    //     .with(JsonStorageLayer)
    //     .with(formatting_layer);
    // set_global_default(log_subscriber.into()).expect("Failed to set logs subscriber");
    // env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    zero2prod::startup::run(listener, connection_pool)?.await
}
