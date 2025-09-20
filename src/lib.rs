use axum::Router;
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;

pub async fn start() {
    let app = initialize().await;
    let port = env::var("APP_PORT").unwrap_or_else(|_| "8080".to_string());

    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn initialize() -> Router {
    Router::new()
}
