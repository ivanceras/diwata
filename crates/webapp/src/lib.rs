#![deny(warnings)]
use wasm_bindgen::{
    self,
    prelude::*,
};
use webview;

#[wasm_bindgen]
pub fn initialize(initial_state: &str) {
    webview::setup_program(initial_state);
}
