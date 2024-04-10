//! src/main.rs
use std::net::TcpListener;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration file.");
    //We have removed the hardcoded `8000` -its now coming from our settings! Woohoo!
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    println!(
        "The application is running on http://{}:{}",
        listener.local_addr().unwrap().ip(),
        listener.local_addr().unwrap().port()
    );
    run(listener)?.await
}
