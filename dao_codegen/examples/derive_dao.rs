extern crate dao;
#[macro_use]
extern crate dao_codegen;

use dao::{FromDao,ToDao};


#[derive(Debug, FromDao, ToDao)]
struct User{
    id: i32,
    username: String,
}

fn main(){
    let user = User{
        id: 1,
        username: "ivanceras".to_string(),
    };
    println!("user: {:#?}", user);
    let dao = user.to_dao();
}

