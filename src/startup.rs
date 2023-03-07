use crate::routes::health_check::health_check;
use crate::routes::subscriptions::subscribe;
use actix_web::{dev::Server, web, App, HttpServer};
use std::{io::Result, net::TcpListener};

pub fn run(listener: TcpListener) -> Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
