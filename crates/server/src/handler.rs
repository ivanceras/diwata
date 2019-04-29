///
/// Derive from:
/// https://github.com/nrc/cargo-src/blob/master/src/server.rs
use futures;
use futures::{
    Future,
    Stream,
};
use hyper::{
    error::Error,
    header::ContentType,
    server::{
        Response,
        Service,
    },
    Headers,
    Request,
    StatusCode,
};

use crate::{
    context::Context,
    credentials::Credentials,
    error::ServiceError,
    global,
};
use diwata_intel::{
    data_container::{
        Filter,
        RecordChangeset,
        SaveContainer,
        Sort,
    },
    data_modify,
    data_read,
    tab::{
        self,
        Tab,
    },
    window,
};
use hyper::server::Http;
use log::*;
use rustorm::{
    Rows,
    TableName,
};
use serde::Serialize;
use serde_json;
use std::{
    convert::TryFrom,
    sync::{
        Arc,
        Mutex,
    },
};
use structopt::StructOpt;

/// An instance of the server. Runs a session of rustw.
pub struct Server {
    #[allow(unused)]
    db_url: String,
}

#[derive(Clone)]
pub struct Instance {
    server: Arc<Mutex<Server>>,
}

impl Instance {
    pub fn new(server: Server) -> Instance {
        Instance {
            server: Arc::new(Mutex::new(server)),
        }
    }
}

impl Service for Instance {
    type Error = Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;
    type Request = Request;
    type Response = Response;

    fn call(&self, req: Request) -> Self::Future {
        let uri = req.uri().clone();
        self.server
            .lock()
            .unwrap()
            .route(uri.path(), uri.query(), req)
    }
}

impl Server {
    pub fn route(
        &self,
        path: &str,
        query: Option<&str>,
        req: Request,
    ) -> <Instance as Service>::Future {
        handle_route(path, query, req)
    }
}

pub fn handle_route(
    mut path: &str,
    query: Option<&str>,
    req: Request,
) -> <Instance as Service>::Future {
    trace!("route: path: {:?}, query: {:?}", path, query);
    println!("route: path: {:?}, query: {:?}", path, query);

    path = path.trim_matches('/');
    let path: Vec<_> = path.split('/').collect();
    let head = path[0];
    let tail = &path[1..];
    let result = if head == "" {
        handle_index(req)
    } else if head == "static" {
        handle_static(req, tail)
    } else if head == "windows" {
        create_response(handle_windows(req))
    } else if head == "window" {
        create_response(handle_window(req, tail))
    } else if head == "data" {
        create_response(handle_data(req, tail))
    } else if head == "select" {
        create_response(handle_select(req, tail))
    } else if head == "has_many_select" {
        create_response(handle_has_many(req, tail))
    } else if head == "indirect_select" {
        create_response(handle_indirect(req, tail))
    } else if head == "lookup" {
        create_response(handle_lookup(req, tail))
    } else if head == "lookup_all" {
        create_response(handle_lookup_all(req, tail))
    } else if head == "test" {
        create_response(handle_test(req))
    } else if head == "is_login_required" {
        create_response(handle_login_required(req))
    } else if head == "database_name" {
        create_response(handle_database_name(req))
    } else if head == "delete" {
        return handle_delete(req, tail);
    } else if head == "record_changeset" {
        return handle_record_changeset(req, tail);
    } else if head == "tab_changeset" {
        return handle_tab_changeset(req);
    } else {
        handle_error(req, StatusCode::NotFound, "Page not found".to_owned())
    };

    Box::new(futures::future::ok(result))
}

fn require_credentials(req: &Request) -> Result<(), ServiceError> {
    let is_required = global::is_login_required()?;

    if is_required {
        let credentials: Result<Credentials, ServiceError> =
            TryFrom::try_from(req);
        match credentials {
            Ok(credentials) => {
                global::test_credentials(
                    &credentials.username,
                    &credentials.password,
                )?;
                Ok(())
            }
            Err(_e) => Err(ServiceError::RequiredCredentialsNotFound),
        }
    } else {
        Ok(())
    }
}

