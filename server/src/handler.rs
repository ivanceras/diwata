///
/// Derive from:
/// https://github.com/nrc/cargo-src/blob/master/src/server.rs
///
use futures;
use futures::Future;
use futures::Stream;
use hyper::error::Error;
use hyper::header::ContentType;
use hyper::server::Request;
use hyper::server::Response;
use hyper::server::Service;
use hyper::Headers;
use hyper::StatusCode;

use context::Context;
use error::ServiceError;
use hyper::server::Http;
use intel::data_container::RecordChangeset;
use intel::data_container::SaveContainer;
use intel::data_container::{Filter, Sort};
use intel::data_modify;
use intel::data_read;
use intel::tab;
use intel::tab::Tab;
use intel::window;
use rustorm::pool;
use rustorm::Rows;
use rustorm::TableName;
use serde::Serialize;
use serde_json;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// An instance of the server. Runs a session of rustw.
pub struct Server {}

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
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let uri = req.uri().clone();
        self.server
            .lock()
            .unwrap()
            .route(uri.path(), uri.query(), req)
    }
}

impl Server {
    pub fn new() -> Self {
        Server {}
    }

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
    } else if head == "db_url" {
        create_response(handle_db_url(req))
    } else if head == "database_name" {
        create_response(handle_database_name(req))
    } else if head == "delete" {
        return handle_delete(req, tail);
    } else if head == "record_changeset" {
        return handle_record_changeset(req, tail);
    } else if head == "tab_changeset" {
        return handle_tab_changeset(req);
    } else if head == "plugin" {
        create_response(handle_plugin(req, tail))
    } else {
        handle_error(req, StatusCode::NotFound, "Page not found".to_owned())
    };

    Box::new(futures::future::ok(result))
}

fn handle_plugin(_req: Request, _tail: &[&str]) -> Result<impl Serialize, ServiceError> {
    Ok(())    
}

fn handle_database_name(_req: Request) -> Result<impl Serialize, ServiceError>{
    let ret = data_read::get_database_name(&::get_pool_em()?)?;
    Ok(ret)
}

fn handle_test(_req: Request) -> Result<(), ServiceError> {
    let db_url = &::get_db_url()?;
    println!("test db_url: {}", db_url);
    let ret = pool::test_connection(&db_url)?;
    Ok(ret)
}

fn handle_db_url(_req: Request) -> Result<String, ServiceError> {
    ::get_db_url()
}

fn handle_index(_req: Request) -> Response {
    handle_static(_req, &["index.html"])
}

fn handle_static(_req: Request, path: &[&str]) -> Response {
    println!("handling static: {:?}", path);
    let mut path_buf = PathBuf::new();
    path_buf.push("public");
    path_buf.push("static");
    for p in path {
        path_buf.push(p);
    }
    trace!("handle_static: requesting `{}`", path_buf.to_str().unwrap());
    println!("handle_static: requesting `{}`", path_buf.to_str().unwrap());

    let content_type = match path_buf.extension() {
        Some(s) if s.to_str().unwrap() == "html" => ContentType::html(),
        Some(s) if s.to_str().unwrap() == "css" => ContentType("text/css".parse().unwrap()),
        Some(s) if s.to_str().unwrap() == "json" => ContentType::json(),
        _ => ContentType("application/octet-stream".parse().unwrap()),
    };
    println!("content type: {:?}", content_type);
    let bytes = {
        let mut file = match File::open(&path_buf){
            Ok(file) => file,
            Err(_e) =>  {return handle_not_found(_req);}
        };
        let mut contents = vec![];
        file.read_to_end(&mut contents).unwrap();
        contents
    };
    trace!(
        "handle_static: serving `{}`. {} bytes, {}",
        path_buf.to_str().unwrap(),
        bytes.len(),
        content_type
    );
    let mut res = Response::new();
    res.headers_mut().set(content_type);
    return res.with_body(bytes);
}

fn handle_not_found(_req: Request) -> Response {
    debug!("NOT FOUND");
    Response::new().with_status(StatusCode::NotFound).with_body("Not Found")
}
fn handle_error(_req: Request, status: StatusCode, msg: String) -> Response {
    debug!("ERROR: {} ({})", msg, status);

    Response::new().with_status(status).with_body(msg)
}

