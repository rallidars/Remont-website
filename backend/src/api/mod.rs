use axum::{ Router, response::IntoResponse, routing::get };
use tower_http::cors::{AllowOrigin, CorsLayer};

mod services;
mod repositories;
mod models;
mod contollers;

pub async fn health_check() -> impl IntoResponse {
    "I am alive".to_string()
}

pub fn router() -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/gallery", get(contollers::gallery))
        .route("/cta", get(contollers::cta))
        .route("/price_catalog", get(contollers::price_catalog))
        .layer(CorsLayer::new().allow_origin(AllowOrigin::any()))
}
