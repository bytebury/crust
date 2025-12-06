use crate::{
    DbPool, DbResult,
    domain::{AuditUser, NewUser, UpdateUser, User},
    infrastructure::db::{AuditUserRepository, UserRepository},
    util::pagination::{PaginatedResponse, Pagination},
};

pub struct UserService {
    user_repository: UserRepository,
    audit_user_repository: AuditUserRepository,
}

impl UserService {
    pub fn new(db: DbPool) -> Self {
        Self {
            user_repository: UserRepository::new(db.clone()),
            audit_user_repository: AuditUserRepository::new(db.clone()),
        }
    }

    pub async fn find_by_id(&self, user_id: i64) -> DbResult<AuditUser> {
        self.audit_user_repository.find_by_id(user_id).await
    }

    pub async fn find_by_email(&self, email: &str) -> DbResult<Option<User>> {
        self.user_repository.find_by_email(email).await
    }

    pub async fn update(&self, user: &UpdateUser) -> DbResult<AuditUser> {
        self.audit_user_repository.update(user).await
    }

    pub async fn search(
        &self,
        pagination: &Pagination,
        search: &str,
    ) -> PaginatedResponse<AuditUser> {
        self.audit_user_repository.search(pagination, search).await
    }

    pub async fn create(&self, user: &NewUser) -> DbResult<User> {
        self.user_repository.create(user).await
    }
}
