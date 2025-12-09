use sqlx::SqlitePool;
use std::sync::Arc;

use crate::AppState;

pub use crate::error::*;
pub use crate::filters::*;

pub type SharedState = Arc<AppState>;
pub type DbPool = Arc<SqlitePool>;
