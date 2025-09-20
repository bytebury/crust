use axum::{
    Router,
    http::{HeaderValue, header::CACHE_CONTROL},
};
use sqlx::{Pool, Sqlite};
use std::{env, net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tower_http::{
    compression::CompressionLayer, services::ServeDir, set_header::SetResponseHeaderLayer,
};

use crate::{
    application::UserService,
    infrastructure::db::{Database, UserRepository},
};

pub mod application;
pub mod domain;
pub mod extract;
pub mod filter;
pub mod infrastructure;
pub mod routes;

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
    let db = Arc::new(Database::initialize().await);
    let app_info = AppInfo::new();
    let state = Arc::new(AppState::new(&db, app_info));
    let serve_static = Router::new()
        .nest_service("/assets", ServeDir::new("public"))
        .layer(SetResponseHeaderLayer::if_not_present(
            CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=31536000"),
        ));

    Router::new()
        .merge(serve_static)
        .merge(routes::homepage::routes())
        .merge(routes::auth::routes())
        .with_state(state)
        .layer(CompressionLayer::new())
}

#[derive(Clone)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
}
impl AppInfo {
    pub fn new() -> Self {
        Self {
            name: env::var("APP_NAME").unwrap_or("Crust App".to_string()),
            version: env::var("APP_VERSION").unwrap_or("0".to_string()),
        }
    }
}

pub struct AppState {
    pub app_info: AppInfo,
    pub user_service: UserService,
}
impl AppState {
    pub fn new(db: &Arc<Pool<Sqlite>>, app_info: AppInfo) -> Self {
        Self {
            app_info,
            user_service: UserService::new(UserRepository::new(db)),
        }
    }
}
