use r2d2;
use r2d2_postgres;
use r2d2_postgres::TlsMode;
use database::Database;
use dao::{Value};
use error::DbError;
use dao::Rows;
use postgres;
use postgres::types::{self,ToSql,FromSql,Type};
use error::PlatformError;
use uuid::Uuid;
use chrono::{DateTime};
use postgres::types::IsNull;
use std::error::Error;
use std::fmt;

pub fn init_pool(db_url: &str) -> r2d2::Pool<r2d2_postgres::PostgresConnectionManager>{
    let config = r2d2::Config::default();
    let manager = r2d2_postgres::PostgresConnectionManager::new(db_url, TlsMode::None).unwrap();
    r2d2::Pool::new(config, manager).expect("Unable to create a pool for postgres connection manager")

}

pub struct PostgresDB(pub r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>);

impl Database for PostgresDB{
    
    fn execute_sql_with_return(&self, sql: &str, param: &[Value]) -> Result<Rows, DbError> {
        let stmt = self.0.prepare(&sql);
        match stmt{
            Ok(stmt) => {
                let pg_values = to_pg_values(param);
                let sql_types = to_sql_types(&pg_values);
                let rows = stmt.query(&sql_types);
                match rows {
                    Ok(rows) => {
                        let columns = rows.columns();
                        let column_names:Vec<String> = columns.iter().map(|c| c.name().to_string() ).collect();
                        let mut records = Rows::new(column_names);
                        for r in rows.iter(){
                            let mut record:Vec<Value> = vec![];
                            for (i,c) in columns.iter().enumerate(){
                                let _column_name = c.name();
                                let value: Option<Result<OwnedPgValue, postgres::Error>> = r.get_opt(i);
                                match value{
                                    Some(value) => match value{
                                        Ok(value) =>  record.push(value.0),
                                        Err(e) => {
                                            return Err(DbError::PlatformError(PlatformError::PostgresError(PostgresError::GenericError(e))))
                                        }
                                    },
                                    None => {
                                        record.push(Value::Nil);// Note: this is important to not mess the spacing of records
                                    }
                                }
                            }
                            records.push(record);
                        }
                        Ok(records)
                    },
                    Err(e) => Err(DbError::PlatformError(PlatformError::PostgresError(PostgresError::SqlError(e, sql.to_string())))),
                }
            },
            Err(e) => Err(DbError::PlatformError(PlatformError::PostgresError(PostgresError::SqlError(e, sql.to_string()))))
        }
    }

}



fn to_pg_values(values: &[Value]) -> Vec<PgValue> {
    values.iter().map(|v| PgValue(v)).collect()
}

fn to_sql_types<'a>(values: &'a Vec<PgValue> ) -> Vec<&'a ToSql> {
    let mut sql_types = vec![];
    for v in values.iter(){
        sql_types.push(&*v as &ToSql);
    }
    sql_types
}

#[derive(Debug)]
pub struct PgValue<'a>(&'a Value);

#[derive(Debug)]
pub struct OwnedPgValue(Value);

impl<'a> ToSql for PgValue<'a>{
    fn to_sql( &self, ty: &Type, out: &mut Vec<u8>) 
        -> Result<IsNull, Box<Error + 'static + Sync + Send>>{
        match *self.0{
            Value::Bool(ref v) => v.to_sql(ty, out),
            Value::Tinyint(ref v) => v.to_sql(ty, out),
            Value::Smallint(ref v) => v.to_sql(ty, out),
            Value::Int(ref v) => v.to_sql(ty, out),
            Value::Bigint(ref v) => v.to_sql(ty, out),
            Value::Float(ref v) => v.to_sql(ty, out),
            Value::Double(ref v) => v.to_sql(ty, out),
            Value::Blob(ref v) => v.to_sql(ty, out),
            Value::Char(ref v) => v.to_string().to_sql(ty, out),
            Value::Text(ref v) => v.to_sql(ty, out),
            Value::Str(ref v) => v.to_sql(ty, out),
            Value::Uuid(ref v) => v.to_sql(ty, out),
            Value::Date(ref v) => v.to_sql(ty, out),
            Value::Timestamp(ref v) => v.to_sql(ty, out),
            Value::Nil => Ok(IsNull::Yes),
        }
    }

    fn accepts(ty: &Type) -> bool{
        match *ty {
            types::BOOL => true,
            types::INT2 | types::INT4 | types::INT8 => true,
            types::FLOAT4 | types::FLOAT8 => true,
            types::TEXT | types::VARCHAR => true,
            types::BPCHAR=> true,
            types::UUID => true,
            types::TIMESTAMPTZ | types::TIMESTAMP => true,
            types::UNKNOWN => false,
            _ => false 
        }
 
    }

    to_sql_checked!();
}

