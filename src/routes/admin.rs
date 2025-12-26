use crate::infrastructure::audit::geolocation::Country;
use crate::prelude::*;
use crate::{
    domain::user::{UpdateUser, User},
    util::htmx::HTMX,
};

use axum::routing::{delete, put};
use axum::{
    Form, Router,
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{get, patch},
};
use reqwest::StatusCode;

use crate::{
    extract::admin_user::AdminUser,
    util::pagination::{PaginatedResponse, Pagination},
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/admin/users", get(users))
        .route("/admin/users/{id}", get(view_user))
        .route("/admin/users/{id}", patch(edit_user))
        .route("/admin/countries", get(countries))
        .route(
            "/admin/countries/{id}/lock-or-unlock",
            patch(lock_or_unlock_country),
        )
        .route("/admin/announcements", get(announcements))
        .route("/admin/announcements", put(new_announcement))
        .route("/admin/announcements/{id}", delete(delete_announcement))
        .route(
            "/admin/announcements/{id}/edit",
            get(edit_announcement_page),
        )
        .route("/admin/announcements/{id}", patch(edit_announcement))
        .route("/admin/announcements/new", get(new_announcement_page))
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
    users: PaginatedResponse<User>,
}

#[derive(Template, WebTemplate)]
#[template(path = "admin/view_user.html")]
struct AdminViewUserTemplate {
    user: User,
}

#[derive(Deserialize)]
struct UpdateUserForm {
    locked: Option<String>,
    role: Role,
}

#[derive(Template, WebTemplate)]
#[template(path = "admin/countries.html")]
struct AdminCountriesTemplate {
    shared: SharedContext,
    countries: Vec<Country>,
}

#[derive(Template, WebTemplate)]
#[template(path = "admin/announcements/announcements.html")]
struct AnnouncementsTemplate {
    shared: SharedContext,
    announcements: Vec<Announcement>,
}

#[derive(Template, WebTemplate)]
#[template(path = "admin/announcements/new_announcement.html")]
struct NewAnnouncementTemplate {
    form: NewAnnouncement,
    error_message: Option<String>,
}

#[derive(Template, WebTemplate)]
#[template(path = "admin/announcements/edit_announcement.html")]
struct EditAnnouncementTemplate {
    form: EditAnnouncement,
    error_message: Option<String>,
}

#[derive(Deserialize)]
struct UpdateCountryForm {
    locked: Option<String>,
}

#[derive(Deserialize)]
struct CountrySearchQuery {
    q: Option<String>,
}

async fn users(
    State(state): State<SharedState>,
    AdminUser(user): AdminUser,
    Query(params): Query<UserSearch>,
) -> impl IntoResponse {
    let pagination = Pagination {
        page: params.page,
        page_size: params.page_size,
    };
    AdminUsersTemplate {
        shared: SharedContext::new(&state.app_info, Some(user)),
        users: state
            .user_service
            .search(&pagination, &params.q.unwrap_or_default())
            .await,
    }
}

async fn view_user(
    State(state): State<SharedState>,
    AdminUser(_): AdminUser,
    Path(user_id): Path<i64>,
) -> impl IntoResponse {
    match state.user_service.find_by_id(user_id).await {
        Ok(user) => AdminViewUserTemplate { user }.into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn edit_user(
    State(state): State<SharedState>,
    AdminUser(_): AdminUser,
    Path(user_id): Path<i64>,
    Form(form): Form<UpdateUserForm>,
) -> impl IntoResponse {
    let user = match state.user_service.find_by_id(user_id).await {
        Ok(user) => user,
        Err(_) => return StatusCode::NOT_FOUND.into_response(),
    };

    let mut user = UpdateUser::from(user);
    user.locked = form.locked.is_some();
    user.role = form.role;

    match state.user_service.update(&user).await {
        Ok(_) => HTMX::refresh().into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn countries(
    State(state): State<SharedState>,
    AdminUser(user): AdminUser,
    Query(params): Query<CountrySearchQuery>,
) -> impl IntoResponse {
    AdminCountriesTemplate {
        countries: state
            .country_service
            .search(&params.q.unwrap_or_default())
            .await,
        shared: SharedContext::new(&state.app_info, Some(user)),
    }
}

async fn lock_or_unlock_country(
    State(state): State<SharedState>,
    AdminUser(_): AdminUser,
    Path(id): Path<i64>,
    Form(form): Form<UpdateCountryForm>,
) -> impl IntoResponse {
    let _ = match form.locked {
        Some(_) => state.country_service.lock(id).await,
        None => state.country_service.unlock(id).await,
    };

    StatusCode::ACCEPTED
}

async fn announcements(
    State(state): State<SharedState>,
    AdminUser(user): AdminUser,
) -> impl IntoResponse {
    AnnouncementsTemplate {
        shared: SharedContext::new(&state.app_info, Some(user)),
        announcements: state
            .announcement_service
            .find_all()
            .await
            .unwrap_or_default(),
    }
}

async fn new_announcement_page(AdminUser(_): AdminUser) -> impl IntoResponse {
    NewAnnouncementTemplate {
        form: NewAnnouncement::default(),
        error_message: None,
    }
}

async fn new_announcement(
    State(state): State<SharedState>,
    AdminUser(_): AdminUser,
    Form(form): Form<NewAnnouncement>,
) -> impl IntoResponse {
    match state.announcement_service.create(&form).await {
        Ok(_) => HTMX::refresh().into_response(),
        Err(err) => NewAnnouncementTemplate {
            form,
            error_message: Some(err.to_string()),
        }
        .into_response(),
    }
}

async fn edit_announcement_page(
    State(state): State<SharedState>,
    AdminUser(_): AdminUser,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match state.announcement_service.find_by_id(id).await {
        Ok(announcement) => EditAnnouncementTemplate {
            form: announcement.into(),
            error_message: None,
        }
        .into_response(),
        Err(err) => EditAnnouncementTemplate {
            form: EditAnnouncement::default(),
            error_message: Some(err.to_string()),
        }
        .into_response(),
    }
}

async fn edit_announcement(
    State(state): State<SharedState>,
    AdminUser(_): AdminUser,
    Form(form): Form<EditAnnouncement>,
) -> impl IntoResponse {
    match state.announcement_service.update(&form).await {
        Ok(_) => HTMX::refresh().into_response(),
        Err(err) => EditAnnouncementTemplate {
            form,
            error_message: Some(err.to_string()),
        }
        .into_response(),
    }
}

async fn delete_announcement(
    State(state): State<SharedState>,
    AdminUser(_): AdminUser,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let _ = state.announcement_service.delete(id).await;
    HTMX::refresh().into_response()
}
