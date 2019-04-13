use crate::error::ServiceError;
use diwata_intel::cache;
use lazy_static::lazy_static;
use rustorm::pool;
use rustorm::DaoManager;
use rustorm::EntityManager;
use rustorm::Pool;
use std::sync::{Arc, RwLock};
use url::Url;

pub static PAGE_SIZE: u32 = 40;

lazy_static! {
    // the configured db url upon instantiaing the app
    pub static ref DB_URL: RwLock<Option<String>> = RwLock::new(None);
    // the session db url when a user is set, this is used when retrieving data from records
    pub static ref SESSION_DB_URL: RwLock<Option<String>> = RwLock::new(None);
    // the role of the currect session user, used in retrieving cached windows and table definition
    pub static ref ROLE_DB_URL: RwLock<Option<String>> = RwLock::new(None);
    // all the connection pool, key is the db_url
    pub static ref POOL: Arc<RwLock<Pool>> = { Arc::new(RwLock::new(Pool::new())) };
    pub static ref LOGIN_REQUIRED: RwLock<bool> = RwLock::new(false);
}

fn get_db_url_value() -> Result<Option<String>, ServiceError> {
    match DB_URL.read() {
        Ok(db_url) => {
            if let Some(ref db_url) = *db_url {
                Ok(Some(db_url.to_owned()))
            } else {
                Ok(None)
            }
        }
        Err(e) => Err(ServiceError::GenericError(format!("{}", e))),
    }
}

fn get_role_db_url_value() -> Result<Option<String>, ServiceError> {
    match ROLE_DB_URL.read() {
        Ok(db_url) => {
            if let Some(ref db_url) = *db_url {
                Ok(Some(db_url.to_owned()))
            } else {
                Ok(None)
            }
        }
        Err(e) => Err(ServiceError::GenericError(format!("{}", e))),
    }
}

fn get_session_db_url_value() -> Result<Option<String>, ServiceError> {
    match SESSION_DB_URL.read() {
        Ok(db_url) => {
            if let Some(ref db_url) = *db_url {
                Ok(Some(db_url.to_owned()))
            } else {
                Ok(None)
            }
        }
        Err(e) => Err(ServiceError::GenericError(format!("{}", e))),
    }
}

pub fn get_db_url() -> Result<String, ServiceError> {
    match get_db_url_value() {
        Ok(db_url) => {
            if let Some(ref db_url) = db_url {
                Ok(db_url.to_owned())
            } else {
                Err(ServiceError::NoDbUrlSpecified)
            }
        }
        Err(e) => Err(e),
    }
}

pub fn get_role_db_url() -> Result<String, ServiceError> {
    match get_role_db_url_value() {
        Ok(db_url) => {
            if let Some(ref db_url) = db_url {
                Ok(db_url.to_owned())
            } else {
                Err(ServiceError::NoDbUrlSpecified)
            }
        }
        Err(e) => Err(e),
    }
}

pub fn get_session_db_url() -> Result<String, ServiceError> {
    match get_session_db_url_value() {
        Ok(db_url) => {
            if let Some(ref db_url) = db_url {
                Ok(db_url.to_owned())
            } else {
                Err(ServiceError::NoDbUrlSpecified)
            }
        }
        Err(e) => Err(e),
    }
}

/// precache the processing of tables, and window in advance
pub fn precache() -> Result<(), ServiceError> {
    match cache::CACHE_POOL.lock() {
        Ok(mut cache_pool) => {
            let em = get_pool_em()?;
            let db_url = get_db_url()?;
            cache_pool.precache(&em, &db_url)?;
            Ok(())
        }
        Err(e) => Err(ServiceError::GenericError(format!("{}", e))),
    }
}

pub fn set_db_url(new_url: &str) -> Result<(), ServiceError> {
    match DB_URL.write() {
        Ok(mut db_url) => {
            println!("setting db_url to: {}", new_url);
            *db_url = Some(new_url.to_string());
            Ok(())
        }
        Err(e) => Err(ServiceError::GenericError(format!("{}", e))),
    }
}

