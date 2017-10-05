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

