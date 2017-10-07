use error::DbError;
use dao::{FromDao, ToDao, ToTable};
use database::Database;

pub struct EntityManager<'a>(pub &'a Database);


impl<'a> EntityManager<'a> {

    /// get all the records of this table
    pub fn get_all<T>(&self) -> Result<Vec<T>, DbError>
        where
            T: ToTable + FromDao
    {
        let table = T::to_table();
        let sql = format!("SELECT * FROM {}",table.name());
        let rows = self.0.execute_sql_select(&sql, &[])?;
        let mut entities = vec![];
        for dao in rows.iter(){
           let entity = T::from_dao(&dao); 
           entities.push(entity)
        }
        Ok(entities)
    }

    /// insert to table the values of this struct
    pub fn insert<TD, FD>(&self, _daos: &[TD]) -> Result<Vec<FD>, DbError>
    where
        TD: ToDao,
        FD: FromDao,
    {
        panic!();
    }
}


#[cfg(test)]
mod test{
    extern crate dao;
    use super::*;
    use dao::{ToDao, FromDao, ToTable};
    use ::pool::Pool;

    #[test]
    fn use_em(){
        #[derive(Debug, FromDao, ToTable)]
        struct Users{
            id: i32,
            email: String,
        }
        let db_url = "postgres://postgres:p0stgr3s@localhost/rforum";
        let mut pool = Pool::new();
        let db  = pool.db(db_url).unwrap();
        let em = EntityManager(&*db);
        let users = em.get_all::<Users>();
    }
}
