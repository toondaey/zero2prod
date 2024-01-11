use std::net::TcpListener;

use secrecy::ExposeSecret;
use sqlx::PgPool;
use zero2prod::{config::Configuration, get_subscriber, init_logger};

#[tokio::main]
async fn main() -> ::std::io::Result<()> {
    let config = Configuration::new().expect("Unable to load configuration");
    let listener = TcpListener::bind(format!(
        "{}:{}",
        config.app.host, config.app.port
    ))
    .expect("Port is already assigned");

    let connection_pool = PgPool::connect(config.database.connection_string().expose_secret())
        .await
        .expect("Could not connect to the database");

    let subscriber = get_subscriber(config.logger.name, config.logger.level, std::io::stdout);
    init_logger(subscriber);

    tracing::info!("Started application at - {}:{}", config.app.host, config.app.port);

    zero2prod::startup::run(listener, connection_pool)?.await
}