impl FromSql for OwnedPgValue{
    fn from_sql(ty: &Type, raw: &[u8]) -> Result<Self, Box<Error + Sync + Send>>{
        macro_rules! match_type {
            ($variant: ident ) => {
                FromSql::from_sql(ty, raw).map(|v|OwnedPgValue(Value::$variant(v)))
            }
        }

        match *ty {
            types::BOOL => match_type!(Bool), 
            types::INT2  => match_type!(Smallint),
            types::INT4  => match_type!(Int),
            types::INT8  => match_type!(Bigint),
            types::FLOAT4 => match_type!(Float),
            types::FLOAT8 => match_type!(Double),
            types::TEXT | types::VARCHAR => match_type!(Text),
            types::BPCHAR => {
                let v: Result<String,_> = FromSql::from_sql(&types::TEXT, raw);
                match v{
                    Ok(v) => {
                        if v.chars().count() == 1 {
                            Ok( OwnedPgValue(Value::Char(v.chars().next().unwrap())))
                        }else {
                            Err(Box::new(PostgresError::ConvertStringToCharError(format!("More than 1 char in '{}'",v))))
                        }
                    },
                    Err(e) => Err(e)
                }
            }
            types::UUID => match_type!(Uuid),
            types::DATE => match_type!(Date),
            types::TIMESTAMPTZ | types::TIMESTAMP => match_type!(Timestamp),
            types::BYTEA => match_type!(Blob),
            _ => panic!("unable to convert from {:?}", ty), 
        }

    }
    fn accepts(ty: &Type) -> bool{
        match *ty {
            types::BOOL => true,
            types::INT2 | types::INT4 | types::INT8 => true,
            types::FLOAT4 | types::FLOAT8 => true,
            types::TEXT | types::VARCHAR => true,
            types::BPCHAR => true,
            types::UUID => true,
            types::DATE => true,
            types::TIMESTAMPTZ | types::TIMESTAMP => true,
            types::BYTEA => true,
            types::UNKNOWN => false,
            _ => panic!("can not accept type {:?}", ty), 
        }
    }

    fn from_sql_null(_ty: &Type) -> Result<Self, Box<Error + Sync + Send>> { 
        Ok(OwnedPgValue(Value::Nil))
    }
    fn from_sql_nullable(
        ty: &Type, 
        raw: Option<&[u8]>
    ) -> Result<Self, Box<Error + Sync + Send>> { 
        match raw{
            Some(raw) => Self::from_sql(ty, raw),
            None => Self::from_sql_null(ty), 
        }

    }
}

#[derive(Debug)]
pub enum PostgresError{
    GenericError(postgres::Error),
    SqlError(postgres::Error, String),
    ConvertStringToCharError(String),
}


impl Error for PostgresError {
    fn description(&self) -> &str{
        "postgres error"
    }

    fn cause(&self) -> Option<&Error> { 
        None
    }
}

impl fmt::Display for PostgresError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}


#[cfg(test)]
mod test{

    use super::*;
    use pool::{Pool, PooledConn};
    use postgres::Connection;
    use std::ops::Deref;
    use dao::Value;
    use dao::Rows;

    #[test]
    fn connect_test_query(){
        let db_url = "postgres://postgres:p0stgr3s@localhost:5432/sakila";
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
        let db_url = "postgres://postgres:p0stgr3s@localhost:5432/sakila";
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
    // no type hinted on 'Hello' will cause unknown type error
    #[test]
    fn test_unknown_type(){
        let mut pool = Pool::new();
        let db_url = "postgres://postgres:p0stgr3s@localhost/sakila";
        let db  = pool.db(db_url).unwrap();
        let values:Vec<Value> = vec![
            "hi".into(),
            true.into(),
            42.into(),
            1.0.into(),
        ];
        let rows:Result<Rows, DbError> = (&db).execute_sql_with_return("select 'Hello', $1::TEXT, $2::BOOL, $3::INT, $4::FLOAT", &values);
        assert!(!rows.is_ok());
    }

    #[test]
    fn using_values(){
        let mut pool = Pool::new();
        let db_url = "postgres://postgres:p0stgr3s@localhost/sakila";
        let db  = pool.db(db_url).unwrap();
        let values:Vec<Value> = vec![
            "hi".into(),
            true.into(),
            42.into(),
            1.0.into(),
        ];
        let rows:Result<Rows, DbError> = (&db).execute_sql_with_return("select 'Hello'::TEXT, $1::TEXT, $2::BOOL, $3::INT, $4::FLOAT", &values);
        println!("columns: {:#?}", rows);
        assert!(rows.is_ok());
        if let Ok(rows) = rows {
            for row in rows.iter(){
                println!("row {:?}", row);
                let v4:Result<f64, _> = row.get("float8");
                assert_eq!(v4.unwrap(), 1.0f64);

                let v3:Result<i32, _> = row.get("int4");
                assert_eq!(v3.unwrap(), 42i32);

                let hi: Result<String, _> = row.get("text");
                assert_eq!(hi.unwrap(), "hi");
                
                let b: Result<bool, _> = row.get("bool");
                assert_eq!(b.unwrap(), true);
            }
        }
    }

    #[test]
    fn with_nulls(){
        let mut pool = Pool::new();
        let db_url = "postgres://postgres:p0stgr3s@localhost/sakila";
        let db  = pool.db(db_url).unwrap();
        let rows:Result<Rows, DbError> = (&db).execute_sql_with_return("select 'rust'::TEXT AS name, NULL::TEXT AS schedule, NULL::TEXT AS specialty from actor", &[]);
        println!("columns: {:#?}", rows);
        assert!(rows.is_ok());
        if let Ok(rows) = rows {
            for row in rows.iter(){
                println!("row {:?}", row);
                let name:Result<Option<String>, _> = row.get("name");
                println!("name: {:?}", name);
                assert_eq!(name.unwrap().unwrap(), "rust");

                let schedule:Result<Option<String>, _> = row.get("schedule");
                println!("schedule: {:?}", schedule);
                assert_eq!(schedule.unwrap(), None);

                let specialty: Result<Option<String>, _> = row.get("specialty");
                println!("specialty: {:?}", specialty);
                assert_eq!(specialty.unwrap(), None);
            }
        }
    }
}
