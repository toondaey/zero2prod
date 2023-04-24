use actix_web::{dev::Server, web, App, HttpResponse, HttpServer };

pub fn run(port: i32) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
    .bind(format!("127.0.0.1:{port}"))?
    .run();
    Ok(server)
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
