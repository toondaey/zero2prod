use std::net::TcpListener;

use sqlx::PgPool;
use env_logger::Env;
use zero2prod::config::app_config::AppConfig;

#[tokio::main]
async fn main() -> ::std::io::Result<()> {
    let config = AppConfig::new().expect("Failed to load configuration");
    let listener = TcpListener::bind(&format!("127.0.0.1:{}", config.app_port))
        .expect("Port is already assigned");
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Could not connect to the database");
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    zero2prod::startup::run(listener, connection_pool)?.await
}
