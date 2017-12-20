#![deny(warnings)]
#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;
extern crate rustorm;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

mod reference;
mod widget;

pub mod window;
pub mod tab;
mod field;
mod table_intel;
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
