use rustorm::DbError;
use std::{
    error::Error,
    fmt,
};

#[derive(Debug)]
pub enum IntelError {
    CacheServiceError,
    DbError(DbError),
    ParamParseError(String),
    /// no matching window
    NoMatching,
    NotFound,
}

impl fmt::Display for IntelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Error for IntelError {}

impl From<DbError> for IntelError {
    fn from(e: DbError) -> Self {
        IntelError::DbError(e)
    }
}
