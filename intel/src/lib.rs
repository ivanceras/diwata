//#![deny(warnings)]
#![allow(dead_code)]
extern crate rustorm;
mod reference;
mod widget;

mod window;
mod tab;
mod field;
mod table_intel;
mod service;
mod data_service;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
