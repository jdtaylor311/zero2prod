//! src/main.rs
use zero2prod::configuration::get_configuration;
use zero2prod::startup::build;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

/// It should only be called once!

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    //Tracin setup
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration file.");

    let server = build(configuration).await?;
    server.await?;
    Ok(())
}
