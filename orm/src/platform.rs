
use url::{Url};
use std::convert::TryFrom;
use error::ParseError;
cfg_if! {if #[cfg(feature = "with-postgres")]{
    use postgres::Connection;
    use r2d2_postgres;
    use pg::PostgresIns;
}}
use r2d2;


pub enum DbPlatform{
    #[cfg(feature = "with-postgres")]
    Postgres(PostgresIns)
}

pub(crate) enum Platform{
    #[cfg(feature = "with-postgres")]
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
                    #[cfg(feature = "with-postgres")]
                    "postgres" => Ok(Platform::Postgres),
                    _ => Ok(Platform::Unsupported(scheme.to_string()))
                }
            },
            Err(e) => Err(ParseError::DbUrlParseError(e))
        }
    }
}


