use rust_api_tutorial_3::startup::run;
use rust_api_tutorial_3::configuration::get_configuration;

use sqlx::{Connection, PgConnection};
use std::net::TcpListener;


// #[actix_web::main] // or #[tokio::main]
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    // We have removed the hard-coded `8000` - it's now coming from our settings!
    let address = format!("127.0.0.1:{}", configuration.application_port);
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let listener = TcpListener::bind(address)?;
    run(listener, connection)?.await
}
