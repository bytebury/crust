use log::error;

#[derive(Debug)]
pub enum DatabaseError {
    NotFound(String),
    NotUnique(String),
    Other,
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DatabaseError::NotFound(msg) => write!(f, "{msg}"),
            DatabaseError::NotUnique(msg) => write!(f, "{msg}"),
            DatabaseError::Other => write!(f, "Something went wrong. Please try again later."),
        }
    }
}

impl From<sqlx::Error> for DatabaseError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => DatabaseError::NotFound("Row not found".to_string()),
            sqlx::Error::Database(err) => {
                if err.message().contains("UNIQUE") || err.code().unwrap_or_default() == "23505" {
                    DatabaseError::NotUnique(err.to_string())
                } else {
                    error!("Database error: {}", err);
                    DatabaseError::Other
                }
            }
            _ => {
                error!("Database error: {}", err);
                DatabaseError::Other
            }
        }
    }
}
