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

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Permission {
    ViewAnyUser,
    ManageUsers,
    ManagePermissions,
}