fn handle_windows(_req: Request) -> Result<impl Serialize, ServiceError> {
    let em = ::get_pool_em()?;
    let db_url = &::get_db_url()?;
    let ret = window::get_grouped_windows_using_cache(&em, db_url)?;
    Ok(ret)
}

fn handle_window(_req: Request, tail: &[&str]) -> Result<impl Serialize, ServiceError> {
    let table_name = &tail[0];
    let context = Context::create()?;
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &context.windows);
    match window {
        Some(window) => Ok(window.to_owned()),
        None => Err(ServiceError::NotFound),
    }
}

///
/// /data/<table_name>/page/<page>/filter/<filter>/sort/<sort>/
///
fn handle_data(_req: Request, path: &[&str]) -> Result<impl Serialize, ServiceError> {
    let table_name = path[0];
    let tail = &path[1..];
    let key_value: Vec<(&str, &str)> = tail.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();
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
    let context = Context::create()?;
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &context.windows);
    let filter = filter_str.map(|s| Filter::from_str(s));
    let sort = sort_str.map(|s| Sort::from_str(s));
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
                ::PAGE_SIZE,
            )?;
            Ok(rows)
        }
        None => Err(ServiceError::NotFound),
    }
}

///
/// /select/<table_name>/<record_id>
///
fn handle_select(_req: Request, path: &[&str]) -> Result<impl Serialize, ServiceError> {
    let table_name = path[0];
    let record_id = path[1];
    let table_name = TableName::from(&table_name);
    let context = Context::create()?;
    let window = window::get_window(&table_name, &context.windows);
    match window {
        Some(window) => {
            let dao = data_read::get_selected_record_detail(
                &context.em,
                &context.dm,
                &context.tables,
                &window,
                &record_id,
                ::PAGE_SIZE,
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
///
fn handle_has_many(_req: Request, path: &[&str]) -> Result<impl Serialize, ServiceError> {
    let table_name = path[0];
    let record_id = path[1];
    let has_many_table = path[2];
    let tail = &path[3..];
    let key_value: Vec<(&str, &str)> = tail.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();
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
    let filter = filter_str.map(|s| Filter::from_str(s));
    let sort = sort_str.map(|s| Sort::from_str(s));
    let context = Context::create()?;
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &context.windows);
    let has_many_table_name = TableName::from(&has_many_table);
    match window {
        Some(window) => {
            let main_table = data_read::get_main_table(window, &context.tables);
            assert!(main_table.is_some());
            let main_table = main_table.unwrap();
            let has_many_tab = tab::find_tab(&window.has_many_tabs, &has_many_table_name);
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
                        ::PAGE_SIZE,
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
///
fn handle_indirect(_req: Request, path: &[&str]) -> Result<impl Serialize, ServiceError> {
    let table_name = path[0];
    let record_id = path[1];
    let indirect_table = path[2];
    let tail = &path[3..];
    let key_value: Vec<(&str, &str)> = tail.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();
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
    let filter = filter_str.map(|s| Filter::from_str(s));
    let sort = sort_str.map(|s| Sort::from_str(s));
    let context = Context::create()?;

    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &context.windows);
    let indirect_table_name = TableName::from(&indirect_table);
    match window {
        Some(window) => {
            let main_table = data_read::get_main_table(window, &context.tables);
            assert!(main_table.is_some());
            let main_table = main_table.unwrap();

            let indirect_tab: Option<&(TableName, Tab)> = window
                .indirect_tabs
                .iter()
                .find(|&(_linker_table, tab)| tab.table_name == indirect_table_name);

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
                        ::PAGE_SIZE,
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
fn handle_lookup(_req: Request, path: &[&str]) -> Result<impl Serialize, ServiceError> {
    let table_name = path[0];
    let page: u32 = path[1].parse().unwrap();
    let context = Context::create()?;

    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &context.windows);
    match window {
        Some(window) => {
            let rows = data_read::get_lookup_data_of_tab(
                &context.em,
                &context.dm,
                &context.tables,
                &window.main_tab,
                ::PAGE_SIZE,
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
fn handle_lookup_all(_req: Request, path: &[&str]) -> Result<impl Serialize, ServiceError> {
    let table_name = path[0];
    let context = Context::create()?;
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &context.windows);
    match window {
        Some(window) => {
            let lookup = data_read::get_all_lookup_for_window(
                &context.em,
                &context.dm,
                &context.tables,
                &window,
                ::PAGE_SIZE,
            )?;
            Ok(lookup)
        }
        None => Err(ServiceError::NotFound),
    }
}

// https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request?rq=1
// https://hyper.rs/guides/server/echo/
fn handle_delete(req: Request, path: &[&str]) -> Box<Future<Item = Response, Error = Error>> {
    let table_name = path[0].to_string();
    let f = req.body().concat2().map(move |chunk| {
        let body = chunk.into_iter().collect::<Vec<u8>>();
        let body_str = String::from_utf8(body.clone()).unwrap();
        let record_ids: Vec<String> = serde_json::from_str(&body_str).unwrap();
        let result = delete_records(&table_name, &record_ids);
        create_response(result)
    });
    Box::new(f)
}

fn handle_record_changeset(
    req: Request,
    path: &[&str],
) -> Box<Future<Item = Response, Error = Error>> {
    let table_name = path[0].to_string();
    let f = req.body().concat2().map(move |chunk| {
        let body = chunk.into_iter().collect::<Vec<u8>>();
        let body_str = String::from_utf8(body).unwrap();
        let changeset: Result<RecordChangeset, _> = serde_json::from_str(&body_str);
        let changeset = changeset.expect(&format!("unable to serialize from json {}", body_str));
        let result = update_record_changeset(&table_name, &changeset);
        create_response(result)
    });
    Box::new(f)
}

fn handle_tab_changeset(req: Request) -> Box<Future<Item = Response, Error = Error>> {
    let f = req.body().concat2().map(move |chunk| {
        let body = chunk.into_iter().collect::<Vec<u8>>();
        let body_str = String::from_utf8(body).unwrap();
        let container: SaveContainer = serde_json::from_str(&body_str).unwrap();
        let result = update_tab_changeset(&container);
        create_response(result)
    });
    Box::new(f)
}

fn delete_records(table_name: &str, record_ids: &Vec<String>) -> Result<Rows, ServiceError> {
    let context = Context::create()?;
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
            let rows = data_modify::delete_records(&context.dm, &main_table, &record_ids)?;
            Ok(rows)
        }
        None => Err(ServiceError::NotFound),
    }
}

fn update_tab_changeset(container: &SaveContainer) -> Result<(), ServiceError> {
    let context = Context::create()?;
    data_modify::save_container(&context.dm, &context.tables, &container)?;
    Ok(())
}

fn update_record_changeset(
    table_name: &str,
    changeset: &RecordChangeset,
) -> Result<(), ServiceError> {
    let context = Context::create()?;
    let table_name = TableName::from(&table_name);
    let window = window::get_window(&table_name, &context.windows);
    match window {
        Some(window) => {
            let table = data_read::get_main_table(window, &context.tables);
            assert!(table.is_some());
            let table = table.unwrap();
            let detail = data_modify::save_changeset(
                &context.dm,
                &context.tables,
                window,
                &table,
                changeset,
            )?;
            Ok(detail)
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
            let mut resp = Response::new().with_headers(headers).with_body(json);
            resp
        }
        Err(e) => {
            eprintln!("\n\nWarning an error response: {:?}", e);
            match e {
                ServiceError::NotFound => Response::new().with_status(StatusCode::NotFound),
                _ => Response::new().with_status(StatusCode::BadRequest),
            }
        }
    }
}

pub fn run(ip: &str, port: u16) -> Result<(), ServiceError> {
    let addr = match format!("{}:{}", ip, port).parse(){
        Ok(addr) => Ok(addr),
        Err(e) => Err(ServiceError::GenericError(format!("{}",e)))
    };
    let server = Server::new();
    let instance = Instance::new(server);
    let http = Http::new()
        .bind(&addr?, move || Ok(instance.clone()));

    let bind = match http{
        Ok(http) => http,
        Err(e) => {return Err(ServiceError::GenericError(format!("{}",e)));}
    };
    match bind.run(){
        Ok(bind) => bind,
        Err(e) => {return Err(ServiceError::GenericError(format!("{}", e)));}
    };
    Ok(())
}
