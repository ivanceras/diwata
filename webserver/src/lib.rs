
#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;
extern crate rustorm;
extern crate intel;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
use rocket::Rocket;
use rustorm::Pool;
use rocket_contrib::Json;
use intel::Window;
use intel::data_service;
use intel::window::{self,GroupedWindow};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use std::sync::{Arc,Mutex};
use rustorm::TableName;
use rocket::fairing::AdHoc;
use rocket::http::hyper::header::AccessControlAllowOrigin;
use std::collections::BTreeMap;
use rustorm::Rows;
use rustorm::error::DbError;
use rustorm::EntityManager;
use error::ServiceError;
use intel::cache;
use intel::data_service::RecordDetail;
use rustorm::RecordManager;

mod error;

static DB_URL: &'static str = "postgres://postgres:p0stgr3s@localhost:5432/bazaar_v8";

lazy_static!{
    pub static ref POOL: Arc<Mutex<Pool>> = {
        Arc::new(Mutex::new(Pool::new()))
    };
}

#[get("/")]
fn index() -> String {
    "Hello".into()
}

fn get_pool_em() -> Result<EntityManager, ServiceError> {
    let mut pool = match POOL.lock(){
        Ok(pool) => pool,
        Err(e) => return Err(ServiceError::PoolResourceError)
    };
    match pool.em(DB_URL){
       Ok(em) => Ok(em),
       Err(e) => return Err(ServiceError::DbError(e))
    }
}

fn get_pool_dm() -> Result<RecordManager, ServiceError> {
    let mut pool = match POOL.lock(){
        Ok(pool) => pool,
        Err(e) => return Err(ServiceError::PoolResourceError)
    };
    match pool.dm(DB_URL){
       Ok(em) => Ok(em),
       Err(e) => return Err(ServiceError::DbError(e))
    }
}


#[get("/windows")]
fn get_windows() -> Result<Json<Vec<GroupedWindow>>, ServiceError> {
    let em = get_pool_em()?;
    let grouped_windows: Vec<GroupedWindow> 
        = window::get_grouped_windows_using_cache(&em, DB_URL)?;
    Ok(Json(grouped_windows))
}


#[get("/window/<table_name>")]
fn get_window(table_name: String) -> Result<Option<Json<Window>>, ServiceError> {
    let em = get_pool_em()?;
    let mut cache_pool = cache::CACHE_POOL.lock().unwrap();
    let windows = cache_pool.get_cached_windows(&em, DB_URL)?;
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &windows);
    match window{
        Some(window) => Ok(Some(Json(window.to_owned()))),
        None => Ok(None)
    }
}

#[get("/window/<table_name>/data")]
fn get_data(table_name: String) -> Result<Option<Json<Rows>>, ServiceError> {
    let em = get_pool_em()?;
    let mut cache_pool = cache::CACHE_POOL.lock().unwrap();
    let windows = cache_pool.get_cached_windows(&em, DB_URL)?;
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &windows);
    let tables = cache_pool.get_cached_tables(&em, DB_URL)?;
    match window{
        Some(window) => {
            let rows: Rows = 
                data_service::get_maintable_data_first_page(&em, &tables,
                                                        &window, None, 20)?;
            Ok(Some(Json(rows)))
        }
        None => Ok(None)
    }
}

#[get("/window/<table_name>/data/select/<record_id>")]
fn get_detailed_record(table_name: String, record_id: String) -> Result<Option<Json<RecordDetail>>, ServiceError> {
    let dm = get_pool_dm()?;
    let em = get_pool_em()?;
    let mut cache_pool = cache::CACHE_POOL.lock().unwrap();
    let windows = cache_pool.get_cached_windows(&em, DB_URL)?;
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &windows);
    let tables = cache_pool.get_cached_tables(&em, DB_URL)?;
    match window{
        Some(window) => {
            let dao: Option<RecordDetail> = 
                data_service::get_selected_record_detail(&dm, &tables,
                                                        &window, &record_id)?;
            match dao{
                Some(dao) => Ok(Some(Json(dao))),
                None => Ok(None)
            }
        }
        None => Ok(None)
    }
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
                    get_detailed_record,
                 ]
        ) 
}
