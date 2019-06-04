use crate::{
    credentials::Credentials,
    error::ServiceError,
    global,
    session,
    api,
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
use diwata_intel::{data_read,Context};
use futures::future::{
    self,
    Future,
};
use sauron::{
    html::{
        attributes::*,
        *,
    },
    html_extra,
    Node,
};
use serde::{
    Deserialize,
    Serialize,
};
use std::convert::TryFrom;
use ron;

fn get_index_html(context: &Context) -> String {
    let app_data = data_read::retrieve_app_data(context)
        .expect("there should be app data");
    let app_data_serialized =
        ron::ser::to_string(&app_data).expect("unable to serialize to ron");
    let view: Node<()> = html_extra::html(
        [lang("en")],
        [
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
                    html_extra::title([], [text("Diwata")]),
                ],
            ),
            body(
                [style("margin: 0; padding: 0; width: 100%; height: 100%;")],
                [
                    div(
                        [id("web-app"), style("width: 100%; height: 100%;")],
                        [text("#HTML_INSERTED_HERE_BY_SERVER#")],
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
                            "window.initial_state = '{}'",
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
    let index_html = get_index_html(&context);
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
