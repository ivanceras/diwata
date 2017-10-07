use url;
use std::error::Error;
use std::fmt;
use r2d2;
cfg_if! {if #[cfg(feature = "with-postgres")]{
    use pg::PostgresError;
}}

#[derive(Debug)]
pub enum ConnectError {
    NoSuchPoolConnection,
    ParseError(ParseError),
    UnsupportedDb(String),
    Timeout(r2d2::GetTimeout),
}

/// TODO: use error_chain i guess?
impl Error for ConnectError {
    fn description(&self) -> &str {
        "short desc"
    }
    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for ConnectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

#[derive(Debug)]
pub enum ParseError {
    DbUrlParseError(url::ParseError),
}


#[derive(Debug)]
pub enum PlatformError {
    #[cfg(feature = "with-postgres")] PostgresError(PostgresError),
}

#[derive(Debug)]
pub enum DbError {
    PlatformError(PlatformError),
    ConvertError(ConvertError),
}

#[derive(Debug)]
pub enum ConvertError {
    UnknownDataType,
    UnsupportedDataType(String),
}
