//#![deny(warnings)]
#![deny(clippy::all)]
use app::{App, Msg};
use diwata_intel::data_container::AppData;
use sauron::Program;
use std::rc::Rc;
#[macro_use]
extern crate log;

mod app;
mod assets;
mod rest_api;
mod widgets;

pub fn setup_program(initial_state: &str) -> Rc<Program<App, Msg>> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    trace!("initial state: {}", initial_state);
    let root_node = sauron::document()
        .get_element_by_id("web-app")
        .expect("Unable to get hold of root-node");
    let app_data: AppData =
        ron::de::from_str(initial_state).expect("unable to deserialize app_data");
    let app = make_app(app_data);
    Program::new_replace_mount(app, &root_node)
}

pub fn make_app(app_data: AppData) -> App {
    let (window_width, window_height) = get_window_size();
    let app = App::new(app_data, window_width, window_height);
    app
}

#[cfg(not(target_arch = "wasm32"))]
fn get_window_size() -> (i32, i32) {
    (800, 800)
}

#[cfg(target_arch = "wasm32")]
fn get_window_size() -> (i32, i32) {
    let window = sauron::window();
    let window_width = window
        .inner_width()
        .expect("unable to get window width")
        .as_f64()
        .expect("cant convert to f64");
    let window_height = window
        .inner_height()
        .expect("unable to get height")
        .as_f64()
        .expect("cant convert to f64");
    (window_width as i32, window_height as i32)
}
