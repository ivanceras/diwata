use crate::{assets, widgets};
use data_table::DataColumn;
use sauron::{
    html::{attributes::*, events::*, *},
    Component, Node,
};

#[derive(Clone)]
pub enum Msg {
    ChangeSearch(String),
}

pub struct ColumnView {
    column: DataColumn,
    width: i32,
    height: i32,
}

impl ColumnView {
    pub fn new(column: DataColumn) -> Self {
        ColumnView {
            column,
            width: 210,
            height: 20,
        }
    }
}

impl Component<Msg> for ColumnView {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::ChangeSearch(search) => {
                sauron::log!("Search term change: {}", search);
            }
        }
    }

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
                widgets::search_widget::new(
                    self.width,
                    self.height,
                    oninput(|input| Msg::ChangeSearch(input.value)),
                ),
            ],
        )
    }
}
