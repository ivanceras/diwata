//#![deny(warnings)]
#![allow(dead_code)]
extern crate rustorm;
mod reference;
mod widget;

mod window;
mod tab;
mod field;
mod table_intel;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
