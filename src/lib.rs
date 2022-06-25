use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use std::collections::HashMap;

async fn greet_check(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}", &name)
}

async fn json_check(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json(HashMap::from([
        ("Mercury", 0.4),
        ("Venus", 0.7),
        ("Earth", 1.0),
        ("Mars", 1.5),
    ]))
}

async fn health_check() -> impl Responder {
    // basic check - we should recieve 200 response code with no body
    HttpResponse::Ok()
}

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/json", web::get().to(json_check))
            .route("/{name}", web::get().to(greet_check))
    }).bind("127.0.0.1:8000")?
        .run().await
}