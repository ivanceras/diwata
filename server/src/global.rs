
use url::Url;
pub use error::ServiceError;
pub use handler::Server;
use intel::cache;
use rustorm::EntityManager;
use rustorm::Pool;
use rustorm::pool;
use rustorm::RecordManager;
use std::sync::{Arc, RwLock};

pub static PAGE_SIZE: u32 = 40;

lazy_static! {
    pub static ref DB_URL: RwLock<Option<String>> = RwLock::new(None);
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

/// precache the processing of tables, and window in advance
pub fn precache()-> Result<(), ServiceError> {
    match cache::CACHE_POOL.lock(){
        Ok(mut cache_pool) => {
            let em = get_pool_em()?;
            let db_url = get_db_url()?;
            cache_pool.precache(&em, &db_url)?;
            Ok(())
        }
        Err(e) => Err(ServiceError::GenericError(format!("{}", e)))
    }
}

pub fn set_db_url(new_url: &str) -> Result<(), ServiceError> {
    match DB_URL.write() {
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
pub fn set_user(user: &str, pwd: &str) -> Result<(), ServiceError> {
    let db_url = &get_db_url()?;
    let url = Url::parse(db_url);
    match url{
        Ok(url) => {
            let scheme = url.scheme();
            let host = url.host_str().unwrap();
            let port = url.port();
            let path = url.path();
            let port_str = if let Some(port) = port {
                format!(":{}", port)
            }else{
                "".to_string()
            };
            // reconstructed db_url
            // postgres://user:pwd@host:port/db
            let cons_url = format!("{}://{}:{}@{}{}{}",scheme, user, pwd, host, port_str, path);
            pool::test_connection(&cons_url)?;
            Ok(())
        }
        Err(_e) => {
            Err(ServiceError::GenericError("Error parsing db_url".into()))
        }
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
        Err(e) => return Err(ServiceError::DbError(e)),
    }
}

pub fn get_pool_dm() -> Result<RecordManager, ServiceError> {
    let mut pool = match POOL.write() {
        Ok(pool) => pool,
        Err(_e) => return Err(ServiceError::PoolResourceError),
    };
    let db_url = &get_db_url()?;
    match pool.dm(db_url) {
        Ok(em) => Ok(em),
        Err(e) => return Err(ServiceError::DbError(e)),
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
