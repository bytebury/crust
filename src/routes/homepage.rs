use std::sync::Arc;

use askama::Template;
use askama_web::WebTemplate;
use axum::{Router, extract::State, response::IntoResponse, routing::get};

use crate::{AppState, routes::SharedContext};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/", get(homepage))
}

#[derive(Template, WebTemplate)]
#[template(path = "homepage.html")]
struct HomepageTemplate {
    shared: SharedContext,
}

async fn homepage(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    HomepageTemplate {
        shared: SharedContext::new(&state.app_info),
    }
    .into_response()
}
