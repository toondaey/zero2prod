use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};

pub async fn run() -> ::std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/health_check", web::get().to(health_check))
            .route("/{name}", web::get().to(hello))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
async fn hello(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {name}!")
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

