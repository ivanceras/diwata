extern crate dao;
#[macro_use]
extern crate dao_codegen;

use dao::{FromDao, ToDao};
use dao::ToTable;


#[derive(Debug, FromDao, ToDao, ToTable)]
struct User {
    id: i32,
    username: String,
}

fn main() {
    let user = User {
        id: 1,
        username: "ivanceras".to_string(),
    };
    println!("user: {:#?}", user);
    let dao = user.to_dao();
    println!("dao: {:#?}", dao);
    let table = User::to_table();
    println!("table name: {}", table.name);
    println!("table: {:#?}", table);
}
