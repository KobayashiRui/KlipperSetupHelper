use axum::{routing::get, Router};
use std::env;
use tower_http::cors::{Any, CorsLayer};
use http::HeaderValue;
use std::process::Command;

mod ksh;

pub fn api_app() -> Router {
    Router::new().nest("/api", api_router())
}

pub fn api_router() -> Router {
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
        .nest("/ksh", ksh::ksh_router())
        .route("/check", get(check))
        .route("/ls", get(ls_com))
        .layer(cors)
}

async fn ls_com() -> String{
  let output = Command::new("ls")
      // プロセスを実行するディレクトリを指定する
      .current_dir("/")
      .output()
      .expect("failed to execute process");
  let hello = output.stdout;
  let ls_result_data =  std::str::from_utf8(&hello).unwrap();
  return ls_result_data.to_string();
}


async fn check() -> &'static str {
    "Hello, world!"
}