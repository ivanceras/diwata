use crate::Window;
use rustorm::{
    DaoManager,
    EntityManager,
    Table,
};

pub struct Context {
    pub em: EntityManager,
    pub dm: DaoManager,
    pub tables: Vec<Table>,
    pub windows: Vec<Window>,
}