fn handle_database_name(_req: Request) -> Result<impl Serialize, ServiceError> {
    let ret = data_read::get_database_name(&global::get_pool_em()?)?;
    Ok(ret)
}

fn handle_test(req: Request) -> Result<(), ServiceError> {
    require_credentials(&req)?;
    Ok(())
}

fn handle_login_required(_req: Request) -> Result<bool, ServiceError> {
    global::is_login_required()
}

fn handle_index(_req: Request) -> Response {
    handle_static(_req, &["index.html"])
}

fn handle_static(_req: Request, _path: &[&str]) -> Response {
    Response::new().with_body("Soon")
}

fn handle_error(_req: Request, status: StatusCode, msg: String) -> Response {
    debug!("ERROR: {} ({})", msg, status);
    Response::new().with_status(status).with_body(msg)
}

fn handle_windows(req: Request) -> Result<impl Serialize, ServiceError> {
    require_credentials(&req)?;
    let credentials: Result<Credentials, ServiceError> =
        TryFrom::try_from(&req);
    let context = Context::create(credentials)?;
    let is_login_required = global::is_login_required()?;
    let db_url = if is_login_required {
        global::get_role_db_url()?
    } else {
        global::get_db_url()?
    };

    // TODO: incorporate this into the context, such that
    // the logic for getting the db_url is unified
    let ret = window::get_grouped_windows_using_cache(&context.em, &db_url)?;
    Ok(ret)
}

fn handle_window(
    req: Request,
    tail: &[&str],
) -> Result<impl Serialize, ServiceError> {
    require_credentials(&req)?;
    let table_name = &tail[0];
    let credentials: Result<Credentials, ServiceError> =
        TryFrom::try_from(&req);
    let context = Context::create(credentials)?;
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &context.windows);
    match window {
        Some(window) => Ok(window.to_owned()),
        None => Err(ServiceError::NotFound),
    }
}

///
/// /data/<table_name>/page/<page>/filter/<filter>/sort/<sort>/
fn handle_data(
    req: Request,
    path: &[&str],
) -> Result<impl Serialize, ServiceError> {
    require_credentials(&req)?;
    let table_name = path[0];
    let tail = &path[1..];
    let key_value: Vec<(&str, &str)> =
        tail.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();
    let mut page = 1;
    let mut filter_str = None;
    let mut sort_str = None;
    for (k, v) in key_value {
        if k == "page" {
            page = v.parse().unwrap();
        } else if k == "filter" {
            filter_str = Some(v);
        } else if k == "sort" {
            sort_str = Some(v);
        }
    }
    let credentials: Result<Credentials, ServiceError> =
        TryFrom::try_from(&req);
    let context = Context::create(credentials)?;
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &context.windows);
    let filter = filter_str.map(Filter::from);
    let sort = sort_str.map(Sort::from);
    match window {
        Some(window) => {
            let rows = data_read::get_maintable_data(
                &context.em,
                &context.dm,
                &context.tables,
                &window,
                filter,
                sort,
                page,
                global::PAGE_SIZE,
            )?;
            Ok(rows)
        }
        None => Err(ServiceError::NotFound),
    }
}

///
/// /select/<table_name>/<record_id>
fn handle_select(
    req: Request,
    path: &[&str],
) -> Result<impl Serialize, ServiceError> {
    require_credentials(&req)?;
    let table_name = path[0];
    let record_id = path[1];
    let table_name = TableName::from(&table_name);
    let credentials: Result<Credentials, ServiceError> =
        TryFrom::try_from(&req);
    let context = Context::create(credentials)?;
    let window = window::get_window(&table_name, &context.windows);
    match window {
        Some(window) => {
            let dao = data_read::get_selected_record_detail(
                &context.em,
                &context.dm,
                &context.tables,
                &window,
                &record_id,
                global::PAGE_SIZE,
            )?;
            match dao {
                Some(dao) => Ok(dao),
                None => Err(ServiceError::NotFound),
            }
        }
        None => Err(ServiceError::NotFound),
    }
}

