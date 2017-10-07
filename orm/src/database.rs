use dao::{Dao, Rows, Value};

use error::DbError;

pub trait Database {
    fn execute_sql_with_return(&self, sql: &str, param: &[Value]) -> Result<Rows, DbError>;

}
