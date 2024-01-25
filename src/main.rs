use std::net::TcpListener;

use zero2prod::{configurations::{self, get_configuration}, startup::run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let configurations = get_configuration().expect("Failed to read configuration");

    let address = format!("127.0.0.1:{}",configurations.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");

    run(listener)?.await
}
