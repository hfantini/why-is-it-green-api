#[tokio::main]
async fn main() {
    let app = why_is_it_green_api::app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Cannot create HTTP listener...");

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .expect("Cannot start HTTP server.");
}
