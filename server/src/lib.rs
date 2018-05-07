#![deny(warnings)]
#![feature(plugin)]
#![feature(rustc_private)]
#![feature(integer_atomics)]

extern crate diwata_intel as intel;
#[macro_use]
extern crate lazy_static;
extern crate rustorm;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate structopt_derive;
extern crate futures;
extern crate hyper;
extern crate structopt;
#[macro_use]
extern crate log;

pub use error::ServiceError;
pub use handler::Server;
use intel::cache;
use rustorm::EntityManager;
use rustorm::Pool;
use rustorm::RecordManager;
use std::sync::{Arc, Mutex};
use structopt::StructOpt;

pub mod context;
mod error;
pub mod handler;

pub static PAGE_SIZE: u32 = 40;

lazy_static! {
    pub static ref DB_URL: Mutex<Option<String>> = Mutex::new(None);
    pub static ref POOL: Arc<Mutex<Pool>> = { Arc::new(Mutex::new(Pool::new())) };
}

fn get_db_url_value() -> Result<Option<String>, ServiceError> {
    match DB_URL.lock() {
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

pub fn set_db_url(new_url: &str) -> Result<(), ServiceError> {
    match DB_URL.lock() {
        Ok(mut db_url) => {
            *db_url = Some(new_url.to_string());
            Ok(())
        }
        Err(e) => Err(ServiceError::GenericError(format!("{}", e))),
    }
}

pub fn get_pool_em() -> Result<EntityManager, ServiceError> {
    let mut pool = match POOL.lock() {
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
    let mut pool = match POOL.lock() {
        Ok(pool) => pool,
        Err(_e) => return Err(ServiceError::PoolResourceError),
    };
    let db_url = &get_db_url()?;
    match pool.dm(db_url) {
        Ok(em) => Ok(em),
        Err(e) => return Err(ServiceError::DbError(e)),
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "diwata", about = "A user friendly database interface")]
pub struct Opt {
    #[structopt(
        short = "u",
        long = "db-url",
        help = "Database url to connect to, when set all data is exposed without login needed in the client side"
    )]
    pub db_url: Option<String>,
    #[structopt(
        short = "a",
        long = "address",
        help = "The address the server would listen, default is 0.0.0.0",
        default_value = "0.0.0.0"
    )]
    pub address: String,
    #[structopt(
        short = "p",
        long = "port",
        help = "What port this server would listen to, default is 8000",
        default_value = "8000"
    )]
    pub port: u16,
}

pub fn start() {
    let opt = Opt::from_args();
    println!("opt: {:?}", opt);
    if let Some(db_url) = opt.db_url {
        match set_db_url(&db_url) {
            Ok(_) => println!("url is set"),
            Err(_) => println!("unable to set db_url"),
        }
    }
    handler::run(&opt.address, opt.port);
}
