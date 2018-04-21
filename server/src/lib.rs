#![deny(warnings)]
#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(match_default_bindings)]

extern crate diwata_intel as intel;
#[macro_use]
extern crate lazy_static;
extern crate rocket;
extern crate rocket_contrib;
extern crate rustorm;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate structopt_derive;
extern crate structopt;

use structopt::StructOpt;

pub use error::ServiceError;
use intel::Window;
use intel::cache;
use intel::data_container::Filter;
use intel::data_container::Lookup;
use intel::data_container::Sort;
use intel::data_modify;
use intel::data_read;
use intel::data_read::RecordDetail;
use intel::tab;
use intel::tab::Tab;
use intel::table_intel;
use intel::window::{self, GroupedWindow};
use rocket::Config;
use rocket::Rocket;
use rocket::config::ConfigError;
use rocket::fairing::AdHoc;
use rocket::http::hyper::header::AccessControlAllowOrigin;
use rocket::response::NamedFile;
use rocket::response::Redirect;
use rocket_contrib::Json;
use rustorm::EntityManager;
use rustorm::Pool;
use rustorm::RecordManager;
use rustorm::Rows;
use rustorm::TableName;
use rustorm::pool;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

mod error;

static PAGE_SIZE: u32 = 40;

lazy_static! {
    pub static ref DB_URL: Mutex<Option<String>> = Mutex::new(None);
    pub static ref POOL: Arc<Mutex<Pool>> = { Arc::new(Mutex::new(Pool::new())) };
}

fn get_db_url_value() -> Result<Option<String>, ServiceError> {
    match DB_URL.lock() {
        Ok(db_url) => {
            if let Some(ref db_url) = *db_url {
                Ok(Some(db_url.to_owned()))
            } else {
                Ok(None)
            }
        }
        Err(e) => Err(ServiceError::GenericError(format!("{}", e))),
    }
}

fn get_db_url() -> Result<String, ServiceError> {
    match get_db_url_value() {
        Ok(db_url) => {
            if let Some(ref db_url) = db_url {
                Ok(db_url.to_owned())
            } else {
                Err(ServiceError::NoDbUrlSpecified)
            }
        }
        Err(e) => Err(e),
    }
}

#[get("/")]
pub fn get_db_url_service() -> Result<Option<String>, ServiceError> {
    get_db_url_value()
}

pub fn set_db_url(new_url: String) -> Result<(), ServiceError> {
    match DB_URL.lock() {
        Ok(mut db_url) => {
            *db_url = Some(new_url);
            Ok(())
        }
        Err(e) => Err(ServiceError::GenericError(format!("{}", e))),
    }
}

fn get_pool_em() -> Result<EntityManager, ServiceError> {
    let mut pool = match POOL.lock() {
        Ok(pool) => pool,
        Err(_e) => return Err(ServiceError::PoolResourceError),
    };
    let db_url = &get_db_url()?;
    match pool.em(db_url) {
        Ok(em) => Ok(em),
        Err(e) => return Err(ServiceError::DbError(e)),
    }
}

#[get("/")]
pub fn test_db_url_connection() -> Result<Json<bool>, ServiceError> {
    let db_url = &get_db_url()?;
    pool::test_connection(&db_url)?;
    Ok(Json(true))
}

fn get_pool_dm() -> Result<RecordManager, ServiceError> {
    let mut pool = match POOL.lock() {
        Ok(pool) => pool,
        Err(_e) => return Err(ServiceError::PoolResourceError),
    };
    let db_url = &get_db_url()?;
    match pool.dm(db_url) {
        Ok(em) => Ok(em),
        Err(e) => return Err(ServiceError::DbError(e)),
    }
}

#[get("/")]
pub fn get_windows() -> Result<Json<Vec<GroupedWindow>>, ServiceError> {
    let em = get_pool_em()?;
    let db_url = &get_db_url()?;
    let grouped_windows: Vec<GroupedWindow> = window::get_grouped_windows_using_cache(&em, db_url)?;
    Ok(Json(grouped_windows))
}

