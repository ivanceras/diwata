#![feature(try_from)]
#[macro_use]
extern crate cfg_if;
extern crate r2d2;
extern crate url;
extern crate dao;
cfg_if! {if #[cfg(feature = "with-postgres")]{
    extern crate r2d2_postgres;
    extern crate postgres;
    mod pg;
}}

mod pool;
mod platform;
mod error;
mod database;
