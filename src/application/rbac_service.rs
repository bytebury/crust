use std::sync::Arc;

use sqlx::SqlitePool;

use crate::{
    domain::rbac::{Permission, Role},
    infrastructure::db::RbacRepository,
};

pub struct RbacService {
    rbac_repository: RbacRepository,
}
impl RbacService {
    pub fn new(db: &Arc<SqlitePool>) -> Self {
        Self {
            rbac_repository: RbacRepository::new(db),
        }
    }

    pub async fn is_admin(&self, user_id: i64) -> bool {
        self.rbac_repository
            .get_user_roles(user_id)
            .await
            .contains(&Role::Admin)
    }

    pub async fn get_user_roles(&self, user_id: i64) -> Vec<Role> {
        self.rbac_repository.get_user_roles(user_id).await
    }

    pub async fn get_user_permissions(&self, user_id: i64) -> Vec<Permission> {
        self.rbac_repository.get_user_permissions(user_id).await
    }

    pub async fn check_permission(&self, user_id: i64, permission: &Permission) -> bool {
        if self.is_admin(user_id).await {
            return true;
        }

        self.rbac_repository
            .check_user_permission(user_id, permission)
            .await
    }
}
