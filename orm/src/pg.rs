use r2d2;
use postgres;

use r2d2_postgres::PostgresConnectionManager;
use r2d2_postgres::TlsMode;
use postgres::Connection;
use std::ops::Deref;

pub fn init_pool(db_url: &str) -> r2d2::Pool<PostgresConnectionManager>{
    let config = r2d2::Config::default();
    let manager = PostgresConnectionManager::new(db_url, TlsMode::None).unwrap();
    r2d2::Pool::new(config, manager).expect("Unable to create a pool for postgres connection manager")

}


#[cfg(test)]
mod test{

    use pool::{Pool,Conn};

    #[test]
    fn connect_test_query(){
        let db_url = "postgres://postgres:p0stgr3s@localhost:5432/rforum";
        let mut pool = Pool::new();
        let conn = pool.connect(db_url);
        assert!(conn.is_ok());
        let conn = conn.unwrap();
        match *conn{
            Conn::PoolPg(ref pool_pg) => {
                let conn_pg = pool_pg.get().unwrap();
                let rows = &conn_pg.query("select 42, 'life'", &[]).unwrap();
                for row in rows{
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
