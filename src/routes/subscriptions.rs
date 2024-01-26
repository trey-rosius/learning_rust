use actix_web::{post, web, HttpResponse, Responder};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
#[derive(Deserialize)]
struct FormData {
    email: String,
    name: String,
}

#[post("/subscriptions")]
async fn subscribe(
    web::Form(form): web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    // We are using the same interpolation syntax of `println`/`print` here! log::info!(

        let request_id = Uuid::new_v4();
    tracing::info!(
        "request_id {} - Adding '{}' '{}' as a new subscriber.",
        request_id,
        form.email,
        form.name,
    );
    tracing::info!("request_id: {} - Saving new subscriber details in the database ",request_id);

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id,email, name,subscribed_at) VALUES($1,$2,$3,$4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            tracing::info!("request_id: {} New subscriber details have been saved",request_id);

            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("request_id: {} - Failed to execute query: {:?}", request_id,
            e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
