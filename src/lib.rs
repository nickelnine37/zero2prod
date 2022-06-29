use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;
use std::collections::HashMap;
use std::net::TcpListener;

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

pub struct LaunchResult {
    pub server: Server,
    pub port: u16,
}

pub fn run() -> Result<LaunchResult, std::io::Error> {

    // binding to port 0 automatically finds an available port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to port");

    // we need to know what that port is so we can send the appropriate request in our tests
    let port = listener.local_addr().unwrap().port();

    // Server implements Future
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/json", web::get().to(json_check))
            .route("/{name}", web::get().to(greet_check))
    })
        .listen(listener)?
        .run();

    // return OK(LaunchResult)
    Ok(LaunchResult{server, port})
}