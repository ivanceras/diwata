use dao::Value;
use dao::Rows;

use error::DbError;

pub trait Database{
    
    fn execute_select_sql(sql: &str, param: &[Value]) -> Result<Rows, DbError>;
}
