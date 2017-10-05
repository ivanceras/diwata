#![feature(try_from)]
extern crate r2d2;
#[cfg(feature = "with-postgres")]
extern crate r2d2_postgres;
#[cfg(feature = "with-postgres")]
extern crate postgres;
extern crate url;

mod pool;
#[cfg(feature = "with-postgres")]
mod pg;
mod platform;
mod error;



