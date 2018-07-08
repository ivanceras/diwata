use hyper::header::{self, Header, Raw};
use std::fmt;
use hyper;

#[derive(Clone,Debug)]
pub struct Username(pub String);

#[derive(Clone, Debug)]
pub struct Password(pub String);

impl Header for Username{
    fn header_name() -> &'static  str{
        "username"
    }

    fn parse_header(raw: &Raw) -> hyper::Result<Username> {
        println!("raw: {:?}", raw);
        if let Some(line) = raw.one(){
            println!("line:{:?}", line);
            let username = String::from_utf8(line.to_owned());
            println!("username: {:?}", username);
            match username{
                Ok(username) => Ok(Username(username)),
                Err(_) => Err(hyper::Error::Header)
            }
        }
        else{
            Err(hyper::Error::Header)
        }
    }

    fn fmt_header(&self, f: &mut header::Formatter) -> fmt::Result {
        f.fmt_line(&self.0)
    }
}

impl Header for Password{
    fn header_name() -> &'static  str{
        "password"
    }

    fn parse_header(raw: &Raw) -> hyper::Result<Password> {
        if let Some(line) = raw.one(){
            let password = String::from_utf8(line.to_owned());
            match password{
                Ok(password) => Ok(Password(password)),
                Err(_) => Err(hyper::Error::Header)
            }
        }
        else{
            Err(hyper::Error::Header)
        }
    }

    fn fmt_header(&self, f: &mut header::Formatter) -> fmt::Result {
        f.fmt_line(&self.0)
    }
}
