use rustorm::EntityManager;

fn get_users(em: &EntityManager) {
    let users = em.get_users();
    println!("users: {:#?}", users);
}
