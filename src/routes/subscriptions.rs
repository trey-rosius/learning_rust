use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;
#[derive(Deserialize)]
struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name="Adding a new subscriber",
    skip(form,pool),
    fields(
        request_id = %Uuid::new_v4(),
        subscriber_email=%form.email,
        subscriber_name=%form.name
    )
)]
#[post("/subscriptions")]
async fn subscribe(web::Form(form): web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let query_span = tracing::info_span!("Saving new subscriber details in the database ");

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id,email, name,subscribed_at) VALUES($1,$2,$3,$4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.as_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
