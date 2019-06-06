use crate::error::ServiceError;
use actix_web::HttpRequest;
use std::convert::TryFrom;

pub struct Credentials {
    pub username: String,
    pub password: String,
}

impl<'a> TryFrom<&'a HttpRequest> for Credentials {
    type Error = ServiceError;

    fn try_from(req: &'a HttpRequest) -> Result<Credentials, Self::Error> {
        let headers = req.headers();
        let username = headers.get("username");
        let password = headers.get("password");
        if let Some(username) = username {
            if let Some(password) = password {
                return Ok(Credentials {
                    username: username.to_str().unwrap().to_string(),
                    password: password.to_str().unwrap().to_string(),
                });
            }
        }
        Err(ServiceError::NotFound)
    }
}
