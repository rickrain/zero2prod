use crate::routes::health_check::health_check;
use crate::routes::subscriptions::subscribe;
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::{io::Result, net::TcpListener};

pub fn run(listener: TcpListener, connection_pool: PgPool) -> Result<Server> {
    let connection_pool = web::Data::new(connection_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
