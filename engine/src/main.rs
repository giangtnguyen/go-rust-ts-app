use axum::{
    Router,
    extract::Json,
    routing::{get, post},
};
use std::net::SocketAddr;

mod domain;

use domain::packing::{PackingRequest, calculate};

async fn health() -> &'static str {
    "ok"
}

async fn packing_handler(
    Json(request): Json<PackingRequest>,
) -> Json<domain::packing::PackingResult> {
    let result = calculate(&request);

    Json(result)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health))
        .route("/packing", post(packing_handler));

    let address = SocketAddr::from(([0, 0, 0, 0], 9000));

    println!("Rust Packing Engine listening on {}", address);

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("failed to bind port 9000");

    axum::serve(listener, app).await.expect("server error");
}
