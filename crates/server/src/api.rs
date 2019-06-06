use crate::{
    credentials::Credentials,
    error::ServiceError,
    global,
    session,
};
use actix_web::{
    web,
    Error,
    HttpRequest,
    HttpResponse,
};
use diwata_intel::{
    data_read,
    Dao,
    TableName,
};
use futures::future::Future;
use serde::{
    Deserialize,
};
use std::convert::TryFrom;

pub fn require_credentials(req: &HttpRequest) -> Result<(), ServiceError> {
    let is_required = global::is_login_required().unwrap();

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

// FIXME: learn how to do custom error in actix
pub fn windows(
    req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = Error> {
    require_credentials(&req).expect("Should have credentials");
    let credentials: Result<Credentials, ServiceError> =
        TryFrom::try_from(&req);

    web::block(move || {
        let context = session::create_context(credentials);
        context.map(|context| context.grouped_window)
    })
    .from_err()
    .then(move |rows| {
        match rows {
            Ok(rows) => {
                Ok(HttpResponse::Ok().body(
                    ron::ser::to_string(&rows)
                        .expect("unable to serialize to ron"),
                ))
            }
            Err(e) => Err(e),
        }
    })
}

#[derive(Deserialize)]
pub struct SqlParam {
    sql: String,
}

// FIXME: learn how to do custom error in actix
pub fn sql(
    req: HttpRequest,
    sql_param: web::Query<SqlParam>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    require_credentials(&req).expect("Should have credentials");
    let credentials: Result<Credentials, ServiceError> =
        TryFrom::try_from(&req);

    web::block(move || {
        let context = session::create_context(credentials)
            .expect("unable to create context");
        data_read::execute_sql_query(&context, &sql_param.sql)
    })
    .from_err()
    .then(move |rows| {
        match rows {
            Ok(rows) => {
                Ok(HttpResponse::Ok().body(
                    ron::ser::to_string(&rows)
                        .expect("unable to serialize to ron"),
                ))
            }
            Err(e) => Err(e),
        }
    })
}

#[derive(Debug, Deserialize)]
pub struct DaoParam {
    dao: String,
}

pub fn record_detail(
    req: HttpRequest,
    table_name_param: web::Path<(String)>,
    dao_param: web::Query<DaoParam>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    require_credentials(&req).expect("Should have credentials");
    let credentials: Result<Credentials, ServiceError> =
        TryFrom::try_from(&req);

    web::block(move || {
        let context = session::create_context(credentials)
            .expect("unable to create context");
        let table_name = TableName::from(&table_name_param.to_string());
        let dao: Dao = ron::de::from_str(&dao_param.dao)
            .expect("Unable to deserialize dao");
        let detail = data_read::fetch_detail(&context, &table_name, &dao);
        println!("detail: {:#?}", detail);
        detail
    })
    .from_err()
    .then(move |record_detail| {
        match record_detail {
            Ok(record_detail) => {
                Ok(HttpResponse::Ok().body(
                    ron::ser::to_string(&record_detail)
                        .expect("unable to serialize to ron"),
                ))
            }
            Err(e) => Err(e),
        }
    })
}

pub fn main_data(
    req: HttpRequest,
    table_name_param: web::Path<(String)>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    require_credentials(&req).expect("Should have credentials");
    let credentials: Result<Credentials, ServiceError> =
        TryFrom::try_from(&req);

    web::block(move || {
        let context = session::create_context(credentials)
            .expect("unable to create context");
        let table_name = TableName::from(&table_name_param.to_string());
        let res = data_read::get_window_main_table_data(&context, &table_name);
        res
    })
    .from_err()
    .then(move |res| {
        match res {
            Ok(res) => {
                Ok(HttpResponse::Ok().body(
                    ron::ser::to_string(&res)
                        .expect("unable to serialize to ron"),
                ))
            }
            Err(e) => Err(e),
        }
    })
}
