#![deny(warnings)]
extern crate diwata;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "diwata", about = "A user friendly database interface")]
struct Opt {
    #[structopt(short = "u", long = "db-url", help = "Database url to connect to")]
    db_url: String,
    #[structopt(short = "a", long = "address",
                help = "The address the server would listen, default is 0.0.0.0")]
    address: Option<String>,
    #[structopt(short = "p", long = "port",
                help = "What port this server would listen to, default is 8000")]
    port: Option<u16>,
}

fn main() {
    let opt = Opt::from_args();
    println!("opt: {:?}", opt);
    match diwata::set_db_url(opt.db_url) {
        Ok(_) => println!("url is set"),
        Err(_) => println!("unable to set db_url"),
    }
    match diwata::rocket(opt.address, opt.port) {
        Ok(server) => {
            println!("Launching..");
            server.launch();
        }
        Err(e) => panic!("unable to initialize server: {}", e),
    }
}
