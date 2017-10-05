use r2d2;
use postgres;

use r2d2_postgres;
use r2d2_postgres::TlsMode;
use postgres::Connection;
use std::ops::Deref;

pub fn init_pool(db_url: &str) -> r2d2::Pool<r2d2_postgres::PostgresConnectionManager>{
    let config = r2d2::Config::default();
    let manager = r2d2_postgres::PostgresConnectionManager::new(db_url, TlsMode::None).unwrap();
    r2d2::Pool::new(config, manager).expect("Unable to create a pool for postgres connection manager")

}

pub struct PostgresIns(pub r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>);



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
            },
            _ => {
                panic!("wrong platform");
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
            },
            _ => {
                panic!("wrong platform");
            }
        }
    }
}
