use intel::error::IntelError;
use rustorm::error::DbError;

#[derive(Debug)]
pub enum ServiceError {
    GenericError(String),
    PoolResourceError,
    IntelError(IntelError),
    DbError(DbError),
    NoDbUrlSpecified,
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
