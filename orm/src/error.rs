use url;

pub enum ConnectError{
    OutOfConnection,
    NoSuchPoolConnection,
    DbUrlParseError(url::ParseError),
    UnsupportedDb(String),
}

enum ParseError{
}
