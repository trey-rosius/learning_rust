use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct FormData {
    email: String,
    name: String,
}

#[post("/subscriptions")]
async fn subscribe(web::Form(_form): web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}
