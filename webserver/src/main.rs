extern crate orm;
use orm::Pool;
use orm::Database;
use orm::Rows;
use orm::Dao;
use orm::DbError;

fn main() {
    let mut pool = Pool::new();
    let db_url = "postgres://postgres:p0stgr3s@localhost/rforum";
    let db  = pool.db(db_url).unwrap();
    let rows:Result<Rows, DbError> = (&db).execute_sql_select("select now(), 'Hello world'::TEXT,  '9a7a36d0-1010-11e5-a475-33082a4698d6'::UUID", &[]);
    println!("columns: {:#?}", rows);
    if let Ok(rows) = rows {
        for row in rows.iter(){
            println!("row {:?}", row);
        }
    }

    let articles: Result<Rows, DbError> = (&db).execute_sql_select("select * from article", &[]);
    println!("articles: {:#?}", articles);
}

