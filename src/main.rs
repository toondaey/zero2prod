use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::{config::Configuration, get_subscriber, init_logger};

#[tokio::main]
async fn main() -> ::std::io::Result<()> {
    let config = Configuration::new().expect("Unable to load configuration");
    let listener = TcpListener::bind(format!("{}:{}", config.app.host, config.app.port))
        .expect("Port is already assigned");


    let connection_pool = PgPool::connect_with(config.database.with_db())
        .await
        .expect("Could not connect to the database");

    let subscriber = get_subscriber(
        config.logger.name.clone(),
        config.logger.level.clone(),
        std::io::stdout,
    );
    init_logger(subscriber);

    tracing::debug!("Configuration properties: {:?}", config);
    tracing::info!(
        "Started application at - {}:{}",
        config.app.host,
        config.app.port
    );

    zero2prod::startup::run(listener, connection_pool)?.await
}
