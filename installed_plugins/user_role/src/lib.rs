extern crate rustorm;
extern crate diwata_server;
extern crate hyper;

use hyper::Request;
use rustorm::EntityManager;

fn get_users(em: &EntityManager) {
    let users = em.get_users();
    println!("users: {:#?}", users);
}


pub fn handle_request(req: Request) {
}

