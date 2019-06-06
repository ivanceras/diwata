//#![deny(warnings)]
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
use rustorm::Pool;

use dotenv::dotenv;
extern crate diwata_server as server;
extern crate structopt;
#[macro_use]
extern crate log;
extern crate serde_derive;

mod api;
mod credentials;
mod error;
mod global;
mod page;
mod session;

fn main() -> io::Result<()> {
    dotenv().ok();

    env::set_var("RUST_LOG", "diwata_server=debug,actix_web=info");
    env_logger::init();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("DATABASE_URL: {}", database_url);
    let port = env::var("PORT").expect("PORT must be set");
    println!("PORT: {}", port);
    global::set_db_url(&database_url).expect("unable to set global db_url");
    let app = move || {
        debug!("Constructing the App");

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

    debug!("Starting server");
    HttpServer::new(app)
        .bind(format!("0.0.0.0:{}", port))?
        .run()
}
