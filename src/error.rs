use log::error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NotFound(String),
    NotUnique(String),
    Other,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Other => write!(f, "Something went wrong. Please try again later."),
            Error::NotUnique(_) => write!(f, "That resource already exists."),
            Error::NotFound(_) => write!(f, "The requested resource was not found."),
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Error::NotFound("Row not found".to_string()),
            sqlx::Error::Database(err) => {
                if err.message().contains("UNIQUE") || err.code().unwrap_or_default() == "23505" {
                    Error::NotUnique(err.to_string())
                } else {
                    error!("Database error: {}", err);
                    Error::Other
                }
            }
            _ => {
                error!("Database error: {}", err);
                Error::Other
            }
        }
    }
}
