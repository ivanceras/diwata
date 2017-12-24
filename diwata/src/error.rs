use rustorm::error::DbError;
use intel::error::IntelError;

#[derive(Debug)]
pub enum ServiceError {
    GenericError(String),
    PoolResourceError,
    IntelError(IntelError),
    DbError(DbError),
}

impl From<DbError> for ServiceError {
    fn from(e: DbError) -> ServiceError {
        ServiceError::DbError(e)
    }
}

impl From<IntelError> for ServiceError {
    fn from(e: IntelError) -> ServiceError {
        ServiceError::IntelError(e)
    }
}
