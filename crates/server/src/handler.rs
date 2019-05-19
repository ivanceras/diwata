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
    header::{
        AccessControlAllowOrigin,
        ContentType,
    },
    server::{
        Response,
        Service,
    },
    Headers,
    Request,
    StatusCode,
};

use crate::{
    credentials::Credentials,
    error::ServiceError,
    global,
    session,
};
use diwata_intel::{
    data_container::{
        RecordChangeset,
        SaveContainer,
    },
    data_modify,
    data_read,
    tab,
    window,
    IndirectTab,
};
use hyper::server::Http;
use log::*;
use ron;
use rustorm::{
    Rows,
    TableName,
};
use serde::{
    Deserialize,
    Serialize,
};
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
    } else if head == "sql" {
        create_response(handle_sql_query(req, tail))
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
    let mut headers = Headers::new();
    headers.set(AccessControlAllowOrigin::Any);
    Response::new()
        .with_headers(headers)
        .with_status(status)
        .with_body(msg)
}

fn handle_windows(req: Request) -> Result<impl Serialize, ServiceError> {
    require_credentials(&req)?;
    let credentials: Result<Credentials, ServiceError> =
        TryFrom::try_from(&req);
    let context = session::create_context(credentials)?;
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

/// /sql?sql=query
fn handle_sql_query(
    req: Request,
    _path: &[&str],
) -> Result<impl Serialize, ServiceError> {
    require_credentials(&req)?;
    let credentials: Result<Credentials, ServiceError> =
        TryFrom::try_from(&req);
    let context = session::create_context(credentials)?;
    let query = req.query().expect("Expecting a query");
    #[derive(Debug, Deserialize)]
    struct SqlQuery {
        sql: String,
    }
    let fields: SqlQuery = serde_urlencoded::from_str(query)
        .map_err(|e| ServiceError::GenericError(e.to_string()))?;
    println!("fields: {:#?}", fields);
    let rows = data_read::execute_sql_query(
        &context,
        fields.sql,
    )?;
    Ok(rows)
}

fn is_credentials_ok(req: &Request) -> bool {
    match require_credentials(&req) {
        Ok(()) => true,
        Err(_) => false,
    }
}

fn create_response<B: Serialize>(body: Result<B, ServiceError>) -> Response {
    match body {
        Ok(body) => {
            //let json = serde_json::to_string(&body).unwrap();
            let json =
                ron::ser::to_string(&body).expect("unable to serialize to ron");
            let mut headers = Headers::new();
            headers.set(ContentType::text());
            headers.set(AccessControlAllowOrigin::Any);
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
