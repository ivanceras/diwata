use crate::{
    app::{
        detail_view::DetailView,
        field_view,
        table_view::{self, TableView},
    },
    data::FrozenData,
};
use data_table::DataRow;

use sauron::{
    html::{attributes::*, *},
    Component, Node,
};

use diwata_intel::Tab;

#[derive(Clone)]
pub enum Msg {
    TableMsg(table_view::Msg),
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
        self.table_view.set_data_rows(data_row);
    }

    pub fn freeze_rows(&mut self, rows: Vec<usize>) {
        self.table_view.freeze_rows(rows);
    }

    pub fn freeze_columns(&mut self, columns: Vec<usize>) {
        sauron::log!("freeze columns: {:?}", columns);
        self.table_view.freeze_columns(columns);
    }
    pub fn set_frozen_data(&mut self, frozen_data: FrozenData) {
        self.freeze_rows(frozen_data.frozen_rows);
        self.freeze_columns(frozen_data.frozen_columns);
    }

    pub fn show(&mut self) {
        self.is_visible = true;
    }
    pub fn hide(&mut self) {
        self.is_visible = false;
    }
}

impl Component<Msg> for TabView {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::TableMsg(table_msg) => self.table_view.update(table_msg),
        }
    }
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
