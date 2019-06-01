use crate::{
    credentials::Credentials,
    error::ServiceError,
    global,
    session,
};
use actix_web::{
    web,
    HttpRequest,
    HttpResponse,
    Error,
};
use diwata_intel::{window,data_read};
use futures::future::Future;
use serde::{
    Deserialize,
    Serialize,
};
use std::convert::TryFrom;
use diwata_intel::error::IntelError;

fn require_credentials(req: &HttpRequest) -> Result<(), ServiceError> {
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


pub fn windows(
    req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = Error> {
    require_credentials(&req).expect("Should have credentials");
    let credentials: Result<Credentials, ServiceError> =
        TryFrom::try_from(&req);
    let context = session::create_context(credentials).expect("unable to create context");
    let is_login_required = global::is_login_required().unwrap();
    let db_url = if is_login_required {
        global::get_role_db_url().unwrap()
    } else {
        global::get_db_url().unwrap()
    };

    web::block(move || window::get_grouped_windows_using_cache(&context.em, &db_url).map_err(|err|ServiceError::IntelError(err)))
        .from_err()
        .then(move |rows| {
            match rows {
                Ok(rows) => Ok(HttpResponse::Ok().body(ron::ser::to_string(&rows).expect("unable to serialize to ron"))),
                Err(e) => Err(e),
            }
        })
}

pub fn sql(
    req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = Error> {
    require_credentials(&req).expect("Should have credentials");
    let credentials: Result<Credentials, ServiceError> =
        TryFrom::try_from(&req);
    let context = session::create_context(credentials).expect("unable to create context");
    let query = req.query_string();
    #[derive(Debug, Deserialize)]
    struct SqlQuery {
        sql: String,
    }
    let fields: SqlQuery = serde_urlencoded::from_str(query)
        .map_err(|e| ServiceError::GenericError(e.to_string())).expect("Unable to get sql query");

    web::block(move || data_read::execute_sql_query(&context, fields.sql))
        .from_err()
        .then(move |rows| {
            match rows {
                Ok(rows) => Ok(HttpResponse::Ok().body(ron::ser::to_string(&rows).expect("unable to serialize to ron"))),
                Err(e) => Err(e),
            }
        })
}
