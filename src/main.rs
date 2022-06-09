use rust_api_tutorial_3::run;
use std::net::TcpListener;

// #[actix_web::main] // or #[tokio::main]
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to port 8000");
    run(listener)?.await
}
