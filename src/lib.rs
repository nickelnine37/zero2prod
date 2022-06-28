use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;
use std::collections::HashMap;

async fn greet_check(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}", &name)
}

async fn json_check() -> impl Responder {
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

pub fn run() -> Result<Server, std::io::Error> {

    // server.bind().run() returns a Server, which is kind of like a future in
    // that we can call .await on it

    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/json", web::get().to(json_check))
            .route("/{name}", web::get().to(greet_check))
    }).bind("127.0.0.1:8000")?
        .run();

    // return OK(server)
    Ok(server)
}