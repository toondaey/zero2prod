use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/health_check", web::get().to(health_check))
            .route("/{name}", web::get().to(hello))
    })
    .bind("127.0.0.1:8000")?
    .run();
    Ok(server)
}

async fn hello(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {name}!")
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
