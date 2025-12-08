use crate::domain::NewUser;
use axum::http::StatusCode;

pub mod google;
pub use google::*;

pub trait OAuthProvider {
    fn get_auth_url(&self) -> String;
    fn exchange_code_for_user(
        &self,
        code: &str,
    ) -> impl Future<Output = Result<NewUser, StatusCode>>;
}
