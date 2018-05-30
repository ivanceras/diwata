#![deny(warnings)]
extern crate diwata_server as server;
extern crate structopt;
use server::ServiceError;

fn main() -> Result<(),ServiceError> {
    server::start()
}
