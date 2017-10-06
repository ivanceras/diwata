extern crate dao;

use dao::{FromDao, ToDao};


struct User {
    id: i32,
    username: String,
}

impl FromDao for User {
    fn from_dao(dao: &dao::Dao) -> Self {
        User {
            id: dao.get("id").unwrap(),
            username: dao.get("username").unwrap(),
        }
    }
}
impl ToDao for User {
    fn to_dao(&self) -> dao::Dao {
        let mut dao = dao::Dao::new();
        dao.insert("id", &self.id);
        dao.insert("username", &self.username);
        dao
    }
}
fn main() {
    let user = User {
        id: 1,
        username: "ivanceras".to_string(),
    };
    let dao = user.to_dao();
    println!("dao: {:#?}", dao);
}