///
///
///  /has_many_select/<table_name>/<record_id>/<has_many_table>/page/<page>/filter/<filter>/sort/<sort>
///
fn handle_has_many(
    req: Request,
    path: &[&str],
) -> Result<impl Serialize, ServiceError> {
    require_credentials(&req)?;
    let table_name = path[0];
    let record_id = path[1];
    let has_many_table = path[2];
    let tail = &path[3..];
    let key_value: Vec<(&str, &str)> =
        tail.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();
    let mut page = 1;
    let mut filter_str = None;
    let mut sort_str = None;
    for (k, v) in key_value {
        if k == "page" {
            page = v.parse().unwrap();
        } else if k == "filter" {
            filter_str = Some(v);
        } else if k == "sort" {
            sort_str = Some(v);
        }
    }
    let filter = filter_str.map(Filter::from);
    let sort = sort_str.map(Sort::from);
    let credentials: Result<Credentials, ServiceError> =
        TryFrom::try_from(&req);
    let context = Context::create(credentials)?;
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &context.windows);
    let has_many_table_name = TableName::from(&has_many_table);
    match window {
        Some(window) => {
            let main_table = data_read::get_main_table(window, &context.tables);
            assert!(main_table.is_some());
            let main_table = main_table.unwrap();
            let has_many_tab =
                tab::find_tab(&window.has_many_tabs, &has_many_table_name);
            match has_many_tab {
                Some(has_many_tab) => {
                    let rows = data_read::get_has_many_records_service(
                        &context.em,
                        &context.dm,
                        &context.tables,
                        &main_table,
                        &record_id,
                        has_many_tab,
                        filter,
                        sort,
                        global::PAGE_SIZE,
                        page,
                    )?;
                    Ok(rows)
                }
                None => Err(ServiceError::NotFound),
            }
        }
        None => Err(ServiceError::NotFound),
    }
}

///
///
///  /indirect_select/<table_name>/<record_id>/<indirect_table>/page/<page>/filter/<filter>/sort/<sort>
///
fn handle_indirect(
    req: Request,
    path: &[&str],
) -> Result<impl Serialize, ServiceError> {
    require_credentials(&req)?;
    let table_name = path[0];
    let record_id = path[1];
    let indirect_table = path[2];
    let tail = &path[3..];
    let key_value: Vec<(&str, &str)> =
        tail.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();
    let mut page = 1;
    let mut filter_str = None;
    let mut sort_str = None;
    for (k, v) in key_value {
        if k == "page" {
            page = v.parse().unwrap();
        } else if k == "filter" {
            filter_str = Some(v);
        } else if k == "sort" {
            sort_str = Some(v);
        }
    }
    let filter = filter_str.map(Filter::from);
    let sort = sort_str.map(Sort::from);
    let credentials: Result<Credentials, ServiceError> =
        TryFrom::try_from(&req);
    let context = Context::create(credentials)?;
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &context.windows);
    let indirect_table_name = TableName::from(&indirect_table);
    match window {
        Some(window) => {
            let main_table = data_read::get_main_table(window, &context.tables);
            assert!(main_table.is_some());
            let main_table = main_table.unwrap();

            let indirect_tab: Option<&(TableName, Tab)> =
                window.indirect_tabs.iter().find(|&(_linker_table, tab)| {
                    tab.table_name == indirect_table_name
                });

            match indirect_tab {
                Some(&(ref linker_table, ref indirect_tab)) => {
                    let rows = data_read::get_indirect_records_service(
                        &context.em,
                        &context.dm,
                        &context.tables,
                        &main_table,
                        &record_id,
                        &indirect_tab,
                        &linker_table,
                        filter,
                        sort,
                        global::PAGE_SIZE,
                        page,
                    )?;
                    Ok(rows)
                }
                None => Err(ServiceError::NotFound),
            }
        }
        None => Err(ServiceError::NotFound),
    }
}

/// retrieve the lookup data of this table at next page
/// Usually the first page of the lookup data is preloaded with the window that
/// may display them in order for the user to see something when clicking on the dropdown list.
/// When the user scrolls to the bottom of the dropdown, a http request is done to retrieve the
/// next page. All other lookup that points to the same table is also updated
fn handle_lookup(
    req: Request,
    path: &[&str],
) -> Result<impl Serialize, ServiceError> {
    require_credentials(&req)?;
    let table_name = path[0];
    let page: u32 = path[1].parse().unwrap();
    let credentials: Result<Credentials, ServiceError> =
        TryFrom::try_from(&req);
    let context = Context::create(credentials)?;

    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &context.windows);
    match window {
        Some(window) => {
            let rows = data_read::get_lookup_data_of_tab(
                &context.em,
                &context.dm,
                &context.tables,
                &window.main_tab,
                global::PAGE_SIZE,
                page,
            )?;
            Ok(rows)
        }
        None => Err(ServiceError::NotFound),
    }
}

