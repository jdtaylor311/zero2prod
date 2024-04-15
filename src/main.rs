//! src/main.rs
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::telemetry::*;
use zero2prod::startup::run;

/// It should only be called once!

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration file.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    //We have removed the hardcoded `8000` -its now coming from our settings! Woohoo!
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    println!(
        "The application is running on http://{}:{}",
        listener.local_addr().unwrap().ip(),
        listener.local_addr().unwrap().port()
    );
    run(listener, connection_pool)?.await
}
