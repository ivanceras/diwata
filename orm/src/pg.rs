use r2d2;
use r2d2_postgres;
use r2d2_postgres::TlsMode;
use database::Database;
use dao::Value;
use error::DbError;
use dao::Rows;
use postgres::types::{ToSql,Type};
use error::PlatformError;
use postgres::rows::Row;
use postgres::types;
use uuid::Uuid;
use chrono::offset::Utc;
use chrono::DateTime;
use postgres;

pub fn init_pool(db_url: &str) -> r2d2::Pool<r2d2_postgres::PostgresConnectionManager>{
    let config = r2d2::Config::default();
    let manager = r2d2_postgres::PostgresConnectionManager::new(db_url, TlsMode::None).unwrap();
    r2d2::Pool::new(config, manager).expect("Unable to create a pool for postgres connection manager")

}

pub struct PostgresDB(pub r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>);

impl Database for PostgresDB{
    
    fn execute_sql_select(&self, sql: &str, param: &[Value]) -> Result<Rows, DbError> {
        let stmt = self.0.prepare(&sql);
        match stmt{
            Ok(stmt) => {
                let sql_types = to_sql_types(param);
                let rows = stmt.query(&sql_types);
                match rows {
                    Ok(rows) => {
                        let columns = rows.columns();
                        let column_names:Vec<String> = columns.iter().map(|c| c.name().to_string() ).collect();
                        let mut records = Rows::new(column_names);
                        for r in rows.iter(){
                            let mut record:Vec<Value> = vec![];
                            for c in columns{
                                let column_name = c.name();
                                let value = convert_from_sql_to_value(c.type_(), &r, column_name);
                                if let Some(value) = value {
                                    record.push(value);
                                }
                            }
                            records.push(record);
                        }
                        Ok(records)
                    },
                    Err(e) => Err(DbError::PlatformError(PlatformError::PostgresError(e))),
                }
            },
            Err(e) => Err(DbError::PlatformError(PlatformError::PostgresError(e)))
        }
    }
}

fn to_sql_types<'a>(values: &'a [Value] ) -> Vec<&'a ToSql> {
    let mut sql_types: Vec<&ToSql> = vec![];
    for t in values {
        match *t {
            Value::Bool(ref v) => sql_types.push(v),
            Value::Tinyint(ref v) => sql_types.push(v),
            Value::Smallint(ref v) => sql_types.push(v),
            Value::Int(ref v) => sql_types.push(v),
            Value::Bigint(ref v) => sql_types.push(v),
            Value::Float(ref v) => sql_types.push(v),
            Value::Double(ref v) => sql_types.push(v),
            Value::Blob(ref v) => sql_types.push(v),
            Value::Text(ref v) => sql_types.push(v),
            Value::Str(ref v) => sql_types.push(v),
            Value::Uuid(ref v) => sql_types.push(v),
            Value::Date(ref v) => sql_types.push(v),
            Value::Timestamp(ref v) => sql_types.push(v),
        }
    }
    sql_types
}


/// hacky way to convert to value, since FromSql is Sized
fn convert_from_sql_to_value(ty: &Type, row: &Row, column: &str) -> Option<Value> {
    match *ty {
        types::UUID => {
            let value: Option<Result<Uuid, postgres::Error>> = row.get_opt(column);
            value.map(|v|v.ok().map(|value|Value::Uuid(value))).unwrap_or(None)
        },
        types::TIMESTAMPTZ => {
            let value: Option<Result<DateTime<Utc>, postgres::Error>> = row.get_opt(column);
            value.map(|v|v.ok().map(|value|Value::Timestamp(value))).unwrap_or(None)
        },
        types::TEXT => {
            let value: Option<Result<String, postgres::Error>> = row.get_opt(column);
            value.map(|v|v.ok().map(|value|Value::Text(value))).unwrap_or(None)
        }
        _ => panic!("Type {:?} is not covered!", ty),
    }
}


#[cfg(test)]
mod test{

    use pool::{Pool, PooledConn};
    use postgres::Connection;
    use std::ops::Deref;

    #[test]
    fn connect_test_query(){
        let db_url = "postgres://postgres:p0stgr3s@localhost:5432/rforum";
        let mut pool = Pool::new();
        let conn = pool.connect(db_url);
        assert!(conn.is_ok());
        let conn: PooledConn = conn.unwrap();
        match conn{
            PooledConn::PooledPg(ref pooled_pg) => {
                let rows = pooled_pg.query("select 42, 'life'", &[]).unwrap();
                for row in rows.iter(){
                    let n: i32 = row.get(0);
                    let l: String = row.get(1);
                    assert_eq!(n, 42);
                    assert_eq!(l, "life");
                }
            }
        }
    }
    #[test]
    fn connect_test_query_explicit_deref(){
        let db_url = "postgres://postgres:p0stgr3s@localhost:5432/rforum";
        let mut pool = Pool::new();
        let conn = pool.connect(db_url);
        assert!(conn.is_ok());
        let conn: PooledConn = conn.unwrap();
        match conn{
            PooledConn::PooledPg(ref pooled_pg) => {
                let c: &Connection = pooled_pg.deref(); //explicit deref here
                let rows = c.query("select 42, 'life'", &[]).unwrap();
                for row in rows.iter(){
                    let n: i32 = row.get(0);
                    let l: String = row.get(1);
                    assert_eq!(n, 42);
                    assert_eq!(l, "life");
                }
            }
        }
    }
}
