use crate::application::rbac::Role;
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
        sqlx::query_as!(User, "SELECT * FROM users_view WHERE id = ?", user_id)
            .fetch_one(self.db.as_ref())
            .await
            .map_err(Into::into)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        sqlx::query_as!(
            User,
            r#"
	            SELECT
	                id as "id!",
	                email,
	                first_name,
	                last_name,
	                full_name,
	                image_url,
	                role,
	                stripe_customer_id,
	                country_id,
	                verified,
	                locked,
	                created_at,
	                updated_at,
	                country_name,
	                country_code,
	                country_locked
	            FROM users_view
	            WHERE email = LOWER(?)
            "#,
            email
        )
        .fetch_optional(self.db.as_ref())
        .await
        .map_err(Into::into)
    }

    pub async fn update(&self, user: &UpdateUser) -> Result<User> {
        let _ = sqlx::query_scalar!(
            "UPDATE users SET role = ?, locked = ? WHERE id = ?",
            user.role,
            user.locked,
            user.id
        )
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
        let mut role = Role::User;
        let num_users: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
            .fetch_one(self.db.as_ref())
            .await?;

        if num_users == 0 {
            role = Role::Admin;
        }

        let user_id = sqlx::query_scalar!(
            r#"
		    INSERT INTO users (
	            email, full_name, first_name, last_name, image_url, country_id, verified, locked, role
	        )
	        VALUES (LOWER(?), ?, ?, ?, ?, ?, ?, ?, ?)
	        RETURNING id
			"#,
            user.email,
            user.full_name,
            user.first_name,
            user.last_name,
            user.image_url,
            user.country_id,
            user.verified,
            user.locked,
            role
        )
        .fetch_one(self.db.as_ref())
        .await?;

        self.find_by_id(user_id).await
    }
}
