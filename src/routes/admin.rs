use std::sync::Arc;

use askama::Template;
use askama_web::WebTemplate;
use axum::{
    Router,
    extract::{Query, State},
    response::IntoResponse,
    routing::get,
};
use serde::Deserialize;

use crate::{
    AppState,
    domain::user::AuditUser,
    extract::current_user::CurrentUser,
    routes::SharedContext,
    util::pagination::{PaginatedResponse, Pagination},
};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/admin/users", get(users))
}

#[derive(Deserialize)]
struct UserSearch {
    page_size: Option<i64>,
    page: Option<i64>,
    q: Option<String>,
}

#[derive(Template, WebTemplate)]
#[template(path = "admin/users.html")]
struct AdminUsersTemplate {
    shared: SharedContext,
    users: PaginatedResponse<AuditUser>,
}

async fn users(
    State(state): State<Arc<AppState>>,
    CurrentUser(user): CurrentUser,
    Query(params): Query<UserSearch>,
) -> impl IntoResponse {
    let pagination = Pagination {
        page: params.page,
        page_size: params.page_size,
    };
    AdminUsersTemplate {
        shared: SharedContext::new(&state.app_info, Some(*user)),
        users: state
            .user_service
            .search(&pagination, &params.q.unwrap_or_default())
            .await,
    }
}
