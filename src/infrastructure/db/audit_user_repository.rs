use crate::DbPool;
use crate::DbResult;
use crate::domain::AuditUser;
use crate::domain::UpdateUser;
use crate::util::pagination::Paginatable;
use crate::util::pagination::PaginatedResponse;
use crate::util::pagination::Pagination;

pub struct AuditUserRepository {
    db: DbPool,
}

impl AuditUserRepository {
    pub fn new(db: DbPool) -> Self {
        Self { db: db.clone() }
    }

    pub async fn find_by_id(&self, id: i64) -> DbResult<AuditUser> {
        sqlx::query_as(r#"SELECT * FROM audit_users WHERE id = ?"#)
            .bind(id)
            .fetch_one(self.db.as_ref())
            .await
            .map_err(Into::into)
    }

    pub async fn update(&self, user: &UpdateUser) -> DbResult<AuditUser> {
        let _ = sqlx::query(r#"UPDATE users SET role = ?, locked = ? WHERE id = ?"#)
            .bind(&user.role)
            .bind(user.locked)
            .bind(user.id)
            .execute(self.db.as_ref())
            .await?;

        self.find_by_id(user.id).await
    }

    pub async fn search(
        &self,
        pagination: &Pagination,
        search: &str,
    ) -> PaginatedResponse<AuditUser> {
        let pattern = &format!("%{}%", search.to_lowercase());

        AuditUser::paginate_filter(
            &self.db,
            pagination,
            Some(r#"LOWER(full_name) LIKE ? OR LOWER(email) LIKE ? ORDER BY updated_at DESC"#),
            vec![pattern, pattern],
        )
        .await
        .unwrap()
    }
}
