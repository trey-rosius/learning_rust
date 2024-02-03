use std::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer}; use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use sqlx::PgPool;
use zero2prod::{configurations::get_configuration, startup::run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer =BunyanFormattingLayer::new("zero2prod".into(),
std::io::stdout) ;

   let subscriber = Registry::default()
   .with(env_filter)
   .with(JsonStorageLayer)
   .with(formatting_layer);

   set_global_default(subscriber).expect("Failed to set  subscriber");
    let configurations = get_configuration().expect("Failed to read configuration");

    let connection_pool = PgPool::connect(&configurations.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let address = format!("127.0.0.1:{}", configurations.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");

    run(listener, connection_pool)?.await
}
