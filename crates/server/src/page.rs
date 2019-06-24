use crate::{
    api,
    credentials::Credentials,
    error::ServiceError,
    global,
    session,
};
use actix_files::NamedFile;
use actix_web::{
    dev,
    middleware::errhandlers::ErrorHandlerResponse,
    web,
    Error,
    HttpRequest,
    HttpResponse,
    Responder,
    Result,
};
use diwata_intel::{
    data_read,
    Context,
    TableName,
};
use futures::future::{
    self,
    Future,
};
use ron;
use sauron::{
    html::{
        attributes::*,
    },
    html_array::*,
    html_extra,
    Node,
};

use std::convert::TryFrom;

fn get_index_html(context: &Context, table_name: Option<TableName>) -> String {
    let app_data =
        data_read::retrieve_app_data(context, table_name, global::PAGE_SIZE)
            .expect("there should be app data");
    let app_data_serialized =
        ron::ser::to_string(&app_data).expect("unable to serialize to ron");
    let view: Node<()> = html_extra::html(
        vec![lang("en")],
        vec![
            head(
                [],
                [
                    meta([charset("UTF-8")], []),
                    meta(
                        [
                            name("viewport"),
                            content("width=device-width, initial-scale=1"),
                        ],
                        [],
                    ),
                    link(
                        [
                            rel("stylesheet"),
                            r#type("text/css"),
                            href("/webapp/style.css"),
                        ],
                        [],
                    ),
                    link(
                        [
                            rel("icon"),
                            r#type("image/png"),
                            href("/webapp/img/favicon-48x48.png"),
                            sizes("48x48"),
                        ],
                        [],
                    ),
                    html_extra::title(vec![], vec![text("Diwata")]),
                ],
            ),
            body(
                [style("margin: 0; padding: 0; width: 100%; height: 100%;")],
                [
                    div(
                        [
                            id("web-app"),
                            style(
                                "width: 100%; height: 100%; text-align:center",
                            ),
                        ],
                        [img(
                            [
                                src("/webapp/img/loading.svg"),
                                style("margin: auto auto;"),
                            ],
                            [],
                        )],
                    ),
                    script([src("/webapp/pkg/webapp.js")], []),
                    script(
                        [r#type("module")],
                        [text(
                            "const { initialize } = wasm_bindgen;
        async function run() {
            await wasm_bindgen('/webapp/pkg/webapp_bg.wasm');
            initialize(window.initial_state);
        }
        run();",
                        )],
                    ),
                    script(
                        [],
                        [text(format!(
                            "window.initial_state = String.raw`{}`",
                            app_data_serialized
                        ))],
                    ),
                ],
            ),
        ],
    );
    view.to_string()
}

pub fn index(
    req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = Error> {
    println!("{:?}", req);
    api::require_credentials(&req).expect("Should have credentials");
    let credentials: Result<Credentials, ServiceError> =
        TryFrom::try_from(&req);
    let context =
        session::create_context(credentials).expect("unable to create context");
    let index_html = get_index_html(&context, None);
    future::ok(
        HttpResponse::Ok()
            .content_type("text/html")
            .body(index_html),
    )
}

pub fn index_with_table(
    req: HttpRequest,
    table_name_param: web::Path<(String)>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    println!("{:?}", req);
    api::require_credentials(&req).expect("Should have credentials");
    let credentials: Result<Credentials, ServiceError> =
        TryFrom::try_from(&req);
    let context =
        session::create_context(credentials).expect("unable to create context");
    let table_name_str = table_name_param.to_string();
    let table_name = if !table_name_str.is_empty() {
        Some(TableName::from(&table_name_str))
    } else {
        println!("There is no table name specified!");
        None
    };
    let index_html = get_index_html(&context, table_name);
    future::ok(
        HttpResponse::Ok()
            .content_type("text/html")
            .body(index_html),
    )
}

pub fn bad_request<B>(
    res: dev::ServiceResponse<B>,
) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("static/errors/400.html")?
        .set_status_code(res.status())
        .respond_to(res.request())?;
    Ok(ErrorHandlerResponse::Response(
        res.into_response(new_resp.into_body()),
    ))
}

pub fn not_found<B>(
    res: dev::ServiceResponse<B>,
) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("static/errors/404.html")?
        .set_status_code(res.status())
        .respond_to(res.request())?;
    Ok(ErrorHandlerResponse::Response(
        res.into_response(new_resp.into_body()),
    ))
}

pub fn internal_server_error<B>(
    res: dev::ServiceResponse<B>,
) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("static/errors/500.html")?
        .set_status_code(res.status())
        .respond_to(res.request())?;
    Ok(ErrorHandlerResponse::Response(
        res.into_response(new_resp.into_body()),
    ))
}
