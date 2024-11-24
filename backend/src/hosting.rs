use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

pub fn hosting_frontend_dir_app() -> Router {
    Router::new().nest_service(
         "/", ServeDir::new("dist")
        .not_found_service(ServeFile::new("dist/index.html")),
    )
}