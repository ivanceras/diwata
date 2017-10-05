#![deny(warnings)]
#![feature(try_from)]
#![feature(conservative_impl_trait)]
#[macro_use]
extern crate cfg_if;
extern crate r2d2;
extern crate url;
extern crate dao;
cfg_if! {if #[cfg(feature = "with-postgres")]{
    extern crate r2d2_postgres;
    extern crate postgres;
    #[macro_use]
    extern crate postgres_shared;
    mod pg;
}}
extern crate uuid;
extern crate chrono;

mod pool;
mod platform;
mod error;
mod database;


pub use pool::Pool;
pub use database::Database;
pub use dao::Dao;
pub use dao::Value;
pub use dao::Rows;
pub use error::DbError;
