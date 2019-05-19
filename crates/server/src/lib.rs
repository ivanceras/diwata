//#![deny(warnings)]
#![deny(clippy::all)]

#[macro_use]
extern crate structopt_derive;

pub use error::ServiceError;
pub use global::{
    set_db_url,
    set_login_required,
};
pub use handler::Server;

mod credentials;
pub mod error;
mod global;
pub mod handler;
pub mod session;

pub fn start() -> Result<(), ServiceError> {
    handler::run()?;
    Ok(())
}
