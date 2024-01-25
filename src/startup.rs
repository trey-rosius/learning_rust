use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};

use crate::routes::{health_check, subscribe};

use sqlx::PgConnection;


pub fn run(listener: TcpListener,
connection:PgConnection) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move|| {
App::new()
    .service(health_check)
    .service(subscribe)
    .app_data(connection.clone())
    })
        
        .listen(listener)?
        .run();
    Ok(server)
}
