use axum::{routing::get, Router};
use std::env;
use tower_http::cors::{Any, CorsLayer};
use http::HeaderValue;

pub fn api_app() -> Router {
    let env = env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string());

    let cors = if env == "development" {
        println!("[develop mode] CORS Allowing all origins");
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
    } else {
        CorsLayer::new()
            .allow_origin("http://0.0.0.0:9000".parse::<HeaderValue>().unwrap())
            .allow_methods(Any)
            .allow_headers(Any)
    };

    Router::new()
        .route("/api/check", get(check))
        .layer(cors)
}


async fn check() -> &'static str {
    "Hello, world!"
}