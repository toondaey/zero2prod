use std::net::TcpListener;

use actix_web::{dev::Server, HttpServer, App};

use crate::routes::{health_check::health_check, subscriptions::subscriptions};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(health_check)
            .service(subscriptions)
            // .route("/health_check", web::get().to(health_check))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

