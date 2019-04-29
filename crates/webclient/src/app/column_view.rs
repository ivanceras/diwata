use data_table::DataColumn;
use sauron::{
    html::{attributes::*, *},
    Component, Node,
};

#[derive(Clone)]
pub enum Msg {}

pub struct ColumnView {
    column: DataColumn,
}

impl ColumnView {
    pub fn new(column: DataColumn) -> Self {
        ColumnView { column }
    }
}

impl Component<Msg> for ColumnView {
    fn update(&mut self, msg: Msg) {}

    fn view(&self) -> Node<Msg> {
        button([class("column_name")], [text(&self.column.name)])
    }
}
