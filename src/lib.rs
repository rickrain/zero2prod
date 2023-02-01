use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};
use std::io::Result;

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run() -> Result<Server> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run();

    Ok(server)
}
