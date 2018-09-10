//#![deny(warnings)]
#![feature(plugin)]
#![feature(rustc_private)]
#![feature(integer_atomics)]
#![feature(try_from)]

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
extern crate url;
#[macro_use]
extern crate cfg_if;
#[macro_use]
extern crate include_dir;

pub use error::ServiceError;
pub use global::set_db_url;
pub use global::set_login_required;
pub use handler::Server;

pub mod context;
pub mod error;
pub mod handler;
mod global;
mod credentials;



pub fn start()-> Result<(),ServiceError> {
    handler::run()?;
    Ok(())
}
