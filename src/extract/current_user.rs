use crate::domain::User;
use crate::{AppState, extract::BaseUser};
use axum::http::StatusCode;
use axum::{extract::FromRequestParts, http::request::Parts};
use std::sync::Arc;

pub struct CurrentUser(pub User);

impl FromRequestParts<Arc<AppState>> for CurrentUser {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, StatusCode> {
        let user = BaseUser::from_request_parts(parts, state)
            .await
            .map_err(|_| StatusCode::FORBIDDEN)?;

        match user {
            BaseUser::User(user) => Ok(CurrentUser(user)),
            _ => Err(StatusCode::FORBIDDEN),
        }
    }
}
