use std::sync::Arc;

use sqlx::{SqlitePool, query_as, query_scalar};

use crate::domain::{User, user::NewUser};

pub struct UserRepository {
    db: Arc<SqlitePool>,
}
impl UserRepository {
    pub fn new(db: &Arc<SqlitePool>) -> Self {
        Self { db: db.clone() }
    }

    pub async fn find_by_id(&self, id: i64) -> Result<User, sqlx::Error> {
        query_as(r#"SELECT * FROM v_users WHERE id = ?"#)
            .bind(id)
            .fetch_one(self.db.as_ref())
            .await
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        query_as(r#"SELECT * FROM v_users WHERE email = ?"#)
            .bind(email)
            .fetch_optional(self.db.as_ref())
            .await
    }

    pub async fn create(&self, user: &NewUser) -> Result<User, sqlx::Error> {
        let user_id: i64 = query_scalar(
            r#"
        INSERT INTO users (
            email, full_name, first_name, last_name, image_url, country_id, region, verified, locked
        )
        VALUES (LOWER(?), LOWER(?), LOWER(?), LOWER(?), ?, ?, ?, ?, ?)
        RETURNING id
        "#,
        )
        .bind(&user.email)
        .bind(&user.full_name)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.image_url)
        .bind(user.country_id)
        .bind(&user.region)
        .bind(user.verified)
        .bind(user.locked)
        .fetch_one(self.db.as_ref())
        .await?;

        query_as(r#"SELECT * FROM v_users WHERE id = ?"#)
            .bind(user_id)
            .fetch_one(self.db.as_ref())
            .await
    }
}