pub fn set_role_db_url(new_url: &str) -> Result<(), ServiceError> {
    match ROLE_DB_URL.write() {
        Ok(mut db_url) => {
            *db_url = Some(new_url.to_string());
            Ok(())
        }
        Err(e) => Err(ServiceError::GenericError(format!("{}", e))),
    }
}

pub fn set_session_db_url(new_url: &str) -> Result<(), ServiceError> {
    match SESSION_DB_URL.write() {
        Ok(mut db_url) => {
            *db_url = Some(new_url.to_string());
            Ok(())
        }
        Err(e) => Err(ServiceError::GenericError(format!("{}", e))),
    }
}

/// sets the session user to the database, this adds restrictions to RLS and security imposed to
/// the database user
/// the user and password combination is tested first if it can connect to the database
/// then the user is set as the session user
pub fn test_credentials(user: &str, pwd: &str) -> Result<(), ServiceError> {
    let db_url = &get_db_url()?;
    let cons_url = recreate_db_url(user, Some(pwd), db_url)?;
    pool::test_connection(&cons_url)?;
    Ok(())
}

/// recreate the db_url substituting the username and password
pub fn recreate_db_url(
    user: &str,
    pwd: Option<&str>,
    db_url: &str,
) -> Result<String, ServiceError> {
    let url = Url::parse(db_url);
    match url {
        Ok(url) => {
            let scheme = url.scheme();
            let host = url.host_str().unwrap();
            let port = url.port();
            let path = url.path();
            let port_str = if let Some(port) = port {
                format!(":{}", port)
            } else {
                "".to_string()
            };
            let pwd_str = if let Some(pwd) = pwd {
                format!(":{}", pwd)
            } else {
                "".to_string()
            };
            // reconstructed db_url
            // postgres://user:pwd@host:port/db
            let cons_url = format!(
                "{}://{}{}@{}{}{}",
                scheme, user, pwd_str, host, port_str, path
            );
            Ok(cons_url)
        }
        Err(_e) => Err(ServiceError::GenericError("Error parsing db_url".into())),
    }
}

pub fn get_pool_em() -> Result<EntityManager, ServiceError> {
    let mut pool = match POOL.write() {
        Ok(pool) => pool,
        Err(_e) => return Err(ServiceError::PoolResourceError),
    };
    let db_url = &get_db_url()?;
    match pool.em(db_url) {
        Ok(em) => Ok(em),
        Err(e) => Err(ServiceError::DbError(e)),
    }
}

pub fn get_pool_session_em() -> Result<EntityManager, ServiceError> {
    let mut pool = match POOL.write() {
        Ok(pool) => pool,
        Err(_e) => return Err(ServiceError::PoolResourceError),
    };
    let db_url = &get_session_db_url()?;
    match pool.em(db_url) {
        Ok(em) => Ok(em),
        Err(e) => Err(ServiceError::DbError(e)),
    }
}

pub fn get_pool_dm() -> Result<DaoManager, ServiceError> {
    let mut pool = match POOL.write() {
        Ok(pool) => pool,
        Err(_e) => return Err(ServiceError::PoolResourceError),
    };
    let db_url = &get_db_url()?;
    match pool.dm(db_url) {
        Ok(em) => Ok(em),
        Err(e) => Err(ServiceError::DbError(e)),
    }
}

pub fn get_pool_session_dm() -> Result<DaoManager, ServiceError> {
    let mut pool = match POOL.write() {
        Ok(pool) => pool,
        Err(_e) => return Err(ServiceError::PoolResourceError),
    };
    let db_url = &get_session_db_url()?;
    match pool.dm(db_url) {
        Ok(em) => Ok(em),
        Err(e) => Err(ServiceError::DbError(e)),
    }
}

pub fn is_login_required() -> Result<bool, ServiceError> {
    match LOGIN_REQUIRED.read() {
        Ok(required) => Ok(*required),
        Err(e) => Err(ServiceError::GenericError(format!("{}", e))),
    }
}

pub fn set_login_required(required: bool) -> Result<(), ServiceError> {
    match LOGIN_REQUIRED.write() {
        Ok(mut login) => {
            *login = required;
            Ok(())
        }
        Err(e) => Err(ServiceError::GenericError(format!("{}", e))),
    }
}
