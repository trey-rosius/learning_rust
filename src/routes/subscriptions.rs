use actix_web::{post, web, HttpResponse, Responder};
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

#[post("/subscriptions")]
async fn subscribe(
    web::Form(form): web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    // We are using the same interpolation syntax of `println`/`print` here! log::info!(

    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    );

    let _request_span_guard = request_span.enter();
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
        Ok(_) => {
            tracing::info!(
                "request_id: {} New subscriber details have been saved",
                request_id
            );

            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "request_id: {} - Failed to execute query: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
