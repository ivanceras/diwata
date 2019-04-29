use crate::app::{
    column_view::ColumnView,
    detail_view::DetailView,
    field_view::{self, FieldView},
    table_view::{self, TableView},
};
use data_table::{DataColumn, DataRow};

use sauron::{
    html::{attributes::*, events::*, *},
    Component, Node,
};

use diwata_intel::Tab;

#[derive(Clone)]
pub enum Msg {
    TableMsg(table_view::Msg),
    FieldMsg(usize, field_view::Msg),
}

pub struct TabView {
    pub name: String,
    detail_view: Option<DetailView>,
    table_view: TableView,
    is_visible: bool,
}

impl TabView {
    pub fn new(tab: Tab) -> Self {
        TabView {
            name: tab.name.clone(),
            table_view: TableView::from_tab(tab),
            detail_view: None,
            is_visible: true,
        }
    }

    pub fn set_data_rows(&mut self, data_row: Vec<DataRow>) {
        self.table_view.set_data_rows(data_row)
    }

    pub fn show(&mut self) {
        self.is_visible = true;
    }
    pub fn hide(&mut self) {
        self.is_visible = false;
    }
}

impl Component<Msg> for TabView {
    fn update(&mut self, msg: Msg) {}
    fn view(&self) -> Node<Msg> {
        section(
            [
                class("tab"),
                styles_flag([
                    ("display", "block", self.is_visible),
                    ("display", "none", !self.is_visible),
                ]),
            ],
            [
                div([], [button([], [text(&self.name)])]),
                section([], [self.table_view.view().map(Msg::TableMsg)]),
            ],
        )
    }
}
