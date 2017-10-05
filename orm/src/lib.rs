#![feature(try_from)]
extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;
extern crate url;

use pg::ConnPg;
use std::collections::BTreeMap;
use error::ConnectError;
use url::{Url, ParseError};

use r2d2_postgres::PostgresConnectionManager;
use postgres::Connection;
use std::convert::TryFrom;

mod pg;
mod error;

pub struct Pool<'a>(BTreeMap<&'a str, Conn>);
enum Conn{
   PoolPg(r2d2::Pool<PostgresConnectionManager>),
}


impl<'a> Pool<'a> {

    fn new() -> Self {
        Pool(BTreeMap::new())
    }

    /// ensure that a connection pool for this db_url exist
    fn ensure(&mut self, db_url: &'a str) -> Result<(), ConnectError> {
        let platform:Result<Platform, ParseError> = TryFrom::try_from(db_url);
        match platform{
            Ok(platform) => {
                match platform {
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
            Err(e) => Err(ConnectError::DbUrlParseError(e))
        }
    }

    /// connect to existing db pool, or to a new pool if it hasn't been created yet
    fn connect(&mut self, db_url: &'a str) -> Result<&Conn, ConnectError> {
        self.ensure(db_url)?;
        let platform:Result<Platform, ParseError> = TryFrom::try_from(db_url);
        match platform{
            Ok(platform) => {
                match platform {
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
            Err(e) => Err(ConnectError::DbUrlParseError(e))
        }
    }

}

enum Platform{
    Postgres,
    Unsupported(String),
}

impl<'a> TryFrom<&'a str> for Platform{
    
    type Error = ParseError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let url = Url::parse(s);
        match url{
            Ok(url) => {
                let scheme = url.scheme();
                match scheme{
                    "postgres" => Ok(Platform::Postgres),
                    _ => Ok(Platform::Unsupported(scheme.to_string()))
                }
            },
            Err(e) => Err(e)
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect1() {
        let db_url = "postgres://postgres:p0stgr3s@localhost:5432/rforum";
        let mut pool = Pool::new();
        pool.ensure(db_url);
        assert!(pool.connect(db_url).is_ok());
    }

    #[test]
    fn connect_no_ensure(){
        let db_url = "postgres://postgres:p0stgr3s@localhost:5432/rforum";
        let mut pool = Pool::new();
        assert!(pool.connect(db_url).is_ok());
    }
}