/// retrieve the first page of all lookup data
/// used in this window
/// Note: window is identified by it's table name of the main tab
fn handle_lookup_all(
    req: Request,
    path: &[&str],
) -> Result<impl Serialize, ServiceError> {
    require_credentials(&req)?;
    let table_name = path[0];
    let credentials: Result<Credentials, ServiceError> =
        TryFrom::try_from(&req);
    let context = Context::create(credentials)?;
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &context.windows);
    match window {
        Some(window) => {
            let lookup = data_read::get_all_lookup_for_window(
                &context.em,
                &context.dm,
                &context.tables,
                &window,
                global::PAGE_SIZE,
            )?;
            Ok(lookup)
        }
        None => Err(ServiceError::NotFound),
    }
}

// https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request?rq=1
// https://hyper.rs/guides/server/echo/
fn handle_delete(
    req: Request,
    path: &[&str],
) -> Box<Future<Item = Response, Error = Error>> {
    let is_cred_ok = is_credentials_ok(&req);

    let credentials: Result<Credentials, ServiceError> =
        TryFrom::try_from(&req);
    let context = Context::create(credentials).unwrap();

    let table_name = path[0].to_string();
    let f = req.body().concat2().map(move |chunk| {
        let result = if is_cred_ok {
            let body = chunk.into_iter().collect::<Vec<u8>>();
            let body_str = String::from_utf8(body.clone()).unwrap();
            let record_ids: Vec<String> =
                serde_json::from_str(&body_str).unwrap();
            delete_records(&context, &table_name, &record_ids)
        } else {
            Err(ServiceError::RequiredCredentialsNotFound)
        };
        create_response(result)
    });
    Box::new(f)
}

fn handle_record_changeset(
    req: Request,
    path: &[&str],
) -> Box<Future<Item = Response, Error = Error>> {
    let is_cred_ok = is_credentials_ok(&req);

    let credentials: Result<Credentials, ServiceError> =
        TryFrom::try_from(&req);
    let context = Context::create(credentials).unwrap();

    let table_name = path[0].to_string();
    let f = req.body().concat2().map(move |chunk| {
        let result = if is_cred_ok {
            let body = chunk.into_iter().collect::<Vec<u8>>();
            let body_str = String::from_utf8(body).unwrap();
            let changeset: Result<RecordChangeset, _> =
                serde_json::from_str(&body_str);
            let changeset = changeset.unwrap_or_else(|_| {
                panic!("unable to serialize from json {}", body_str)
            });
            update_record_changeset(&context, &table_name, &changeset)
        } else {
            Err(ServiceError::RequiredCredentialsNotFound)
        };
        create_response(result)
    });
    Box::new(f)
}

fn is_credentials_ok(req: &Request) -> bool {
    match require_credentials(&req) {
        Ok(()) => true,
        Err(_) => false,
    }
}

fn handle_tab_changeset(
    req: Request,
) -> Box<Future<Item = Response, Error = Error>> {
    let is_cred_ok = is_credentials_ok(&req);
    let credentials: Result<Credentials, ServiceError> =
        TryFrom::try_from(&req);
    let context = Context::create(credentials).unwrap();

    let f = req.body().concat2().map(move |chunk| {
        let result = if is_cred_ok {
            let body = chunk.into_iter().collect::<Vec<u8>>();
            let body_str = String::from_utf8(body).unwrap();
            let container: SaveContainer =
                serde_json::from_str(&body_str).unwrap();
            update_tab_changeset(&context, &container)
        } else {
            Err(ServiceError::RequiredCredentialsNotFound)
        };
        create_response(result)
    });
    Box::new(f)
}

