
#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;
extern crate rustorm;
extern crate intel;
#[macro_use]
extern crate lazy_static;
use rocket::Rocket;
use rustorm::Pool;
use rocket_contrib::Json;
use intel::Window;
use intel::data_service;
use intel::window::{self,GroupedWindow};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use std::sync::Mutex;
use std::sync::Arc;
use rustorm::TableName;
use rocket::fairing::AdHoc;
use rocket::http::hyper::header::AccessControlAllowOrigin;
use std::collections::BTreeMap;
use rustorm::Rows;
use rustorm::error::DbError;

static DB_URL: &'static str = "postgres://postgres:p0stgr3s@localhost:5432/bazaar_v8";

#[get("/")]
fn index() -> String {
    "Hello".into()
}

#[get("/windows")]
fn get_windows() -> Json<Vec<GroupedWindow>> {
    let mut pool = POOL.lock().unwrap();
    let em = pool.em(DB_URL).unwrap();
    let grouped_windows: Vec<GroupedWindow> = window::get_grouped_windows(&em).unwrap();
    Json(grouped_windows)
}


#[get("/window/<table_name>")]
fn get_window(table_name: String) -> Option<Json<Window>> {
    let mut pool = POOL.lock().unwrap();
    let em = pool.em(DB_URL).unwrap();
    let tables = em.get_all_tables().unwrap();
    let windows = window::derive_all_windows(&tables);
    println!("all windows: {:#?}", windows);
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &windows);
    println!("window: {:?}", window);
    match window{
        Some(window) => Some(Json(window.to_owned())),
        None => None
    }
}

#[get("/window/<table_name>/data")]
fn get_data(table_name: String) -> Option<Json<Rows>> {
    let mut pool = POOL.lock().unwrap();
    let em = pool.em(DB_URL).unwrap();
    let tables = em.get_all_tables().unwrap();
    let windows = window::derive_all_windows(&tables);
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &windows);
    match window{
        Some(window) => {
            let res:Result<Rows,DbError> = data_service
                        ::get_maintable_data_first_page(&em, &tables, 
                                                        &window, None, 20);
            match res{
                Ok(rows) => Some(Json(rows)),
                Err(e) => None
            }
        }
        None => None
    }
}

lazy_static!{
    pub static ref POOL: Arc<Mutex<Pool>> = {
        Arc::new(Mutex::new(Pool::new()))
    };
}


pub fn rocket() -> Rocket {
    rocket::ignite()
        .attach(AdHoc::on_response(|req, resp| {
            resp.set_header(AccessControlAllowOrigin::Any);
        }))
        .mount(
            "/", routes![
                    index,
                    get_windows,
                    get_window,
                    get_data,
                 ]
        ) 
}
