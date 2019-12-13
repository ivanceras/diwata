use crate::{assets, widgets};
use data_table::DataColumn;
use sauron::{
    html::{attributes::*, events::*, units::*, *},
    Cmd, Component, Node,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Msg {
    ChangeSearch(String),
}

pub struct ColumnView {
    pub column: DataColumn,
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
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::ChangeSearch(search) => {
                trace!("Search term change: {}", search);
                Cmd::none()
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        div(
            vec![class("column_name_and_control")],
            vec![
                div(
                    vec![
                        class("column_name_and_sort"),
                        styles(vec![("width", px(self.width)), ("height", px(self.height))]),
                    ],
                    vec![
                        div(vec![class("column_name")], vec![text(&self.column.name)]),
                        div(vec![], vec![assets::sort_btn_asc(18, 18, "#888")]),
                    ],
                ),
                div(
                    vec![class("column_name_search_widget_container")],
                    vec![widgets::search_widget(oninput(|input| {
                        Msg::ChangeSearch(input.value)
                    }))],
                ),
            ],
        )
    }
}
