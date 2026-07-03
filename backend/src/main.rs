use std::net::SocketAddr;

use axum::{Router, routing::get};
use tower_http::cors::{AllowOrigin, CorsLayer};

use crate::api::health_check;

mod api;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let router = api::router();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("server runs on: {}", addr.to_string());
    axum::serve(listener, router).await.unwrap();
}

