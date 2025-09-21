use crate::{AppState, domain::User, extract::BaseUser};
use axum::http::StatusCode;
use axum::{extract::FromRequestParts, http::request::Parts};
use std::sync::Arc;

pub struct MaybeCurrentUser(pub Option<User>);

impl FromRequestParts<Arc<AppState>> for MaybeCurrentUser {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, StatusCode> {
        let user = BaseUser::from_request_parts(parts, state)
            .await
            .map_err(|_| StatusCode::FORBIDDEN)?;

        let user = match user {
            BaseUser::User(user) => Some(user),
            _ => None,
        };

        Ok(MaybeCurrentUser(user))
    }
}
