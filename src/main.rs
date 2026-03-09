mod adapters;

use axum::{Router, routing::get};

use crate::adapters::inbound::http::healthcheck_handler::health;

#[tokio::main]
async fn main() {
    // Define the application routes
    let app = Router::new().route("/healthcheck", get(health));

    // Start the HTTP server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Cannot create HTTP listener...");

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .expect("Cannot start HTTP server.");
}
