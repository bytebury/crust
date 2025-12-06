use crate::{
    SharedState,
    domain::User,
    infrastructure::jwt::{JwtService, user_claims::UserClaims},
};
use axum::{
    extract::{FromRef, FromRequestParts},
    http::StatusCode,
    http::request::Parts,
};
use axum_extra::extract::CookieJar;
use std::sync::Arc;

pub mod admin_user;
pub mod current_user;
pub mod maybe_current_user;
pub mod no_user;
pub mod real_ip;

#[derive(Clone)]
pub enum BaseUser {
    User(User),
    None,
}

impl FromRequestParts<SharedState> for BaseUser {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &SharedState,
    ) -> Result<Self, StatusCode> {
        let state = Arc::from_ref(state);
        let jar = CookieJar::from_headers(&parts.headers);

        let token = match jar.get("auth_token") {
            Some(cookie) => cookie.value(),
            None => return Ok(BaseUser::None),
        };

        if let Ok(token_data) = JwtService::verify::<UserClaims>(token) {
            let email = token_data.claims.sub;
            if let Ok(Some(user)) = state.user_service.find_by_email(&email).await {
                return Ok(BaseUser::User(user));
            }
        }

        Ok(BaseUser::None)
    }
}
