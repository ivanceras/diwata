#![deny(warnings)]
#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(match_default_bindings)]

extern crate intel;
#[macro_use]
extern crate lazy_static;
extern crate rocket;
extern crate rocket_contrib;
extern crate rustorm;
use rocket::Rocket;
use rustorm::Pool;
use rocket_contrib::Json;
use intel::Window;
use intel::data_service;
use intel::window::{self, GroupedWindow};
use std::sync::{Arc, Mutex};
use rustorm::TableName;
use rocket::fairing::AdHoc;
use rocket::http::hyper::header::AccessControlAllowOrigin;
use rustorm::Rows;
use rustorm::EntityManager;
use error::ServiceError;
use intel::cache;
use intel::data_service::RecordDetail;
use rustorm::RecordManager;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;
use rocket::response::Redirect;
use intel::tab::Tab;
use intel::data_container::Lookup;
use intel::table_intel;

mod error;

static DB_URL: &'static str = "postgres://postgres:p0stgr3s@localhost:5432/sakila";
//static DB_URL: &'static str = "postgres://postgres:p0stgr3s@localhost:5432/bazaar_v8";

static PAGE_SIZE: u32 = 40;

lazy_static!{
    pub static ref POOL: Arc<Mutex<Pool>> = {
        Arc::new(Mutex::new(Pool::new()))
    };
}

fn get_pool_em() -> Result<EntityManager, ServiceError> {
    let mut pool = match POOL.lock() {
        Ok(pool) => pool,
        Err(_e) => return Err(ServiceError::PoolResourceError),
    };
    match pool.em(DB_URL) {
        Ok(em) => Ok(em),
        Err(e) => return Err(ServiceError::DbError(e)),
    }
}

fn get_pool_dm() -> Result<RecordManager, ServiceError> {
    let mut pool = match POOL.lock() {
        Ok(pool) => pool,
        Err(_e) => return Err(ServiceError::PoolResourceError),
    };
    match pool.dm(DB_URL) {
        Ok(em) => Ok(em),
        Err(e) => return Err(ServiceError::DbError(e)),
    }
}

#[get("/")]
fn get_windows() -> Result<Json<Vec<GroupedWindow>>, ServiceError> {
    let em = get_pool_em()?;
    let grouped_windows: Vec<GroupedWindow> = window::get_grouped_windows_using_cache(&em, DB_URL)?;
    Ok(Json(grouped_windows))
}

#[get("/<table_name>")]
fn get_window(table_name: String) -> Result<Option<Json<Window>>, ServiceError> {
    let em = get_pool_em()?;
    let mut cache_pool = cache::CACHE_POOL.lock().unwrap();
    let windows = cache_pool.get_cached_windows(&em, DB_URL)?;
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &windows);
    match window {
        Some(window) => Ok(Some(Json(window.to_owned()))),
        None => Ok(None),
    }
}

#[get("/<table_name>")]
fn get_total_records(table_name: String) -> Result<Option<Json<u64>>, ServiceError> {
    let em = get_pool_em()?;
    let mut cache_pool = cache::CACHE_POOL.lock().unwrap();
    let table_name = TableName::from(&table_name);
    let tables = cache_pool.get_cached_tables(&em, DB_URL)?;
    let table = table_intel::get_table(&table_name, &tables); 
    match table{
        Some(table) => {
            let count = data_service::get_total_records(&em, &table.name)?;
            Ok(Some(Json(count)))
        }
        None => Ok(None)
    }
}

#[get("/<table_name>")]
fn get_data(table_name: String) -> Result<Option<Json<Rows>>, ServiceError> {
    get_data_with_page(table_name, 1)
}

#[get("/<table_name>/<page>")]
fn get_data_with_page(table_name: String, page: u32) -> Result<Option<Json<Rows>>, ServiceError> {
    let em = get_pool_em()?;
    let mut cache_pool = cache::CACHE_POOL.lock().unwrap();
    let windows = cache_pool.get_cached_windows(&em, DB_URL)?;
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &windows);
    let tables = cache_pool.get_cached_tables(&em, DB_URL)?;
    match window {
        Some(window) => {
            let rows: Rows =
                data_service::get_maintable_data(&em, &tables, &window, None, page, PAGE_SIZE)?;
            Ok(Some(Json(rows)))
        }
        None => Ok(None),
    }
}

#[get("/<table_name>/select/<record_id>")]
fn get_detailed_record(
    table_name: String,
    record_id: String,
) -> Result<Option<Json<RecordDetail>>, ServiceError> {
    let dm = get_pool_dm()?;
    let em = get_pool_em()?;
    let mut cache_pool = cache::CACHE_POOL.lock().unwrap();
    let windows = cache_pool.get_cached_windows(&em, DB_URL)?;
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &windows);
    let tables = cache_pool.get_cached_tables(&em, DB_URL)?;
    match window {
        Some(window) => {
            let dao: Option<RecordDetail> = data_service::get_selected_record_detail(
                &dm,
                &tables,
                &window,
                &record_id,
                PAGE_SIZE,
            )?;
            match dao {
                Some(dao) => Ok(Some(Json(dao))),
                None => Ok(None),
            }
        }
        None => Ok(None),
    }
}

