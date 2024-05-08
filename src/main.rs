use crate::controller::transform;
use axum::{Router, extract::multipart};

mod controller;
mod generator;
mod zipper;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest("/api", transform());

    let addr = "localhost:8081";

    let listener = tokio::net::TcpListener::bind(addr)
        .await.expect("Failed to bind to address");

    println!("Server is running on {}", addr);

    // Start serving the application
    if let Err(err) = axum::serve(listener, app).await {
        eprintln!("Server error: {}", err);
    }
}
