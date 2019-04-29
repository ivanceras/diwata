//#![deny(warnings)]
#![deny(clippy::all)]
use app::App;
use diwata_intel::{
    field::ColumnDetail,
    widget::{Alignment, ControlWidget, Widget},
    ColumnName, Field, IndirectTab, SqlType, Tab, TableName, Window,
};
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
        sample_window("Window1"),
        sample_window("Window2"),
        sample_window("Window3"),
    ];
    Program::new_replace_mount(App::new(windows), &root_node);
}

fn sample_window(name: &str) -> Window {
    Window {
        name: name.to_string(),
        description: None,
        group: None,
        main_tab: sample_tab(&format!("Main tab of {}", name)),
        has_one_tabs: vec![sample_tab("Has one 1"), sample_tab("Has one 2")],
        one_one_tabs: vec![sample_tab("One one 1")],
        has_many_tabs: vec![sample_tab("Has many 1"), sample_tab("Has many 2")],
        indirect_tabs: vec![
            IndirectTab::new(TableName::from("bazaar.table1"), sample_tab("Indirect 1")),
            IndirectTab::new(TableName::from("bazaar.table2"), sample_tab("Indirect 2")),
        ],
        is_view: false,
    }
}

fn sample_tab(name: &str) -> Tab {
    Tab {
        name: name.to_string(),
        description: None,
        table_name: TableName::from("bazaar.product"),
        fields: (0..10)
            .into_iter()
            .map(|n| sample_field(&format!("Field {}", n)))
            .collect(),
        is_view: false,
        display: None,
    }
}

fn sample_field(name: &str) -> Field {
    Field {
        name: name.to_string(),
        description: None,
        info: None,
        is_primary: false,
        column_detail: sample_column_detail(name),
        control_widget: sample_control_widget(name),
    }
}

fn sample_column_detail(name: &str) -> ColumnDetail {
    ColumnDetail::Simple(ColumnName::from(name), SqlType::Text)
}

fn sample_control_widget(name: &str) -> ControlWidget {
    ControlWidget {
        widget: Widget::Textbox,
        dropdown: None,
        width: 100,
        max_len: Some(100),
        height: 20,
        alignment: Alignment::Left,
    }
}
