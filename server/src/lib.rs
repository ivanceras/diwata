//#![deny(warnings)]
#![feature(plugin)]
#![feature(rustc_private)]
#![feature(integer_atomics)]
#![feature(try_from)]

extern crate diwata_intel as intel;
#[macro_use]
extern crate lazy_static;
extern crate rustorm;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate structopt_derive;
extern crate futures;
extern crate hyper;
extern crate structopt;
#[macro_use]
extern crate log;
extern crate url;

use structopt::StructOpt;


pub use error::ServiceError;
pub use global::set_db_url;
pub use global::set_login_required;
pub use handler::Server;

pub mod context;
pub mod error;
pub mod handler;
mod global;
mod credentials;


#[derive(StructOpt, Debug)]
#[structopt(name = "diwata", about = "A user friendly database interface")]
pub struct Opt {
    #[structopt(
        short = "u",
        long = "db-url",
        help = "Database url to connect to, when set all data is exposed without login needed in the client side"
    )]
    pub db_url: Option<String>,

    #[structopt(
        short = "a",
        long = "address",
        help = "The address the server would listen, default is 0.0.0.0",
        default_value = "0.0.0.0"
    )]
    pub address: String,

    #[structopt(
        short = "p",
        long = "port",
        help = "What port this server would listen to, default is 8000",
        default_value = "8000"
    )]
    pub port: u16,
    #[structopt(
        short = "c",
        long = "cache",
        help = "precache the tables and windows so the first web request loads instantly, this requires the db-url to be set and login_required disabled, in order to work",
    )]
    pub precache: bool,

    #[structopt(
        short = "l",
        long = "login-required",
        help = "If enabled, then the user must supply username and password in all of the API calls",
    )]
    pub login_required: bool,
}

pub fn start()-> Result<(),ServiceError> {
    let opt = Opt::from_args();
    println!("opt: {:?}", opt);
    if let Some(db_url) = opt.db_url {
        global::set_db_url(&db_url)?;
        println!("url is set");
        if opt.precache && !opt.login_required{
            println!("precaching..");
            global::precache()?;
            println!("precaching complete!");
        }
    }
    global::set_login_required(opt.login_required)?;
    handler::run(&opt.address, opt.port)?;
    println!("server ready...");
    Ok(())
}
