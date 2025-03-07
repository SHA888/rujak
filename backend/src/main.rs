use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Define a simple Axum router
    let app = Router::new()
        .route("/", get(root_handler));

    // Server address
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Rujak backend running at http://{}", addr);

    // ✅ Corrected Axum server setup using `TcpListener`
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Simple handler for "/"
async fn root_handler() -> &'static str {
    "Welcome to Rujak - Rust-based Open Journal Management!"
}
