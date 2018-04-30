#![deny(warnings)]
extern crate diwata_server as server;
extern crate structopt;

fn main() {
    server::hyper_server::run();
    server::start();
}
