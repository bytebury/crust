use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, Serialize, Deserialize, Default)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Role {
    #[default]
    User,
    Admin,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Action {
    Read,
    Create,
    Update,
    Delete,
}

pub trait Can<T> {
    fn can(&self, action: Action, resource: &T) -> bool;
    fn cannot(&self, action: Action, resource: &T) -> bool {
        !self.can(action, resource)
    }
}
