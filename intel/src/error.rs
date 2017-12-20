use rustorm::DbError;

#[derive(Debug)]
pub enum IntelError {
    CacheServiceError,
    DbError(DbError),
    ParamParseError(String),
}

impl From<DbError> for IntelError {
    fn from(e: DbError) -> Self {
        IntelError::DbError(e)
    }
}
