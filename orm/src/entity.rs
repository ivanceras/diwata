use error::DbError;
use dao::{FromDao, ToDao};
use database::Database;

pub struct Entity<'a>(&'a Database);


impl<'a> Entity<'a> {
    /*
    fn get_all<T,D>(&self, T) -> Result<Vec<D>, DbError>
        where
            T: FromTable,
            D: FromDao
    {
        panic!();
    }
    */

    #[allow(unused)]
    fn insert<T, F>(&self, _daos: &[T]) -> Result<Vec<F>, DbError>
    where
        T: ToDao,
        F: FromDao,
    {
        panic!();
    }
}
