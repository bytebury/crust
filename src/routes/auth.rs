use axum::{
    Router,
    extract::State,
    response::{IntoResponse, Redirect},
    routing::{delete, get},
};
use axum_extra::extract::{
    CookieJar, Query,
    cookie::{self, Cookie},
};
use reqwest::StatusCode;
use serde::Deserialize;
use std::{net::IpAddr, sync::Arc};

use crate::{
    AppState,
    extract::real_ip::RealIp,
    infrastructure::{
        audit,
        auth::{OAuthProvider, google::GoogleOAuth},
        jwt::{JwtService, user_claims::UserClaims},
    },
    util::htmx::HTMX,
};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/auth/google", get(signin_with_google))
        .route("/auth/google/callback", get(google_callback))
        .route("/auth/signout", delete(signout))
}

#[derive(Debug, Deserialize)]
struct AuthRequest {
    code: String,
}

async fn signin_with_google() -> impl IntoResponse {
    Redirect::to(GoogleOAuth::default().get_auth_url().as_str())
}

async fn google_callback(
    State(state): State<Arc<AppState>>,
    Query(params): Query<AuthRequest>,
    RealIp(ip): RealIp,
    cookies: CookieJar,
) -> Result<impl IntoResponse, StatusCode> {
    let mut user = GoogleOAuth::default()
        .exchange_code_for_user(&params.code)
        .await?;

    let ip: IpAddr = ip.parse().unwrap();
    let country_details = audit::geolocation::get_country_details(ip).unwrap_or_default();

    if let Ok(country) = state.country_service.create(&country_details).await {
        user.country_id = Some(country.id);
        user.locked = country.locked;
    }

    user.region = country_details.region;

    let user = match state.user_service.find_by_email(&user.email).await {
        Ok(Some(user)) => user,
        Ok(None) => state
            .user_service
            .create(&user)
            .await
            .inspect_err(|e| {
                eprintln!(
                    "Something happened while creating user ({}): {e}",
                    user.email
                )
            })
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let token = JwtService::generate(&UserClaims::from(user))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let auth_cookie = Cookie::build(("auth_token", token))
        .path("/")
        .http_only(true)
        .same_site(cookie::SameSite::None)
        .secure(true);

    let cookies = cookies.add(auth_cookie);

    Ok((cookies, Redirect::to("/")))
}

async fn signout(State(_state): State<Arc<AppState>>, cookies: CookieJar) -> impl IntoResponse {
    let cookies = cookies.remove(
        Cookie::build(("auth_token", ""))
            .path("/")
            .http_only(true)
            .same_site(cookie::SameSite::Strict),
    );
    (cookies, HTMX::redirect("/")).into_response()
}
