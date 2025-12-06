use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;

use crate::{domain::rbac::Role, util::pagination::Paginatable};

#[derive(Serialize, FromRow, Clone)]
pub struct AuditUser {
    pub id: i64,
    pub email: String,
    pub verified: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub full_name: String,
    pub image_url: String,
    pub role: Role,
    pub stripe_customer_id: Option<String>,
    pub country_id: Option<i64>,
    pub country_code: Option<String>,
    pub country_name: Option<String>,
    pub country_locked: bool,
    pub region_id: Option<i64>,
    pub country_region: Option<String>,
    pub locked: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Paginatable for AuditUser {
    fn table_name() -> &'static str {
        "audit_users"
    }
}
