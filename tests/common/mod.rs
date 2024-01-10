//! tests/health_check.rs
//!
use std::net::TcpListener;

use once_cell::sync::Lazy;
use secrecy::ExposeSecret;
use sqlx::{Executor, PgPool};
use tokio::task::JoinHandle;
use uuid::Uuid;
use zero2prod::{
    config::{Configuration, db::DatabaseConfiguration},
    get_subscriber, init_logger,
};

pub struct ConfigureTestContext {
    pub port: u16,
    pub connection_pool: PgPool,
    app_config: Configuration,
    handle: JoinHandle<Result<(), std::io::Error>>,
}

// Ensure that the tracing stack is only initialized once using once_cell
static LOGGER: Lazy<()> = Lazy::new(|| {
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber("zero2prod".to_owned(), "info".to_owned(), std::io::stdout);
        init_logger(subscriber);
    } else {
        let subscriber = get_subscriber("zero2prod".to_owned(), "info".to_owned(), std::io::sink);
        init_logger(subscriber);
    };
});

impl ConfigureTestContext {
    pub async fn setup() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
        let port = listener.local_addr().unwrap().port();
        let mut config =Configuration::new().expect("Could not load config");
        let connection_pool = Self::setup_db(&mut config.database).await;
        let server = zero2prod::startup::run(listener, connection_pool.clone())
            .expect("Could not start server");
        Lazy::force(&LOGGER);
        let handle = tokio::spawn(server);
        ConfigureTestContext {
            port,
            connection_pool,
            app_config: config,
            handle,
        }
    }
    pub async fn setup_db(db_config: &mut DatabaseConfiguration) -> PgPool {
        db_config.name = Uuid::new_v4().to_string();
        let connection = PgPool::connect(
            &db_config
                .connection_string_without_db_name()
                .expose_secret(),
        )
        .await
        .expect("Could not connect to database");

        connection
            .execute(sqlx::query(&format!(
                r#"CREATE DATABASE "{}";"#,
                db_config.name.as_str()
            )))
            .await
            .expect("Could not create database");

        let connection_pool = PgPool::connect(&db_config.connection_string().expose_secret())
            .await
            .expect("Could not connect to database");

        sqlx::migrate!("./migrations")
            .run(&connection_pool)
            .await
            .expect("Could not run migrations");
        connection_pool
    }

    pub async fn teardown(&self) {
        self.connection_pool.close().await;
        let _ = &self.handle.abort();
        let connection = PgPool::connect(
            &self
                .app_config
                .database
                .connection_string_without_db_name()
                .expose_secret(),
        )
        .await
        .expect("Could not connect to database");

        connection
            .execute(sqlx::query(&format!(
                r#"DROP DATABASE "{}";"#,
                &self.app_config.database.name
            )))
            .await
            .expect(&format!(
                "Could not drop database: {}",
                &self.app_config.database.name
            ));
    }
}
