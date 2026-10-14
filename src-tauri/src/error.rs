use serde::{Serialize, Serializer};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Db(#[from] rusqlite::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("not found")]
    NotFound,
    #[error("internal error: lock poisoned")]
    Lock,
}

pub type AppResult<T> = Result<T, AppError>;

#[derive(Serialize)]
struct ErrorBody {
    code: &'static str,
    message: String,
}

impl Serialize for AppError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let code = match self {
            AppError::Db(_) => "db",
            AppError::Io(_) => "io",
            AppError::NotFound => "not_found",
            AppError::Lock => "internal",
        };
        ErrorBody { code, message: self.to_string() }.serialize(serializer)
    }
}