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
    #[error("slots are numbered 1 to 6")]
    InvalidSlot,
    #[error("this game isn't owned yet; mark it as owned first")]
    NotOwned,
    #[error("that slot is already taken")]
    SlotTaken,
    #[error("this game is already on the shelf")]
    AlreadyOnShelf,
    #[error("there is no game in that slot")]
    NotOnShelf,
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
            AppError::InvalidSlot => "invalid_slot",
            AppError::NotOwned => "not_owned",
            AppError::SlotTaken => "slot_taken",
            AppError::AlreadyOnShelf => "already_on_shelf",
            AppError::NotOnShelf => "not_on_shelf",
        };
        ErrorBody { code, message: self.to_string() }.serialize(serializer)
    }
}