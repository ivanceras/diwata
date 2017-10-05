use r2d2;
use r2d2_postgres;
use r2d2_postgres::TlsMode;
use database::Database;
use dao::Value;
use error::DbError;
use dao::Rows;
use postgres::types::{ToSql,FromSql,Type};
use error::PlatformError;
use postgres::rows::Row;
use postgres::types;
use uuid::Uuid;
use chrono::offset::Utc;
use chrono::{NaiveDate, DateTime};
use postgres;
use postgres::types::IsNull;
use std::error::Error;
use error::ConvertError;

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
                                //let value: Option<Result<Value, DbError>> = convert_from_sql_to_value(c.type_(), &r, i);
                                let value: Option<Result<OwnedPgValue, postgres::Error>> = r.get_opt(i);
                                match value{
                                    Some(value) => match value{
                                        Ok(value) =>  record.push(value.0),
                                        Err(e) => {
                                            return Err(DbError::PlatformError(PlatformError::PostgresError(e)))
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
                    Err(e) => Err(DbError::PlatformError(PlatformError::SqlError(e, sql.to_string()))),
                }
            },
            Err(e) => Err(DbError::PlatformError(PlatformError::SqlError(e, sql.to_string())))
        }
    }
}


#[allow(unused)]
fn to_sql_types1<'a>(values: &'a [Value] ) -> Vec<&'a ToSql> {
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
            Value::Nil => panic!("This should be null"),
        }
    }
    sql_types
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
            ($ty: ident, $variant: ident ) => {
                $ty::from_sql(ty, raw).map(|v|OwnedPgValue(Value::$variant(v)))
            }
        }

        match *ty {
            types::BOOL => match_type!(bool, Bool), 
            types::INT2  => match_type!(i16, Smallint),
            types::INT4  => match_type!(i32, Int),
            types::INT8  => match_type!(i64, Bigint),
            types::FLOAT4 => match_type!(f32, Float),
            types::FLOAT8 => match_type!(f64, Double),
            types::TEXT | types::VARCHAR => match_type!(String, Text),
            types::UUID => match_type!(Uuid, Uuid),
            types::TIMESTAMPTZ | types::TIMESTAMP => match_type!(DateTime,Timestamp),
            _ => panic!("unable to convert from {:?}", ty), 
        }

    }
    fn accepts(ty: &Type) -> bool{
        match *ty {
            types::BOOL => true,
            types::INT2 | types::INT4 | types::INT8 => true,
            types::FLOAT4 | types::FLOAT8 => true,
            types::TEXT | types::VARCHAR => true,
            types::UUID => true,
            types::TIMESTAMPTZ | types::TIMESTAMP => true,
            types::UNKNOWN => false,
            _ => false 
        }
    }

    fn from_sql_null(_ty: &Type) -> Result<Self, Box<Error + Sync + Send>> { 
        Ok(OwnedPgValue(Value::Nil))
    }
    fn from_sql_nullable(
        ty: &Type, 
        raw: Option<&[u8]>
    ) -> Result<Self, Box<Error + Sync + Send>> { 

        macro_rules! match_type {
            ($ty: ident, $variant: ident ) => {
                $ty::from_sql_nullable(ty, raw).map(|v|OwnedPgValue(Value::$variant(v)))
            }
        }
        match *ty {
            types::BOOL => match_type!(bool, Bool), 
            types::INT2  => match_type!(i16, Smallint),
            types::INT4  => match_type!(i32, Int),
            types::INT8  => match_type!(i64, Bigint),
            types::FLOAT4 => match_type!(f32, Float),
            types::FLOAT8 => match_type!(f64, Double),
            types::TEXT | types::VARCHAR => match_type!(String, Text),
            types::UUID => match_type!(Uuid, Uuid),
            types::TIMESTAMPTZ | types::TIMESTAMP => match_type!(DateTime,Timestamp),
            _ => panic!("unable to convert from {:?}", ty), 
        }

    }
}


