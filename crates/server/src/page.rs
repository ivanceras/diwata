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
use rustorm::{
    DaoManager,
    EntityManager,
};
use sauron::{
    html::{
        attributes::*,
        *,
    },
    html_extra,
    Node,
};
use std::convert::TryFrom;

fn get_index_html(
    context: &Context,
    em: &mut EntityManager,
    dm: &mut DaoManager,
    table_name: Option<TableName>,
) -> String {
    let app_data = data_read::retrieve_app_data(
        context,
        em,
        dm,
        table_name,
        global::PAGE_SIZE,
    )
    .expect("there should be app data");
    let app_data_serialized =
        ron::ser::to_string(&app_data).expect("unable to serialize to ron");
    let view: Node<()> = html_extra::html(
        vec![lang("en")],
        vec![
            head(
                vec![],
                vec![
                    meta(vec![charset("UTF-8")], vec![]),
                    meta(
                        vec![
                            name("viewport"),
                            content("width=device-width, initial-scale=1"),
                        ],
                        vec![],
                    ),
                    link(
                        vec![
                            rel("stylesheet"),
                            r#type("text/css"),
                            href("/webapp/style.css"),
                        ],
                        vec![],
                    ),
                    link(
                        vec![
                            rel("icon"),
                            r#type("image/png"),
                            href("/webapp/img/favicon-48x48.png"),
                            sizes("48x48"),
                        ],
                        vec![],
                    ),
                    html_extra::title(vec![], vec![text("Diwata")]),
                ],
            ),
            body(
                vec![style(
                    "margin: 0; padding: 0; width: 100%; height: 100%;",
                )],
                vec![
                    div(
                        vec![
                            id("web-app"),
                            style(
                                "width: 100%; height: 100%; text-align:center",
                            ),
                        ],
                        vec![img(
                            vec![
                                src("/webapp/img/loading.svg"),
                                style("margin: auto auto;"),
                            ],
                            vec![],
                        )],
                    ),
                    script(vec![src("/webapp/pkg/webapp.js")], vec![]),
                    script(
                        vec![r#type("module")],
                        vec![text(
                            "const { initialize } = wasm_bindgen;
        async function run() {
            await wasm_bindgen('/webapp/pkg/webapp_bg.wasm');
            initialize(window.initial_state);
        }
        run();",
                        )],
                    ),
                    script(
                        vec![],
                        vec![text(format!(
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

    let credentials = credentials.ok();

    let context = session::create_context(credentials.clone())
        .expect("unable to create context");
    let (mut em, mut dm) =
        crate::session::get_em_dm(credentials).expect("must not error");
    let index_html = get_index_html(&context, &mut em, &mut dm, None);
    future::ok(
        HttpResponse::Ok()
            .content_type("text/html")
            .body(index_html),
    )
}

pub fn index_with_table(
    req: HttpRequest,
    table_name_param: web::Path<String>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    println!("{:?}", req);
    api::require_credentials(&req).expect("Should have credentials");
    let credentials: Result<Credentials, ServiceError> =
        TryFrom::try_from(&req);

    let credentials = credentials.ok();
    let context = session::create_context(credentials.clone())
        .expect("unable to create context");
    let (mut em, mut dm) =
        crate::session::get_em_dm(credentials).expect("must not error");
    let table_name_str = table_name_param.to_string();
    let table_name = if !table_name_str.is_empty() {
        Some(TableName::from(&table_name_str))
    } else {
        println!("There is no table name specified!");
        None
    };
    let index_html = get_index_html(&context, &mut em, &mut dm, table_name);
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
