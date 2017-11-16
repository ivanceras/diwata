use rustorm::error::DbError;

#[derive(Debug)]
pub enum ServiceError{
    PoolResourceError,
    DbError(DbError)
}

impl From<DbError> for ServiceError{
    
    fn from(e: DbError) -> ServiceError {
        ServiceError::DbError(e)
    }
}
