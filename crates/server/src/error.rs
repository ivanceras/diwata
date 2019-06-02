use diwata_intel::error::IntelError;
use rustorm::error::DbError;
use std::{
    error::Error,
    fmt,
};

#[derive(Debug)]
pub enum ServiceError {
    GenericError(String),
    PoolResourceError,
    CacheLockError,
    IntelError(IntelError),
    DbError(DbError),
    NoDbUrlSpecified,
    NotFound,
    RequiredCredentialsNotFound,
}

impl Error for ServiceError {}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
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
