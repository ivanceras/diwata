use data_table::DataColumn;
use sauron::{
    html::{attributes::*, *},
    Component, Node,
};

#[derive(Clone)]
pub enum Msg {}

pub struct ColumnView {
    column: DataColumn,
    width: u32,
}

impl ColumnView {
    pub fn new(column: DataColumn) -> Self {
        ColumnView { column, width: 100 }
    }
}

impl Component<Msg> for ColumnView {
    fn update(&mut self, _msg: Msg) {}

    fn view(&self) -> Node<Msg> {
        button(
            [
                class("column_name"),
                styles([("width", format!("{}px", self.width))]),
            ],
            [text(&self.column.name)],
        )
    }
}
