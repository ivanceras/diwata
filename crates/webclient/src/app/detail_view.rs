use crate::app::field_view::FieldView;

use data_table::DataRow;
use sauron::{
    html::{attributes::*, events::*, *},
    Component, Node,
};

#[derive(Clone)]
pub enum Msg {
    Close,
}

/// When a record from the main tab is clicked, it will show the detailed view of that
/// row, displaying only that 1 row, and it's related content
/// such as one_one tab, has_many and indirect tab
pub struct DetailView {
    fields: Vec<FieldView>,
    pub is_visible: bool,
    pub row_index: Option<usize>,
}

impl DetailView {
    pub fn new() -> Self {
        DetailView {
            fields: vec![],
            is_visible: false,
            row_index: None,
        }
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
    }

    pub fn show(&mut self) {
        self.is_visible = true;
    }

    pub fn set_row(&mut self, row_index: usize) {
        self.row_index = Some(row_index);
    }
}

impl Component<Msg> for DetailView {
    fn update(&mut self, msg: Msg) {}

    fn view(&self) -> Node<Msg> {
        main(
            [
                class("detail_view"),
                styles_flag([
                    ("display", "flex", self.is_visible),
                    ("display", "none", !self.is_visible),
                ]),
            ],
            [
                text(format!("Detailed view {:?}", self.row_index)),
                button([onclick(|_| Msg::Close)], [text("Close")]),
            ],
        )
    }
}
