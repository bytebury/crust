use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;

use crate::infrastructure::auth::GoogleUser;

#[derive(FromRow, Clone)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub verified: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub full_name: String,
    pub image_url: String,
    pub stripe_customer_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
impl User {
    pub fn full_name(&self) -> String {
        match &self.last_name {
            Some(last_name) => format!("{} {}", self.first_name, last_name),
            None => self.first_name.clone(),
        }
    }
}

impl From<GoogleUser> for User {
    fn from(google_user: GoogleUser) -> Self {
        User {
            id: 0,
            email: google_user.email,
            verified: google_user.email_verified,
            first_name: google_user.given_name.unwrap_or(google_user.name.clone()),
            last_name: google_user.family_name,
            full_name: google_user.name,
            image_url: google_user.picture,
            stripe_customer_id: None,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
