//#![deny(warnings)]
#![deny(clippy::all)]
use browser::*;
use console_error_panic_hook;
use wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys;

use app::App;
use vdom::{Component, View, Widget};

mod app;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Client {
    app: App,
    dom_updater: DomUpdater,
}

// Expose globals from JS for things such as request animation frame
// that web sys doesn't seem to have yet
//
// TODO: Remove this and use RAF from Rust
// https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Window.html#method.request_animation_frame
#[wasm_bindgen]
extern "C" {
    pub type GlobalJS;

    pub static global_js: GlobalJS;

    #[wasm_bindgen(method)]
    pub fn update(this: &GlobalJS);
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new(initial_state: &str) -> Client {
        console_error_panic_hook::set_once();

        let root_node = document().get_element_by_id("web-app").unwrap();

        let app = App::new();

        let dom_updater = DomUpdater::new_replace_mount(app.view(), root_node);
        let mut client = Client { app, dom_updater };
        client.subscribe();
        client
    }

    /// set up the app.store
    /// whenever there is a changes to the store
    /// the app.update function will be called
    pub fn subscribe(&mut self) {
        self.app.subscribe(Box::new(|| {
            global_js.update();
        }));
    }

    /// called from the js side
    /// When global_js.update is called
    /// it will call these render function in an animation frame
    pub fn render(&mut self) {
        self.app.update();
        let vdom = self.app.view();
        self.dom_updater.update(vdom);
    }
}
