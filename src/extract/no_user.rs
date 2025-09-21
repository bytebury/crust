use crate::AppState;
use crate::extract::BaseUser;
use axum::http::StatusCode;
use axum::{extract::FromRequestParts, http::request::Parts};
use std::sync::Arc;

pub struct NoUser;

impl FromRequestParts<Arc<AppState>> for NoUser {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let user = BaseUser::from_request_parts(parts, state)
            .await
            .map_err(|_| StatusCode::FORBIDDEN)?;

        match user {
            BaseUser::User(_) => Err(StatusCode::FORBIDDEN),
            _ => Ok(NoUser),
        }
    }
}
