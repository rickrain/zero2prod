use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("Failed to read configuration.");

    let address = format!("127.0.0.1:{}", config.application_port);
    //let port = listener.local_addr().unwrap().port();
    let listener = TcpListener::bind(address)?;
    println!("Server listening at http://{}", listener.local_addr()?);
    run(listener)?.await
}
