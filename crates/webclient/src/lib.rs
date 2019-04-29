//#![deny(warnings)]
#![deny(clippy::all)]
use app::App;
use diwata_intel::{IndirectTab, Tab, TableName, Window};
use sauron::Program;
use wasm_bindgen::{self, prelude::*};

mod app;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn initialize(initial_state: &str) {
    sauron::log!("initial state: {}", initial_state);
    let root_node = sauron::document()
        .get_element_by_id("web-app")
        .expect("Unable to get hold of root-node");
    let windows: Vec<Window> = vec![
        make_sample_window("Window1"),
        make_sample_window("Window2"),
        make_sample_window("Window3"),
    ];
    Program::new_replace_mount(App::new(windows), &root_node);
}

fn make_sample_window(name: &str) -> Window {
    Window {
        name: name.to_string(),
        description: None,
        group: None,
        main_tab: make_sample_tab(&format!("Main tab of {}", name)),
        has_one_tabs: vec![make_sample_tab("Has one 1"), make_sample_tab("Has one 2")],
        one_one_tabs: vec![make_sample_tab("One one 1")],
        has_many_tabs: vec![make_sample_tab("Has many 1"), make_sample_tab("Has many 2")],
        indirect_tabs: vec![
            IndirectTab::new(
                TableName::from("bazaar.table1"),
                make_sample_tab("Indirect 1"),
            ),
            IndirectTab::new(
                TableName::from("bazaar.table2"),
                make_sample_tab("Indirect 2"),
            ),
        ],
        is_view: false,
    }
}

fn make_sample_tab(name: &str) -> Tab {
    Tab {
        name: name.to_string(),
        description: None,
        table_name: TableName::from("bazaar.product"),
        fields: vec![],
        is_view: false,
        display: None,
    }
}
