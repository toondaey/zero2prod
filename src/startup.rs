use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer, middleware::Logger};
use sqlx::PgPool;

use crate::routes::{health_check::health_check, subscriptions::subscriptions};

pub fn run(listener: TcpListener, connection_pool: PgPool) -> Result<Server, std::io::Error> {
    let connection_pool = web::Data::new(connection_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(health_check)
            .service(subscriptions)
            .app_data(connection_pool.clone())
        // .route("/health_check", web::get().to(health_check))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
