use url;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ConnectError{
    OutOfConnection,
    NoSuchPoolConnection,
    ParseError(ParseError),
    UnsupportedDb(String),
}

/// TODO: use error_chain i guess?
impl Error for ConnectError{
   fn description(&self) -> &str{
       "short desc"
   }
   fn cause(&self) -> Option<&Error> {
       None
   }
}

impl fmt::Display for ConnectError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

#[derive(Debug)]
pub enum ParseError{
    DbUrlParseError(url::ParseError),
}
