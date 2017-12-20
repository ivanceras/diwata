#![deny(warnings)]
extern crate diwata;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "diwata", about = "A user friendly database interface")] 
struct Opt{
    #[structopt(short = "db", long = "dburl", help = "Database url to connect to")]
    db_url: Option<String>,
}

fn main() {
    let opt = Opt::from_args();
    println!("opt: {:?}", opt);
    diwata::rocket().launch();
}
