use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};
use std::{io::Result, net::TcpListener};

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .listen(listener)?
        .run();

    Ok(server)
}
