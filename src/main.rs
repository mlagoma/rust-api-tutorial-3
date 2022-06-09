use rust_api_tutorial_3::run;

// #[actix_web::main] // or #[tokio::main]
#[tokio::main]
async fn main() -> std::io::Result<()> {
    run().await
}