#[get("/<table_name>")]
pub fn get_window(table_name: String) -> Result<Option<Json<Window>>, ServiceError> {
    let em = get_pool_em()?;
    let db_url = &get_db_url()?;
    let mut cache_pool = cache::CACHE_POOL.lock().unwrap();
    println!("{:#?}", db_url);
    let windows = cache_pool.get_cached_windows(&em, db_url)?;
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &windows);
    match window {
        Some(window) => Ok(Some(Json(window.to_owned()))),
        None => Ok(None),
    }
}

#[get("/<table_name>")]
pub fn get_total_records(table_name: String) -> Result<Option<Json<u64>>, ServiceError> {
    let em = get_pool_em()?;
    let db_url = &get_db_url()?;
    let mut cache_pool = cache::CACHE_POOL.lock().unwrap();
    let table_name = TableName::from(&table_name);
    let tables = cache_pool.get_cached_tables(&em, db_url)?;
    let table = table_intel::get_table(&table_name, &tables);
    match table {
        Some(table) => {
            let count = data_read::get_total_records(&em, &table.name)?;
            Ok(Some(Json(count)))
        }
        None => Ok(None),
    }
}

#[get("/<table_name>")]
pub fn get_data(table_name: String) -> Result<Option<Json<Rows>>, ServiceError> {
    get_data_with_page(table_name, 1)
}

#[get("/<table_name>/page/<page>")]
pub fn get_data_with_page(
    table_name: String,
    page: u32,
) -> Result<Option<Json<Rows>>, ServiceError> {
    let em = get_pool_em()?;
    let dm = get_pool_dm()?;
    let db_url = &get_db_url()?;
    let mut cache_pool = cache::CACHE_POOL.lock().unwrap();
    let windows = cache_pool.get_cached_windows(&em, db_url)?;
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &windows);
    let tables = cache_pool.get_cached_tables(&em, db_url)?;
    match window {
        Some(window) => {
            let rows: Rows =
                data_read::get_maintable_data(&dm, &tables, &window, None, None, page, PAGE_SIZE)?;
            Ok(Some(Json(rows)))
        }
        None => Ok(None),
    }
}

#[get("/<table_name>/page/<page>/sort/<sort>")]
pub fn get_data_with_page_sort(
    table_name: String,
    page: u32,
    sort: String,
) -> Result<Option<Json<Rows>>, ServiceError> {
    let em = get_pool_em()?;
    let dm = get_pool_dm()?;
    let db_url = &get_db_url()?;
    let mut cache_pool = cache::CACHE_POOL.lock().unwrap();
    let windows = cache_pool.get_cached_windows(&em, db_url)?;
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &windows);
    let tables = cache_pool.get_cached_tables(&em, db_url)?;
    let sort = Sort::from_str(&sort);
    match window {
        Some(window) => {
            let rows: Rows = data_read::get_maintable_data(
                &dm,
                &tables,
                &window,
                None,
                Some(sort),
                page,
                PAGE_SIZE,
            )?;
            Ok(Some(Json(rows)))
        }
        None => Ok(None),
    }
}

#[get("/<table_name>/page/<page>/filter/<filter>")]
pub fn get_data_with_page_filter(
    table_name: String,
    page: u32,
    filter: String,
) -> Result<Option<Json<Rows>>, ServiceError> {
    let em = get_pool_em()?;
    let dm = get_pool_dm()?;
    let db_url = &get_db_url()?;
    let mut cache_pool = cache::CACHE_POOL.lock().unwrap();
    let windows = cache_pool.get_cached_windows(&em, db_url)?;
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &windows);
    let tables = cache_pool.get_cached_tables(&em, db_url)?;
    let filter = Filter::from_str(&filter);
    match window {
        Some(window) => {
            let rows: Rows = data_read::get_maintable_data(
                &dm,
                &tables,
                &window,
                Some(filter),
                None,
                page,
                PAGE_SIZE,
            )?;
            Ok(Some(Json(rows)))
        }
        None => Ok(None),
    }
}