/// hacky way to convert to value, since FromSql is Sized
#[allow(unused)]
fn convert_from_sql_to_value(ty: &Type, row: &Row, index: usize) -> Option<Result<Value, DbError>> {

    macro_rules! match_type  {
        ($ty: ty, $variant: ident) => {
            {
                let value: Option<Result<$ty, postgres::Error>> = row.get_opt(index);
                value.map(|v:Result<$ty, postgres::Error>| {
                        match v{
                            Ok(value) => Ok(Value::$variant(value)),
                            Err(e) => {
                               Err(DbError::PlatformError(PlatformError::PostgresError(e)))
                            }
                        }
                    }
                )
            }
        }
    }
        
    match *ty {
        types::BOOL => {
            match_type!(bool, Bool)
        },
        types::INT2 => {
            match_type!(i16, Smallint)
        }
        types::INT4 => {
            match_type!(i32, Int)
        }
        types::INT8 => {
            match_type!(i64, Bigint)
        }
        types::FLOAT4 => {
            match_type!(f32, Float)
        },
        types::FLOAT8 => {
            match_type!(f64, Double)
        },
        types::TEXT | types::VARCHAR => {
            match_type!(String, Text)
        },
        types::UUID => {
            match_type!(Uuid, Uuid)
        },
        types::DATE => {
            match_type!(NaiveDate, Date)
        }
        types::TIMESTAMPTZ | types::TIMESTAMP => {
            match_type!(DateTime<Utc>, Timestamp)
        },
        types::UNKNOWN => {
            Some(Err(DbError::ConvertError(ConvertError::UnknownDataType)))
        },

        _ => Some(Err(DbError::ConvertError(ConvertError::UnsupportedDataType(ty.name().to_string())))) 
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
    // no type hinted on 'Hello' will cause unknown type error
    #[test]
    fn test_unknown_type(){
        let mut pool = Pool::new();
        let db_url = "postgres://postgres:p0stgr3s@localhost/rforum";
        let db  = pool.db(db_url).unwrap();
        let values:Vec<Value> = vec![
            "hi".into(),
            true.into(),
            42.into(),
            1.0.into(),
        ];
        let rows:Result<Rows, DbError> = (&db).execute_sql_select("select 'Hello', $1::TEXT, $2::BOOL, $3::INT, $4::FLOAT", &values);
        assert!(!rows.is_ok());
    }

    #[test]
    fn using_values(){
        let mut pool = Pool::new();
        let db_url = "postgres://postgres:p0stgr3s@localhost/rforum";
        let db  = pool.db(db_url).unwrap();
        let values:Vec<Value> = vec![
            "hi".into(),
            true.into(),
            42.into(),
            1.0.into(),
        ];
        let rows:Result<Rows, DbError> = (&db).execute_sql_select("select 'Hello'::TEXT, $1::TEXT, $2::BOOL, $3::INT, $4::FLOAT", &values);
        println!("columns: {:#?}", rows);
        if let Ok(rows) = rows {
            for row in rows.iter(){
                println!("row {:?}", row);
                let v4:Result<f32, _> = row.get("float8");
                assert_eq!(v4.unwrap(), 1.0f32);// Hello was overwritten

                let v3:Result<i32, _> = row.get("int4");
                assert_eq!(v3.unwrap(), 42i32);// Hello was overwritten

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
        let db_url = "postgres://postgres:p0stgr3s@localhost/medical";
        let db  = pool.db(db_url).unwrap();
        let rows:Result<Rows, DbError> = (&db).execute_sql_select("select * from doctor", &[]);
        println!("columns: {:#?}", rows);
        if let Ok(rows) = rows {
            for row in rows.iter(){
                println!("row {:?}", row);
                let v4:Result<f32, _> = row.get("float8");
                assert_eq!(v4.unwrap(), 1.0f32);// Hello was overwritten

                let v3:Result<i32, _> = row.get("int4");
                assert_eq!(v3.unwrap(), 42i32);// Hello was overwritten

                let hi: Result<String, _> = row.get("text");
                assert_eq!(hi.unwrap(), "hi");
                
                let b: Result<bool, _> = row.get("bool");
                assert_eq!(b.unwrap(), true);
            }
        }
        panic!();
    }
}
