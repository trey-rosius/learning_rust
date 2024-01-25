use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct FormData {
    email: String,
    name: String,
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[post("/subscriptions")]
async fn subscribe(web::Form(_form): web::Form<FormData>) -> impl Responder {

    
    HttpResponse::Ok().finish()
}

pub fn run(listener:TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check)
    .service(subscribe)
)
    
        .listen(listener)?
        .run();
    Ok(server)
}
