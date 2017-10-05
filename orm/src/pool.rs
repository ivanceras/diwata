cfg_if! {if #[cfg(feature = "with-postgres")]{
    use r2d2_postgres::PostgresConnectionManager;
    use pg::{self, PostgresDB};
    use r2d2;
}}
use std::convert::TryFrom;
use platform::Platform;
use error::{ParseError, ConnectError};
use std::collections::BTreeMap;
use platform::DBPlatform;

pub struct Pool<'a>(BTreeMap<&'a str, ConnPool>);
pub enum ConnPool {
    #[cfg(feature = "with-postgres")]
    PoolPg(r2d2::Pool<PostgresConnectionManager>),
}

pub enum PooledConn {
    #[cfg(feature = "with-postgres")]
    PooledPg(r2d2::PooledConnection<PostgresConnectionManager>),
}


impl<'a> Pool<'a> {
    pub fn new() -> Self {
        Pool(BTreeMap::new())
    }

    /// ensure that a connection pool for this db_url exist
    fn ensure(&mut self, db_url: &'a str) -> Result<(), ConnectError> {
        let platform: Result<Platform, _> = TryFrom::try_from(db_url);
        match platform {
            Ok(platform) => {
                match platform {
                    #[cfg(feature = "with-postgres")]
                    Platform::Postgres => {
                        let pool_pg = pg::init_pool(db_url);
                        if self.0.get(db_url).is_none() {
                            self.0.insert(db_url, ConnPool::PoolPg(pool_pg));
                        }
                        Ok(())
                    }
                    Platform::Unsupported(scheme) => Err(ConnectError::UnsupportedDb(scheme)),
                }
            }
            Err(e) => Err(ConnectError::ParseError(e)),
        }
    }

    /// get the pool for this specific db_url, create one if it doesn't have yet.
    fn get_pool(&mut self, db_url: &'a str) -> Result<&ConnPool, ConnectError> {
        self.ensure(db_url)?;
        let platform: Result<Platform, ParseError> = TryFrom::try_from(db_url);
        match platform {
            Ok(platform) => {
                match platform {
                    #[cfg(feature = "with-postgres")]
                    Platform::Postgres => {
                        let conn: Option<&ConnPool> = self.0.get(db_url);
                        if let Some(conn) = conn {
                            Ok(conn)
                        } else {
                            Err(ConnectError::NoSuchPoolConnection)
                        }
                    }
                    Platform::Unsupported(scheme) => Err(ConnectError::UnsupportedDb(scheme)),
                }
            }
            Err(e) => Err(ConnectError::ParseError(e)),
        }
    }

    /// get a usable database connection from
    pub fn connect(&mut self, db_url: &'a str) -> Result<PooledConn, ConnectError> {
        let pool = self.get_pool(db_url)?;
        match *pool {
            #[cfg(feature = "with-postgres")]
            ConnPool::PoolPg(ref pool_pg) => {
                let pooled_conn = pool_pg.get();
                match pooled_conn {
                    Ok(pooled_conn) => Ok(PooledConn::PooledPg(pooled_conn)),
                    Err(e) => Err(ConnectError::Timeout(e)),
                }
            }
        }
    }

    /// get a database instance with a connection, ready to send sql statements
    pub fn db(&mut self, db_url: &'a str) -> Result<DBPlatform, ConnectError> {
        let pooled_conn = self.connect(db_url)?;
        match pooled_conn {
            #[cfg(feature = "with-postgres")]
            PooledConn::PooledPg(pooled_pg) => Ok(DBPlatform::Postgres(PostgresDB(pooled_pg))),
        }
    }

}

#[cfg(test)]
#[cfg(feature = "with-postgres")]
mod tests_pg {
    use super::*;

    #[test]
    fn connect() {
        let db_url = "postgres://postgres:p0stgr3s@localhost:5432/rforum";
        let mut pool = Pool::new();
        pool.ensure(db_url).is_ok();
        let pooled = pool.get_pool(db_url);
        match pooled {
            Ok(_) => println!("ok"),
            Err(ref e) => eprintln!("error: {:?}", e),
        }
        assert!(pooled.is_ok());
    }

    #[test]
    fn connect_no_ensure() {
        let db_url = "postgres://postgres:p0stgr3s@localhost:5432/rforum";
        let mut pool = Pool::new();
        assert!(pool.get_pool(db_url).is_ok());
    }

}
