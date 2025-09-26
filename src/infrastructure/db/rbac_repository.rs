use std::sync::Arc;

use sqlx::{SqlitePool, query_as};

use crate::domain::rbac::{Permission, Role};

pub struct RbacRepository {
    db: Arc<SqlitePool>,
}
impl RbacRepository {
    pub fn new(db: &Arc<SqlitePool>) -> Self {
        Self { db: db.clone() }
    }

    pub async fn get_user_roles(&self, user_id: i64) -> Vec<Role> {
        let result: Result<Vec<(Role,)>, sqlx::Error> = query_as(
            r#"
        SELECT r.name FROM roles r 
         INNER JOIN user_roles ur ON ur.role_id = r.id
         WHERE ur.user_id = ?
        "#,
        )
        .bind(user_id)
        .fetch_all(self.db.as_ref())
        .await;

        match result {
            Ok(rows) => rows.into_iter().map(|(name,)| name).collect(),
            Err(_) => Vec::new(),
        }
    }

    pub async fn get_user_permissions(&self, user_id: i64) -> Vec<Permission> {
        let result: Result<Vec<(Permission,)>, sqlx::Error> = query_as(
            r#"
        SELECT p.name
        FROM permissions p
        INNER JOIN role_permissions rp ON rp.permission_id = p.id
        INNER JOIN roles r ON r.id = rp.role_id
        INNER JOIN user_roles ur ON ur.role_id = r.id
        WHERE ur.user_id = ?
        "#,
        )
        .bind(user_id)
        .fetch_all(self.db.as_ref())
        .await;

        match result {
            Ok(rows) => rows.into_iter().map(|(name,)| name).collect(),
            Err(_) => Vec::new(),
        }
    }

    pub async fn check_user_permission(&self, user_id: i64, permission: &Permission) -> bool {
        let count: Result<(i64,), sqlx::Error> = query_as(
            r#"
        SELECT COUNT(1)
          FROM permissions p
         INNER JOIN role_permissions rp ON rp.permission_id = p.id
         INNER JOIN user_roles ur ON ur.role_id = rp.id
         WHERE ur.user_id = ? AND p.name = ?
        "#,
        )
        .bind(user_id)
        .bind(permission)
        .fetch_one(self.db.as_ref())
        .await;

        match count {
            Ok((count,)) => count > 0,
            Err(_) => false,
        }
    }
}
