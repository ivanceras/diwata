
use r2d2;
#[cfg(feature = "with-postgres")]
use r2d2_postgres::PostgresConnectionManager;
#[cfg(feature = "with-postgres")]
use postgres::Connection;
use std::convert::TryFrom;
use platform::Platform;
use error::{ParseError,ConnectError};
use std::collections::BTreeMap;
use url;
#[cfg(feature = "with-postgres")]
use pg;

pub struct Pool<'a>(BTreeMap<&'a str, Conn>);
pub enum Conn{
    #[cfg(feature = "with-postgres")]
    PoolPg(r2d2::Pool<PostgresConnectionManager>),
}


impl<'a> Pool<'a> {

    pub fn new() -> Self {
        Pool(BTreeMap::new())
    }

    /// ensure that a connection pool for this db_url exist
    fn ensure(&mut self, db_url: &'a str) -> Result<(), ConnectError> {
        let platform:Result<Platform, _> = TryFrom::try_from(db_url);
        match platform{
            Ok(platform) => {
                match platform {
                    #[cfg(feature = "with-postgres")]
                    Platform::Postgres => {
                        let pool_pg = pg::init_pool(db_url);
                        if self.0.get(db_url).is_none(){
                            self.0.insert(db_url, Conn::PoolPg(pool_pg));
                        }
                        Ok(())
                    },
                    Platform::Unsupported(scheme) => {
                        Err(ConnectError::UnsupportedDb(scheme))
                    }
                }
            },
            Err(e) => Err(ConnectError::ParseError(e))
        }
    }

    /// connect to existing db pool, or to a new pool if it hasn't been created yet
    pub fn connect(&mut self, db_url: &'a str) -> Result<&Conn, ConnectError> {
        self.ensure(db_url)?;
        let platform:Result<Platform, ParseError> = TryFrom::try_from(db_url);
        match platform{
            Ok(platform) => {
                match platform {
                    #[cfg(feature = "with-postgres")]
                    Platform::Postgres => {
                        let conn:Option<&Conn> = self.0.get(db_url);
                        if let Some(conn) = conn {
                            Ok(conn)
                        }
                        else{
                            Err(ConnectError::NoSuchPoolConnection)
                        }
                    },
                    Platform::Unsupported(scheme) => {
                        Err(ConnectError::UnsupportedDb(scheme))
                    }
                }
            },
            Err(e) => Err(ConnectError::ParseError(e))
        }
    }

}

#[cfg(test)]
mod tests_pg {
    use super::*;

    #[test]
    fn connect() {
        let db_url = "postgres://postgres:p0stgr3s@localhost:5432/rforum";
        let mut pool = Pool::new();
        pool.ensure(db_url);
        let pooled = pool.connect(db_url);
        match pooled{
            Ok(ref conn) => println!("ok"),
            Err(ref e) => eprintln!("error: {:?}", e),
        }
        assert!(pooled.is_ok());
    }

    #[test]
    fn connect_no_ensure(){
        let db_url = "postgres://postgres:p0stgr3s@localhost:5432/rforum";
        let mut pool = Pool::new();
        assert!(pool.connect(db_url).is_ok());
    }

}
