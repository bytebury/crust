use std::sync::Arc;

use askama::Template;
use askama_web::WebTemplate;
use axum::{Router, extract::State, response::IntoResponse, routing::get};

use crate::{
    AppState,
    domain::User,
    extract::{current_user::CurrentUser, no_user::NoUser},
    routes::SharedContext,
};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(homepage))
        .route("/dashboard", get(dashboard))
}

#[derive(Template, WebTemplate)]
#[template(path = "homepage.html")]
struct HomepageTemplate {
    shared: SharedContext,
}

#[derive(Template, WebTemplate)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {
    shared: SharedContext,
    current_user: User,
}

async fn homepage(State(state): State<Arc<AppState>>, NoUser: NoUser) -> impl IntoResponse {
    HomepageTemplate {
        shared: SharedContext::new(&state.app_info),
    }
    .into_response()
}

async fn dashboard(
    State(state): State<Arc<AppState>>,
    CurrentUser(current_user): CurrentUser,
) -> impl IntoResponse {
    DashboardTemplate {
        shared: SharedContext::new(&state.app_info),
        current_user,
    }
    .into_response()
}
