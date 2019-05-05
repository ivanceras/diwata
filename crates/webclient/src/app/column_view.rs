use crate::assets;
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
        ColumnView { column, width: 210 }
    }
}

impl Component<Msg> for ColumnView {
    fn update(&mut self, _msg: Msg) {}

    fn view(&self) -> Node<Msg> {
        div(
            [class("column_name_and_control")],
            [
                div(
                    [class("column_name_and_sort")],
                    [
                        div([class("column_name")], [text(&self.column.name)]),
                        div([], [assets::sort_btn_asc(18, 18, "#888")]),
                    ],
                ),
                div(
                    [
                        class("filter_icon_and_column_filter"),
                        styles([("width", format!("{}px", self.width))]),
                    ],
                    [
                        div(
                            [class("filter_icon")],
                            [assets::svg_filter_icon(18, 18, "#888")],
                        ),
                        input([r#type("text"), class("column_filter")], []),
                    ],
                ),
            ],
        )
    }
}