#[get("/<table_name>/sort/<sort>")]
pub fn get_data_with_sort(
    table_name: String,
    sort: String,
) -> Result<Option<Json<Rows>>, ServiceError> {
    get_data_with_page_filter_sort(table_name, 1, None, Some(sort))
}

#[get("/<table_name>/page/<page>/filter/<filter>/sort/<sort>")]
pub fn get_data_with_page_filter_sort(
    table_name: String,
    page: u32,
    filter: Option<String>,
    sort: Option<String>,
) -> Result<Option<Json<Rows>>, ServiceError> {
    let em = get_pool_em()?;
    let dm = get_pool_dm()?;
    let db_url = &get_db_url()?;
    let mut cache_pool = cache::CACHE_POOL.lock().unwrap();
    let windows = cache_pool.get_cached_windows(&em, db_url)?;
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &windows);
    let tables = cache_pool.get_cached_tables(&em, db_url)?;
    let filter = filter.map(|s| Filter::from_str(&s));
    let sort = sort.map(|s| Sort::from_str(&s));
    match window {
        Some(window) => {
            let rows: Rows = data_read::get_maintable_data(
                &dm,
                &tables,
                &window,
                filter,
                sort,
                page,
                PAGE_SIZE,
            )?;
            Ok(Some(Json(rows)))
        }
        None => Ok(None),
    }
}

