use std::sync::Arc;

use sqlx::{SqlitePool, query_as};

use crate::domain::User;

pub struct UserRepository {
    db: Arc<SqlitePool>,
}

impl UserRepository {
    pub fn new(db: &Arc<SqlitePool>) -> Self {
        Self { db: db.clone() }
    }

    pub async fn find_by_id(&self, id: i64) -> Result<User, sqlx::Error> {
        query_as(r#"SELECT * FROM users WHERE id = ?"#)
            .bind(id)
            .fetch_one(self.db.as_ref())
            .await
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        query_as(r#"SELECT * FROM users WHERE email = ?"#)
            .bind(email)
            .fetch_optional(self.db.as_ref())
            .await
    }

    pub async fn create(&self, user: User) -> Result<User, sqlx::Error> {
        query_as(
            r#"
            INSERT INTO users (email, full_name, first_name, last_name, image_url, verified) 
            VALUES (LOWER(?), LOWER(?), LOWER(?), LOWER(?), ?, ?)
            RETURNING *
        "#,
        )
        .bind(user.email)
        .bind(user.full_name)
        .bind(user.first_name)
        .bind(user.last_name)
        .bind(user.image_url)
        .bind(user.verified)
        .fetch_one(self.db.as_ref())
        .await
    }
}
