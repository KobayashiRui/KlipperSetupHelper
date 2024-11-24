use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod hosting;
mod api;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    tokio::join!(
        serve(hosting::hosting_frontend_dir_app(), 9000),
        serve(api::api_app(), 9001),
    );
    Ok(())
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}