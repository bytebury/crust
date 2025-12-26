use sqlx::SqlitePool;
use std::sync::Arc;

use crate::AppState;

pub use crate::application::*;
pub use crate::domain::*;
pub use crate::error::*;
pub use crate::filters;
pub use crate::routes::SharedContext;
pub use askama::Template;
pub use askama_web::WebTemplate;
pub use chrono::prelude::*;
pub use serde::{Deserialize, Serialize};
pub use sqlx::prelude::*;

pub type SharedState = Arc<AppState>;
pub type DbPool = Arc<SqlitePool>;
