use log::error;

pub type DbResult<T> = Result<T, DbError>;

#[derive(Debug)]
pub enum DbError {
    NotFound(String),
    NotUnique(String),
    Other,
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DbError::NotFound(msg) => write!(f, "{msg}"),
            DbError::NotUnique(msg) => write!(f, "{msg}"),
            DbError::Other => write!(f, "Something went wrong. Please try again later."),
        }
    }
}

impl From<sqlx::Error> for DbError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => DbError::NotFound("Row not found".to_string()),
            sqlx::Error::Database(err) => {
                if err.message().contains("UNIQUE") || err.code().unwrap_or_default() == "23505" {
                    DbError::NotUnique(err.to_string())
                } else {
                    error!("Database error: {}", err);
                    DbError::Other
                }
            }
            _ => {
                error!("Database error: {}", err);
                DbError::Other
            }
        }
    }
}
