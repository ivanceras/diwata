//#![deny(warnings)]
#![deny(clippy::all)]
use sauron::*;
use console_error_panic_hook;
use wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys;

use app::App;
use app::store::Msg;

mod app;

pub type Node = sauron::Node<Msg>; 
pub type Element = sauron::Element<Msg>;
pub type Patch<'a> = sauron::Patch<'a,Msg>;
pub type Attribute<'a> = sauron::Attribute<'a,Msg>; 

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn initialize(initial_state: &str) {
    sauron::log!("initial state: {}", initial_state);
    let root_node = sauron::document().get_element_by_id("web-app").expect("Unable to get hold of root-node");
    let program = Program::new_replace_mount(App::new(), &root_node);
}
