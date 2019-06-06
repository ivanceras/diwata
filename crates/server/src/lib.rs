#![deny(warnings)]
#![deny(clippy::all)]

pub use error::ServiceError;
pub use global::{
    set_db_url,
    set_login_required,
};
use std::{
    env,
    io,
};

use actix_files as fs;
use actix_web::{
    http,
    middleware::{
        errhandlers::ErrorHandlers,
        Logger,
    },
    web,
    App,
    HttpServer,
};

use dotenv::dotenv;

mod api;
mod credentials;
pub mod error;
mod global;
mod page;
pub mod session;

pub fn start() -> io::Result<()> {
    dotenv().ok();

    env::set_var("RUST_LOG", "diwata_server=debug,actix_web=info");
    env_logger::init();

    let database_url: String =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("DATABASE_URL: {}", database_url);
    let port = env::var("PORT").expect("PORT must be set");
    println!("PORT: {}", port);
    global::set_db_url(&database_url).expect("unable to set global db_url");
    global::precache().expect("unable to precache");
    let app = move || {
        let error_handlers = ErrorHandlers::new()
            .handler(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                page::internal_server_error,
            )
            .handler(http::StatusCode::BAD_REQUEST, page::bad_request)
            .handler(http::StatusCode::NOT_FOUND, page::not_found);
        App::new()
            .wrap(Logger::default())
            .wrap(error_handlers)
            .service(web::resource("/").route(web::get().to_async(page::index)))
            .service(
                web::resource("/{table_name}")
                    .route(web::get().to_async(page::index_with_table)),
            )
            .service(
                web::resource("/sql/").route(web::get().to_async(api::sql)),
            )
            .service(
                web::resource("/windows")
                    .route(web::get().to_async(api::windows)),
            )
            .service(
                web::resource("/main_data/{table_name}/")
                    .route(web::get().to_async(api::main_data)),
            )
            .service(
                web::resource("/record_detail/{table_name}/")
                    .route(web::get().to_async(api::record_detail)),
            )
            .service(fs::Files::new("/webapp", "crates/webapp/"))
    };

    HttpServer::new(app)
        .bind(format!("0.0.0.0:{}", port))?
        .run()
}
