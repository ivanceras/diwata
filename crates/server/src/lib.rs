//#![deny(warnings)]
#![feature(plugin)]
#![feature(rustc_private)]
#![feature(integer_atomics)]
#![feature(try_from)]

#[macro_use]
extern crate structopt_derive;
#[macro_use]
extern crate include_dir;

pub use error::ServiceError;
pub use global::set_db_url;
pub use global::set_login_required;
pub use handler::Server;

pub mod context;
mod credentials;
pub mod error;
mod global;
pub mod handler;

pub fn start() -> Result<(), ServiceError> {
    handler::run()?;
    Ok(())
}
