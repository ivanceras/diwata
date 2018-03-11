#![deny(warnings)]
#![allow(dead_code)]
extern crate bigdecimal;
#[macro_use]
extern crate lazy_static;
extern crate rustorm;
#[macro_use]
extern crate rustorm_codegen;
extern crate rustorm_dao as dao;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

mod reference;
mod widget;

pub mod window;
pub mod tab;
mod field;
pub mod table_intel;
pub mod data_read;
pub mod data_modify;
pub mod data_container;
pub mod cache;
pub mod error;
mod common;
mod query_builder;

pub use window::Window;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
