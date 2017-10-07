use dao::{Dao, Rows, Value};

use error::DbError;

pub trait Database {
    fn execute_sql_select(&self, sql: &str, param: &[Value]) -> Result<Rows, DbError>;


    fn insert(&self, dao: &[Dao]) -> Result<Rows, DbError>;
}
