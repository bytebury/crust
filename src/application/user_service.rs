use std::sync::Arc;

use sqlx::SqlitePool;

use crate::{domain::User, infrastructure::db::UserRepository};

pub struct UserService {
    user_repository: UserRepository,
}
impl UserService {
    pub fn new(db: &Arc<SqlitePool>) -> Self {
        Self {
            user_repository: UserRepository::new(db),
        }
    }

    pub async fn find_by_id(&self, user_id: i64) -> Result<User, sqlx::Error> {
        self.user_repository.find_by_id(user_id).await
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        self.user_repository.find_by_email(email).await
    }

    pub async fn create(&self, user: User) -> Result<User, sqlx::Error> {
        self.user_repository.create(user).await
    }
}
