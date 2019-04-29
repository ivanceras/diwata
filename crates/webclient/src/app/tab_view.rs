use crate::app::{
    column_view::ColumnView,
    detail_view::DetailView,
    field_view::{self, FieldView},
    table_view::TableView,
};
use data_table::DataColumn;

use sauron::{
    html::{attributes::*, events::*, *},
    Component, Node,
};

use diwata_intel::Tab;

#[derive(Debug, Clone)]
pub enum Msg {
    FieldMsg(usize, field_view::Msg),
}

pub struct TabView {
    name: String,
    detail_view: Option<DetailView>,
    table_view: TableView,
}

impl TabView {
    pub fn new(tab: Tab) -> Self {
        TabView {
            name: tab.name.clone(),
            table_view: TableView::from_tab(tab),
            detail_view: None,
        }
    }
}

impl Component<Msg> for TabView {
    fn update(&mut self, msg: Msg) {}
    fn view(&self) -> Node<Msg> {
        div([], [div([], [button([], [text(&self.name)])])])
    }
}
