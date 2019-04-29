use crate::app::field_view::{self, FieldView};
use data_table::DataRow;
use sauron::{
    html::{attributes::*, *},
    Component, Node,
};

#[derive(Clone)]
pub enum Msg {
    FieldMsg(usize, field_view::Msg),
}

pub struct RowView {
    fields: Vec<FieldView>,
}

impl RowView {
    pub fn new(data_rows: DataRow) -> Self {
        RowView {
            fields: data_rows.into_iter().map(FieldView::new).collect(),
        }
    }
}

impl Component<Msg> for RowView {
    fn update(&mut self, msg: Msg) {}

    fn view(&self) -> Node<Msg> {
        li(
            [class("row")],
            self.fields
                .iter()
                .enumerate()
                .map(|(index, field)| {
                    field
                        .view()
                        .map(move |field_msg| Msg::FieldMsg(index, field_msg))
                })
                .collect::<Vec<Node<Msg>>>(),
        )
    }
}