#[get("/<table_name>")]
fn get_window_lookup_data(table_name: String) -> Result<Option<Json<Lookup>>, ServiceError> {
    let dm = get_pool_dm()?;
    let em = get_pool_em()?;
    let mut cache_pool = cache::CACHE_POOL.lock().unwrap();
    let windows = cache_pool.get_cached_windows(&em, DB_URL)?;
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &windows);
    let tables = cache_pool.get_cached_tables(&em, DB_URL)?;
    match window {
        Some(window) => {
            let lookup: Lookup =
                data_service::get_all_lookup_for_window(&dm, &tables, &window, PAGE_SIZE)?;
            Ok(Some(Json(lookup)))
        }
        None => Ok(None),
    }
}

#[get("/<table_name>/<page>")]
fn get_lookup_data(table_name: String, page: u32) -> Result<Option<Json<Rows>>, ServiceError> {
    let dm = get_pool_dm()?;
    let em = get_pool_em()?;
    let mut cache_pool = cache::CACHE_POOL.lock().unwrap();
    let windows = cache_pool.get_cached_windows(&em, DB_URL)?;
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &windows);
    let tables = cache_pool.get_cached_tables(&em, DB_URL)?;
    match window {
        Some(window) => {
            let rows: Rows = data_service::get_lookup_data_of_tab(
                &dm,
                &tables,
                &window.main_tab,
                PAGE_SIZE,
                page,
            )?;
            Ok(Some(Json(rows)))
        }
        None => Ok(None),
    }
}

/// retrieve records from a has_many table based on the selected main records
/// from the main table
#[get("/<table_name>/select/<record_id>/has_many/<has_many_table>/<page>")]
fn get_has_many_records(
    table_name: String,
    record_id: String,
    has_many_table: String,
    page: u32,
) -> Result<Option<Json<Rows>>, ServiceError> {
    let dm = get_pool_dm()?;
    let em = get_pool_em()?;
    let mut cache_pool = cache::CACHE_POOL.lock().unwrap();
    let windows = cache_pool.get_cached_windows(&em, DB_URL)?;
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &windows);
    let tables = cache_pool.get_cached_tables(&em, DB_URL)?;
    let has_many_table_name = TableName::from(&has_many_table);
    match window {
        Some(window) => {
            let main_table = data_service::get_main_table(window, &tables);
            assert!(main_table.is_some());
            let main_table = main_table.unwrap();
            let has_many_tab = data_service::find_tab(&window.has_many_tabs, &has_many_table_name);
            match has_many_tab {
                Some(has_many_tab) => {
                    let rows = data_service::get_has_many_records_service(
                        &dm,
                        &tables,
                        &main_table,
                        &record_id,
                        has_many_tab,
                        PAGE_SIZE,
                        page,
                    )?;
                    Ok(Some(Json(rows)))
                }
                None => Ok(None),
            }
        }
        None => Ok(None),
    }
}

/// retrieve records from a has_many table based on the selected main records
/// from the main table
#[get("/<table_name>/select/<record_id>/indirect/<indirect_table>/<page>")]
fn get_indirect_records(
    table_name: String,
    record_id: String,
    indirect_table: String,
    page: u32,
) -> Result<Option<Json<Rows>>, ServiceError> {
    let dm = get_pool_dm()?;
    let em = get_pool_em()?;
    let mut cache_pool = cache::CACHE_POOL.lock().unwrap();
    let windows = cache_pool.get_cached_windows(&em, DB_URL)?;
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &windows);
    let tables = cache_pool.get_cached_tables(&em, DB_URL)?;
    let indirect_table_name = TableName::from(&indirect_table);
    match window {
        Some(window) => {
            let main_table = data_service::get_main_table(window, &tables);
            assert!(main_table.is_some());
            let main_table = main_table.unwrap();

            let indirect_tab: Option<&(TableName, Tab)> = window
                .indirect_tabs
                .iter()
                .find(|&(_linker_table, tab)| tab.table_name == indirect_table_name);

            match indirect_tab {
                Some(&(ref linker_table, ref indirect_tab)) => {
                    let rows = data_service::get_indirect_records_service(
                        &dm,
                        &tables,
                        &main_table,
                        &record_id,
                        &indirect_tab,
                        &linker_table,
                        PAGE_SIZE,
                        page,
                    )?;
                    Ok(Some(Json(rows)))
                }
                None => Ok(None),
            }
        }
        None => Ok(None),
    }
}

#[get("/")]
fn webclient_index() -> Option<NamedFile> {
    NamedFile::open(Path::new("./public/index.html")).ok()
}

#[get("/<file..>")]
fn webclient(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("./public/").join(file)).ok()
}

#[get("/")]
fn redirect_to_web() -> Redirect {
    Redirect::to("/web/")
}

#[get("/favicon.ico")]
fn favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("./public/img/favicon.ico")).ok()
}

pub fn rocket() -> Rocket {
    rocket::ignite()
        .attach(AdHoc::on_response(|_req, resp| {
            resp.set_header(AccessControlAllowOrigin::Any);
        }))
        .mount("/", routes![redirect_to_web, favicon])
        .mount("/web", routes![webclient_index, webclient])
        .mount(
            "/data",
            routes![
                get_data,
                get_data_with_page,
                get_detailed_record,
                get_has_many_records,
                get_indirect_records,
            ],
        )
        .mount("/lookup", routes![get_lookup_data])
        .mount("/lookup_all", routes![get_window_lookup_data])
        .mount("/record_count", routes![get_total_records])
        .mount("/window", routes![get_window,])
        .mount("/windows", routes![get_windows,])
}
