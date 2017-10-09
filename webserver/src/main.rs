extern crate rustorm;
#[macro_use]
extern crate dao_codegen;
extern crate dao;
extern crate chrono;

use rustorm::Pool;
use rustorm::Database;
use rustorm::Rows;
use rustorm::Dao;
use rustorm::DbError;

use chrono::DateTime;
use chrono::offset::Utc;
use rustorm::{ToTableName, ToColumnNames, FromDao, ToDao};


#[derive(Debug, ToTableName, ToColumnNames, FromDao, ToDao)]
struct Film{
    film_id: i32,
    title: String,
    description: Option<String>,
    release_year: Option<i32>,
    language_id: i16,
    original_language_id: Option<i16>,
    rental_duration: i16,
    rental_rate: f64,//TODO: HACKY: column is casted into f64 since support for numeric is not yet finalized
    length: Option<i16>,
    replacement_cost: f64, //TODO: numeric hack
    //rating: Option<String>,
    last_update: DateTime<Utc>,
    special_features:Vec<String>,
}

fn main() {
    let mut pool = Pool::new();
    let db_url = "postgres://postgres:p0stgr3s@localhost/sakila";
    let em = pool.em(db_url).unwrap();
    
    //let films: Result<Vec<Film>, DbError> = em.get_all();
    let films: Result<Vec<Film>, DbError> = em.execute_sql_with_return(
        "SELECT film_id, title, description,
        release_year, language_id,
        original_language_id,
        rental_duration,
        rental_rate::FLOAT8,
        length,
        replacement_cost::FLOAT8,
        --rating,
        last_update,
        special_features
        FROM film", &[]);
    println!("films: {:#?}", films);
    if let Ok(films) = films{
        for film in films{
            println!("{:#?}", film);
        }
    }
}
