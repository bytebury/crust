use crate::application::rbac::Role;
use crate::{SharedState, extract::maybe_current_user::MaybeCurrentUser};
use askama::Template;
use askama_web::WebTemplate;
use axum::{Router, extract::State, response::IntoResponse, routing::get};

use crate::routes::SharedContext;

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(homepage))
        .route("/terms", get(terms))
        .route("/privacy", get(privacy))
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
}

#[derive(Template, WebTemplate)]
#[template(path = "privacy.html")]
struct PrivacyTemplate {
    shared: SharedContext,
}

#[derive(Template, WebTemplate)]
#[template(path = "terms.html")]
struct TermsTemplate {
    shared: SharedContext,
}

async fn homepage(
    State(state): State<SharedState>,
    MaybeCurrentUser(user): MaybeCurrentUser,
) -> impl IntoResponse {
    match user {
        Some(user) => DashboardTemplate {
            shared: SharedContext::new(&state.app_info, Some(user)),
        }
        .into_response(),
        None => HomepageTemplate {
            shared: SharedContext::new(&state.app_info, None),
        }
        .into_response(),
    }
}

async fn terms(
    State(state): State<SharedState>,
    MaybeCurrentUser(user): MaybeCurrentUser,
) -> TermsTemplate {
    TermsTemplate {
        shared: SharedContext::new(&state.app_info, user),
    }
}

async fn privacy(
    State(state): State<SharedState>,
    MaybeCurrentUser(user): MaybeCurrentUser,
) -> PrivacyTemplate {
    PrivacyTemplate {
        shared: SharedContext::new(&state.app_info, user),
    }
}
