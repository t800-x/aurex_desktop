use thiserror::Error;

#[derive(Debug, Error)]
pub enum LibraryError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Could not resolve app data directory")]
    NoAppDir,

    #[error("Record not found")]
    NotFound,
}

pub type Result<T> = std::result::Result<T, LibraryError>;
