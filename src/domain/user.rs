use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;

use crate::{
    application::{Action, Can, Role},
    infrastructure::auth::oauth::GoogleUser,
    util::pagination::Paginatable,
};

#[derive(Serialize, FromRow, Clone)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub verified: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub full_name: String,
    pub image_url: String,
    pub role: Role,
    pub stripe_customer_id: Option<String>,
    pub country_id: Option<i64>,
    pub country_code: Option<String>,
    pub country_name: Option<String>,
    pub country_locked: bool,
    pub locked: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn is_admin(&self) -> bool {
        self.role == Role::Admin
    }
}

impl Paginatable for User {
    fn table_name() -> &'static str {
        "users_view"
    }
}

impl Can<User> for User {
    fn can(&self, _: Action, _: &User) -> bool {
        matches!(self.role, Role::Admin)
    }
}

pub struct NewUser {
    pub id: i64,
    pub email: String,
    pub verified: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub full_name: String,
    pub image_url: String,
    pub country_id: Option<i64>,
    pub locked: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<GoogleUser> for NewUser {
    fn from(google_user: GoogleUser) -> Self {
        Self {
            id: 0,
            email: google_user.email,
            verified: google_user.email_verified,
            first_name: google_user.given_name.unwrap_or(google_user.name.clone()),
            last_name: google_user.family_name,
            full_name: google_user.name,
            image_url: google_user.picture,
            country_id: None,
            locked: false,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

pub struct UpdateUser {
    pub id: i64,
    pub locked: bool,
    pub role: Role,
}

impl From<User> for UpdateUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            locked: user.locked,
            role: user.role,
        }
    }
}
