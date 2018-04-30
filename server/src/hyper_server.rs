///
/// Derive from:
/// https://github.com/nrc/cargo-src/blob/master/src/server.rs
///
use futures;
use futures::Future;
use hyper::error::Error as HyperError;
use hyper::header::ContentType;
use hyper::server::Request;
use hyper::server::Response;
use hyper::server::Service;
use hyper::Headers;
use hyper::StatusCode;

use error::ServiceError;
use hyper::server::Http;
use intel::data_read;
use intel::error::IntelError;
use intel::tab::{self, Tab};
use intel::window;
use rustorm::Pool;
use rustorm::TableName;
use serde::Serialize;
use serde_json;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::{self, Command};
use std::str::FromStr;
use std::sync::{atomic::{AtomicU32, Ordering},
                Arc,
                Mutex};
use std::thread;

use rustorm::pool;

use intel::data_container::{Filter, Lookup, Sort};

static PAGE_SIZE: u32 = 40;

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
    type Error = HyperError;
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
    fn new() -> Self {
        Server {}
    }
    fn route(
        &self,
        mut path: &str,
        query: Option<&str>,
        req: Request,
    ) -> <Instance as Service>::Future {
        ::set_db_url("postgres://postgres:p0stgr3s@localhost/sakila".to_string());
        trace!("route: path: {:?}, query: {:?}", path, query);

        path = path.trim_matches('/');
        let path: Vec<_> = path.split('/').collect();
        let head = path[0];
        let tail = &path[1..];

        let result = if head == "" {
            self.handle_index(req)
        } else if head == "static" {
            self.handle_static(req, tail)
        } else if head == "windows" {
            create_response(self.handle_windows(req))
        } else if head == "window" {
            create_response(self.handle_window(req, tail))
        } else if head == "data" {
            create_response(self.handle_data(req, tail))
        } else if head == "select" {
            create_response(self.handle_select(req, tail))
        } else if head == "has_many_select" {
            create_response(self.handle_has_many(req, tail))
        } else if head == "indirect_select" {
            create_response(self.handle_indirect(req, tail))
        } else if head == "lookup" {
            create_response(self.handle_lookup(req, tail))
        } else if head == "lookup_all" {
            create_response(self.handle_lookup_all(req, tail))
        } else if head == "test" {
            create_response(self.handle_test(req))
        } else if head == "db_url" {
            create_response(self.handle_db_url(req))
        } else {
            self.handle_error(req, StatusCode::NotFound, "Page not found".to_owned())
        };

        Box::new(futures::future::ok(result))
    }
    fn handle_test(&self, _req: Request) -> Result<(), ServiceError> {
        let db_url = &::get_db_url()?;
        let ret = pool::test_connection(&db_url)?;
        Ok(ret)
    }

    fn handle_db_url(&self, _req: Request) -> Result<String, ServiceError> {
        match ::get_db_url_value() {
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

    fn handle_index(&self, _req: Request) -> Response {
        self.handle_static(_req, &["index.html"])
    }

    fn handle_static(&self, req: Request, path: &[&str]) -> Response {
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
            let mut file = File::open(&path_buf).unwrap();
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

        trace!("404 {:?}", path_buf);
        self.handle_error(req, StatusCode::NotFound, "Page not found".to_owned())
    }
    fn handle_error(&self, _req: Request, status: StatusCode, msg: String) -> Response {
        debug!("ERROR: {} ({})", msg, status);

        Response::new().with_status(status).with_body(msg)
    }

    fn handle_windows(&self, _req: Request) -> Result<impl Serialize, ServiceError> {
        let em = ::get_pool_em()?;
        let db_url = &::get_db_url()?;
        let ret = window::get_grouped_windows_using_cache(&em, db_url)?;
        Ok(ret)
    }

    fn handle_window(&self, _req: Request, tail: &[&str]) -> Result<impl Serialize, ServiceError> {
        let table_name = &tail[0];
        let em = ::get_pool_em()?;
        let db_url = &::get_db_url()?;
        let mut cache_pool = ::cache::CACHE_POOL.lock().unwrap();
        println!("{:#?}", db_url);
        let windows = cache_pool.get_cached_windows(&em, db_url)?;
        let table_name = TableName::from(&table_name);
        let window = window::get_window(&table_name, &windows);
        match window {
            Some(window) => Ok(window.to_owned()),
            None => Err(ServiceError::NotFound),
        }
    }

    ///
    /// /data/<table_name>/page/<page>/filter/<filter>/sort/<sort>/
    ///
    fn handle_data(&self, _req: Request, path: &[&str]) -> Result<impl Serialize, ServiceError> {
        println!("path:{:?}", path);
        let table_name = path[0];
        let tail = &path[1..];
        println!("tail {:?}", tail);
        let key_value: Vec<(&str, &str)> =
            tail.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();
        let mut page = 1;
        let mut filter_str = None;
        let mut sort_str = None;
        for (k, v) in key_value {
            println!("{} = {}", k, v);
            if k == "page" {
                page = v.parse().unwrap();
            } else if k == "filter" {
                filter_str = Some(v);
            } else if k == "sort" {
                sort_str = Some(v);
            }
        }

        let em = ::get_pool_em()?;
        let dm = ::get_pool_dm()?;
        let db_url = &::get_db_url()?;
        let mut cache_pool = ::cache::CACHE_POOL.lock().unwrap();
        let windows = cache_pool.get_cached_windows(&em, db_url)?;
        let table_name = TableName::from(&table_name);
        let window = window::get_window(&table_name, &windows);
        let tables = cache_pool.get_cached_tables(&em, db_url)?;
        let filter = filter_str.map(|s| Filter::from_str(s));
        let sort = sort_str.map(|s| Sort::from_str(s));
        match window {
            Some(window) => {
                let rows = data_read::get_maintable_data(
                    &em, &dm, &tables, &window, filter, sort, page, PAGE_SIZE,
                )?;
                Ok(rows)
            }
            None => Err(ServiceError::NotFound),
        }
    }

    ///
    /// /select/<table_name>/<record_id>
    ///
    fn handle_select(&self, _req: Request, path: &[&str]) -> Result<impl Serialize, ServiceError> {
        let table_name = path[0];
        let record_id = path[1];
        let dm = ::get_pool_dm()?;
        let em = ::get_pool_em()?;
        let db_url = &::get_db_url()?;
        let mut cache_pool = ::cache::CACHE_POOL.lock().unwrap();
        let windows = cache_pool.get_cached_windows(&em, db_url)?;
        let table_name = TableName::from(&table_name);
        let window = window::get_window(&table_name, &windows);
        let tables = cache_pool.get_cached_tables(&em, db_url)?;
        match window {
            Some(window) => {
                let dao = data_read::get_selected_record_detail(
                    &dm, &tables, &window, &record_id, PAGE_SIZE,
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
    fn handle_has_many(
        &self,
        _req: Request,
        path: &[&str],
    ) -> Result<impl Serialize, ServiceError> {
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
            println!("{} = {}", k, v);
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

        let dm = ::get_pool_dm()?;
        let em = ::get_pool_em()?;
        let db_url = &::get_db_url()?;
        let mut cache_pool = ::cache::CACHE_POOL.lock().unwrap();
        let windows = cache_pool.get_cached_windows(&em, db_url)?;
        let table_name = TableName::from(&table_name);
        let window = window::get_window(&table_name, &windows);
        let tables = cache_pool.get_cached_tables(&em, db_url)?;
        let has_many_table_name = TableName::from(&has_many_table);
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
    fn handle_indirect(
        &self,
        _req: Request,
        path: &[&str],
    ) -> Result<impl Serialize, ServiceError> {
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
            println!("{} = {}", k, v);
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

        let dm = ::get_pool_dm()?;
        let em = ::get_pool_em()?;
        let db_url = &::get_db_url()?;
        let mut cache_pool = ::cache::CACHE_POOL.lock().unwrap();
        let windows = cache_pool.get_cached_windows(&em, db_url)?;
        let table_name = TableName::from(&table_name);
        let window = window::get_window(&table_name, &windows);
        let tables = cache_pool.get_cached_tables(&em, db_url)?;
        let indirect_table_name = TableName::from(&indirect_table);
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
                        Ok(rows)
                    }
                    None => Err(ServiceError::NotFound),
                }
            }
            None => Err(ServiceError::NotFound),
        }
    }

    fn handle_lookup(&self, _req: Request, path: &[&str]) -> Result<impl Serialize, ServiceError> {
        println!("path:{:?}", path);
        let table_name = path[0];
        let page: u32 = path[1].parse().unwrap();

        let dm = ::get_pool_dm()?;
        let em = ::get_pool_em()?;
        let db_url = &::get_db_url()?;
        let mut cache_pool = ::cache::CACHE_POOL.lock().unwrap();
        let windows = cache_pool.get_cached_windows(&em, db_url)?;
        let table_name = TableName::from(&table_name);
        let window = window::get_window(&table_name, &windows);
        let tables = cache_pool.get_cached_tables(&em, db_url)?;
        match window {
            Some(window) => {
                let rows = data_read::get_lookup_data_of_tab(
                    &dm,
                    &tables,
                    &window.main_tab,
                    PAGE_SIZE,
                    page,
                )?;
                Ok(rows)
            }
            None => Err(ServiceError::NotFound),
        }
    }
    fn handle_lookup_all(
        &self,
        _req: Request,
        path: &[&str],
    ) -> Result<impl Serialize, ServiceError> {
        let table_name = path[0];
        let dm = ::get_pool_dm()?;
        let em = ::get_pool_em()?;
        let db_url = &::get_db_url()?;
        let mut cache_pool = ::cache::CACHE_POOL.lock().unwrap();
        let windows = cache_pool.get_cached_windows(&em, db_url)?;
        let table_name = TableName::from(&table_name);
        let window = window::get_window(&table_name, &windows);
        let tables = cache_pool.get_cached_tables(&em, db_url)?;
        match window {
            Some(window) => {
                let lookup =
                    data_read::get_all_lookup_for_window(&dm, &tables, &window, PAGE_SIZE)?;
                Ok(lookup)
            }
            None => Err(ServiceError::NotFound),
        }
    }
}

fn create_response<B: Serialize>(body: Result<B, ServiceError>) -> Response {
    match body {
        Ok(body) => {
            let json = serde_json::to_string(&body).unwrap();
            println!("json:{}", json);
            let mut headers = Headers::new();
            headers.set(ContentType::json());
            let mut resp = Response::new().with_headers(headers).with_body(json);
            resp
        }
        Err(e) => Response::new().with_status(StatusCode::BadRequest),
    }
}

pub fn run() {
    let ip = "0.0.0.0";
    let port = 8001;
    let addr = format!("{}:{}", ip, port).parse().unwrap();
    let server = Server::new();
    let instance = Instance::new(server);
    Http::new()
        .bind(&addr, move || Ok(instance.clone()))
        .unwrap()
        .run()
        .unwrap();
}
