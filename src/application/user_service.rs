use crate::domain::user::{NewUser, UpdateUser, User};
use crate::prelude::*;
use crate::util::pagination::*;

pub struct UserService {
    db: DbPool,
}

impl UserService {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }

    pub async fn find_by_id(&self, user_id: i64) -> Result<User> {
        sqlx::query_as(r#"SELECT * FROM users_view WHERE id = ?"#)
            .bind(user_id)
            .fetch_one(self.db.as_ref())
            .await
            .map_err(Into::into)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        sqlx::query_as(r#"SELECT * FROM users_view WHERE email = LOWER(?)"#)
            .bind(email)
            .fetch_optional(self.db.as_ref())
            .await
            .map_err(Into::into)
    }

    pub async fn update(&self, user: &UpdateUser) -> Result<User> {
        let _ = sqlx::query(r#"UPDATE users SET role = ?, locked = ? WHERE id = ?"#)
            .bind(&user.role)
            .bind(user.locked)
            .bind(user.id)
            .execute(self.db.as_ref())
            .await?;

        self.find_by_id(user.id).await
    }

    pub async fn search(&self, pagination: &Pagination, search: &str) -> PaginatedResponse<User> {
        let pattern = &format!("%{}%", search.to_lowercase());

        User::paginate_filter(
            &self.db,
            pagination,
            Some(r#"LOWER(full_name) LIKE ? OR LOWER(email) LIKE ? ORDER BY updated_at DESC"#),
            vec![pattern, pattern],
        )
        .await
        .unwrap()
    }

    pub async fn create(&self, user: &NewUser) -> Result<User> {
        let user_id = sqlx::query_scalar(
            r#"
		    INSERT INTO users (
	            email, full_name, first_name, last_name, image_url, country_id, verified, locked
	        )
	        VALUES (LOWER(?), LOWER(?), LOWER(?), LOWER(?), ?, ?, ?, ?)
	        RETURNING id
			"#,
        )
        .bind(&user.email)
        .bind(&user.full_name)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.image_url)
        .bind(user.country_id)
        .bind(user.verified)
        .bind(user.locked)
        .fetch_one(self.db.as_ref())
        .await?;

        self.find_by_id(user_id).await
    }
}
