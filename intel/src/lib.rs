#![deny(warnings)]
#![allow(dead_code)]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rustorm_codegen;
extern crate rustorm;
extern crate dao;
#[macro_use]
extern crate serde_derive;
extern crate uuid;
extern crate bigdecimal;

mod reference;
mod widget;

pub mod window;
pub mod tab;
mod field;
pub mod table_intel;
mod service;
pub mod data_service;
pub mod data_container;
pub mod cache;
pub mod error;

pub use window::Window;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
