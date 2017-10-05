use dao::Value;
use dao::Rows;

use error::DbError;

pub trait Database {
    fn execute_sql_select(&self, sql: &str, param: &[Value]) -> Result<Rows, DbError>;
}
