use rust_api_tutorial_3::run;

// #[actix_web::main] // or #[tokio::main]
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    run()?.await
}