#[get("/<table_name>/select/<record_id>")]
pub fn get_detailed_record(
    table_name: String,
    record_id: String,
) -> Result<Option<Json<RecordDetail>>, ServiceError> {
    let dm = get_pool_dm()?;
    let em = get_pool_em()?;
    let db_url = &get_db_url()?;
    let mut cache_pool = cache::CACHE_POOL.lock().unwrap();
    let windows = cache_pool.get_cached_windows(&em, db_url)?;
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &windows);
    let tables = cache_pool.get_cached_tables(&em, db_url)?;
    match window {
        Some(window) => {
            let dao: Option<RecordDetail> = data_read::get_selected_record_detail(
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

/// retrieve the first page of all lookup data
/// used in this window
/// Note: window is identified by it's table name of the main tab
#[get("/<table_name>")]
pub fn get_window_lookup_data(table_name: String) -> Result<Option<Json<Lookup>>, ServiceError> {
    let dm = get_pool_dm()?;
    let em = get_pool_em()?;
    let db_url = &get_db_url()?;
    let mut cache_pool = cache::CACHE_POOL.lock().unwrap();
    let windows = cache_pool.get_cached_windows(&em, db_url)?;
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &windows);
    let tables = cache_pool.get_cached_tables(&em, db_url)?;
    match window {
        Some(window) => {
            let lookup: Lookup =
                data_read::get_all_lookup_for_window(&dm, &tables, &window, PAGE_SIZE)?;
            Ok(Some(Json(lookup)))
        }
        None => Ok(None),
    }
}

/// retrieve the lookup data of this table at next page
/// Usually the first page of the lookup data is preloaded with the window that
/// may display them in order for the user to see something when clicking on the dropdown list.
/// When the user scrolls to the bottom of the dropdown, a http request is done to retrieve the
/// next page. All other lookup that points to the same table is also updated
#[get("/<table_name>/<page>")]
pub fn get_lookup_data(table_name: String, page: u32) -> Result<Option<Json<Rows>>, ServiceError> {
    let dm = get_pool_dm()?;
    let em = get_pool_em()?;
    let db_url = &get_db_url()?;
    let mut cache_pool = cache::CACHE_POOL.lock().unwrap();
    let windows = cache_pool.get_cached_windows(&em, db_url)?;
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &windows);
    let tables = cache_pool.get_cached_tables(&em, db_url)?;
    match window {
        Some(window) => {
            let rows: Rows =
                data_read::get_lookup_data_of_tab(&dm, &tables, &window.main_tab, PAGE_SIZE, page)?;
            Ok(Some(Json(rows)))
        }
        None => Ok(None),
    }
}

#[get("/<table_name>/select/<record_id>/has_many/<has_many_table>/<page>")]
pub fn get_has_many_records_with_page(
    table_name: String,
    record_id: String,
    has_many_table: String,
    page: u32,
) -> Result<Option<Json<Rows>>, ServiceError> {
    get_has_many_records_with_page_sort(table_name, record_id, has_many_table, page, None)
}

/// retrieve records from a has_many table based on the selected main records
/// from the main table
#[get("/<table_name>/select/<record_id>/has_many/<has_many_table>/<page>/sort/<sort>")]
pub fn get_has_many_records_with_page_sort(
    table_name: String,
    record_id: String,
    has_many_table: String,
    page: u32,
    sort: Option<String>,
) -> Result<Option<Json<Rows>>, ServiceError> {
    let dm = get_pool_dm()?;
    let em = get_pool_em()?;
    let db_url = &get_db_url()?;
    let mut cache_pool = cache::CACHE_POOL.lock().unwrap();
    let windows = cache_pool.get_cached_windows(&em, db_url)?;
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &windows);
    let tables = cache_pool.get_cached_tables(&em, db_url)?;
    let has_many_table_name = TableName::from(&has_many_table);
    println!("sort: {:#?}", sort);
    match window {
        Some(window) => {
            let main_table = data_read::get_main_table(window, &tables);
            assert!(main_table.is_some());
            let main_table = main_table.unwrap();
            let has_many_tab = tab::find_tab(&window.has_many_tabs, &has_many_table_name);
            match has_many_tab {
                Some(has_many_tab) => {
                    let rows = data_read::get_has_many_records_service(
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

#[get("/<table_name>/select/<record_id>/indirect/<indirect_table>/<page>")]
pub fn get_indirect_records_with_page(
    table_name: String,
    record_id: String,
    indirect_table: String,
    page: u32,
) -> Result<Option<Json<Rows>>, ServiceError> {
    get_indirect_records_with_page_sort(table_name, record_id, indirect_table, page, None)
}

/// retrieve records from a has_many table based on the selected main records
/// from the main table
#[get("/<table_name>/select/<record_id>/indirect/<indirect_table>/<page>/sort/<sort>")]
pub fn get_indirect_records_with_page_sort(
    table_name: String,
    record_id: String,
    indirect_table: String,
    page: u32,
    sort: Option<String>,
) -> Result<Option<Json<Rows>>, ServiceError> {
    let dm = get_pool_dm()?;
    let em = get_pool_em()?;
    let db_url = &get_db_url()?;
    let mut cache_pool = cache::CACHE_POOL.lock().unwrap();
    let windows = cache_pool.get_cached_windows(&em, db_url)?;
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &windows);
    let tables = cache_pool.get_cached_tables(&em, db_url)?;
    let indirect_table_name = TableName::from(&indirect_table);
    println!("sort: {:#?}", sort);
    match window {
        Some(window) => {
            let main_table = data_read::get_main_table(window, &tables);
            assert!(main_table.is_some());
            let main_table = main_table.unwrap();

            let indirect_tab: Option<&(TableName, Tab)> = window
                .indirect_tabs
                .iter()
                .find(|&(_linker_table, tab)| tab.table_name == indirect_table_name);

            match indirect_tab {
                Some(&(ref linker_table, ref indirect_tab)) => {
                    let rows = data_read::get_indirect_records_service(
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

#[delete("/<table_name>", data = "<record_ids>")]
pub fn delete_records(
    table_name: String,
    record_ids: Json<Vec<String>>,
) -> Result<Option<Json<Rows>>, ServiceError> {
    let dm = get_pool_dm()?;
    let em = get_pool_em()?;
    let db_url = &get_db_url()?;
    let table_name = TableName::from(&table_name);
    let mut cache_pool = cache::CACHE_POOL.lock().unwrap();
    let windows = cache_pool.get_cached_windows(&em, db_url)?;
    let window = window::get_window(&table_name, &windows);
    let tables = cache_pool.get_cached_tables(&em, db_url)?;
    match window {
        Some(window) => {
            let main_table = data_read::get_main_table(window, &tables);
            assert!(main_table.is_some());
            let main_table = main_table.unwrap();
            println!(
                "delete these records: {:?} from table: {:?}",
                record_ids, table_name
            );
            let rows = data_modify::delete_records(&dm, &main_table, &*record_ids)?;
            Ok(Some(Json(rows)))
        }
        None => Ok(None),
    }
}

pub fn rocket(address: Option<String>, port: Option<u16>) -> Result<Rocket, ConfigError> {
    let address = match address {
        Some(address) => address,
        None => "0.0.0.0".to_string(),
    };
    let port = match port {
        Some(port) => port,
        None => 8000,
    };
    println!("address: {:?}", address);
    println!("port: {:?}", port);
    let mut config = Config::development()?;
    config.set_port(port);
    config.set_address(address)?;
    let server = rocket::custom(config, true)
        .attach(AdHoc::on_response(|_req, resp| {
            resp.set_header(AccessControlAllowOrigin::Any);
        }))
        .attach(AdHoc::on_request(|req, _resp| {
            let db_urls: Vec<&str> = req.headers().get("db_url").collect();
            println!("db_urls: {:?}", db_urls);
            if db_urls.len() != 1 {
                println!("no db_url specified, using the previous one");
            } else {
                let db_url = db_urls[0];
                match set_db_url(db_url.to_string()) {
                    Ok(_) => println!("db_url is set"),
                    Err(e) => println!("error setting db_url: {:?}", e),
                }
            }
        }))
        .mount("/", routes![redirect_to_web, favicon])
        .mount("/web", routes![webclient_index, webclient])
        .mount("/test", routes![test_db_url_connection])
        .mount("/db_url", routes![get_db_url_service])
        .mount(
            "/data",
            routes![
                get_data,
                get_data_with_sort,
                get_data_with_page,
                get_data_with_page_filter,
                get_data_with_page_sort,
                get_data_with_page_filter_sort,
                get_detailed_record,
                get_has_many_records_with_page,
                get_has_many_records_with_page_sort,
                get_indirect_records_with_page_sort,
                delete_records,
            ],
        )
        .mount("/lookup", routes![get_lookup_data])
        .mount("/lookup_all", routes![get_window_lookup_data])
        .mount("/record_count", routes![get_total_records])
        .mount("/window", routes![get_window,])
        .mount("/windows", routes![get_windows,]);
    Ok(server)
}

#[derive(StructOpt, Debug)]
#[structopt(name = "diwata", about = "A user friendly database interface")]
struct Opt {
    #[structopt(short = "u", long = "db-url",
                help = "Database url to connect to, when set all data is exposed without login needed in the client side")]
    db_url: Option<String>,
    #[structopt(short = "a", long = "address",
                help = "The address the server would listen, default is 0.0.0.0")]
    address: Option<String>,
    #[structopt(short = "p", long = "port",
                help = "What port this server would listen to, default is 8000")]
    port: Option<u16>,
}

pub fn start() {
    let opt = Opt::from_args();
    println!("opt: {:?}", opt);
    if let Some(db_url) = opt.db_url {
        match set_db_url(db_url) {
            Ok(_) => println!("url is set"),
            Err(_) => println!("unable to set db_url"),
        }
    }
    match rocket(opt.address, opt.port) {
        Ok(server) => {
            println!("Launching..");
            server.launch();
        }
        Err(e) => panic!("unable to initialize server: {}", e),
    }
}