fn delete_records(
    context: &Context,
    table_name: &str,
    record_ids: &[String],
) -> Result<Rows, ServiceError> {
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &context.windows);
    match window {
        Some(window) => {
            let main_table = data_read::get_main_table(window, &context.tables);
            assert!(main_table.is_some());
            let main_table = main_table.unwrap();
            println!(
                "delete these records: {:?} from table: {:?}",
                record_ids, table_name
            );
            let rows = data_modify::delete_records(
                &context.dm,
                &main_table,
                &record_ids,
            )?;
            Ok(rows)
        }
        None => Err(ServiceError::NotFound),
    }
}

fn update_tab_changeset(
    context: &Context,
    container: &SaveContainer,
) -> Result<(), ServiceError> {
    data_modify::save_container(&context.dm, &context.tables, &container)?;
    Ok(())
}

fn update_record_changeset(
    context: &Context,
    table_name: &str,
    changeset: &RecordChangeset,
) -> Result<(), ServiceError> {
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &context.windows);
    match window {
        Some(window) => {
            let table = data_read::get_main_table(window, &context.tables);
            assert!(table.is_some());
            let table = table.unwrap();
            data_modify::save_changeset(
                &context.dm,
                &context.tables,
                window,
                &table,
                changeset,
            )?;
            Ok(())
        }
        None => Err(ServiceError::NotFound),
    }
}

fn create_response<B: Serialize>(body: Result<B, ServiceError>) -> Response {
    match body {
        Ok(body) => {
            let json = serde_json::to_string(&body).unwrap();
            let mut headers = Headers::new();
            headers.set(ContentType::json());
            Response::new().with_headers(headers).with_body(json)
        }
        Err(e) => {
            eprintln!("\n\nWarning an error response: {:?}", e);
            match e {
                ServiceError::NotFound => {
                    Response::new()
                        .with_status(StatusCode::NotFound)
                        .with_body("Not Found")
                }
                ServiceError::DbError(_) => {
                    Response::new()
                        .with_status(StatusCode::BadRequest)
                        .with_body("Wrong credentials")
                }
                _ => Response::new().with_status(StatusCode::BadRequest),
            }
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "diwata", about = "A user friendly database interface")]
pub struct Opt {
    #[structopt(
        short = "u",
        long = "db-url",
        help = "Database url to connect to, when set all data is exposed without login needed in the client side"
    )]
    pub db_url: String,

    #[structopt(
        short = "a",
        long = "address",
        help = "The address the server would listen, default is 0.0.0.0",
        default_value = "0.0.0.0"
    )]
    pub address: String,

    #[structopt(
        short = "p",
        long = "port",
        help = "What port this server would listen to, default is 8000",
        default_value = "8000"
    )]
    pub port: u16,
    #[structopt(
        short = "c",
        long = "cache",
        help = "precache the tables and windows so the first web request loads instantly, this requires the db-url to be set and login_required disabled, in order to work"
    )]
    pub precache: bool,

    #[structopt(
        short = "l",
        long = "login-required",
        help = "If enabled, then the user must supply username and password in all of the API calls"
    )]
    pub login_required: bool,
}

pub fn run() -> Result<(), ServiceError> {
    let opt = Opt::from_args();
    println!("opt: {:?}", opt);
    global::set_db_url(&opt.db_url)?;
    println!("url is set");
    if opt.precache && !opt.login_required {
        println!("precaching..");
        global::precache()?;
        println!("precaching complete!");
    }
    global::set_login_required(opt.login_required)?;
    let addr = match format!("{}:{}", opt.address, opt.port).parse() {
        Ok(addr) => Ok(addr),
        Err(e) => Err(ServiceError::GenericError(format!("{}", e))),
    };
    let server = Server {
        db_url: opt.db_url.to_owned(),
    };
    let instance = Instance::new(server);
    let http = Http::new().bind(&addr?, move || Ok(instance.clone()));

    let bind = match http {
        Ok(http) => http,
        Err(e) => {
            return Err(ServiceError::GenericError(format!("{}", e)));
        }
    };
    match bind.run() {
        Ok(bind) => bind,
        Err(e) => {
            return Err(ServiceError::GenericError(format!("{}", e)));
        }
    };
    Ok(())
}
