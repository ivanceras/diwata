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
    frozen_fields: Vec<usize>,
}

impl RowView {
    pub fn new(data_rows: DataRow) -> Self {
        RowView {
            fields: data_rows.into_iter().map(FieldView::new).collect(),
            frozen_fields: vec![],
        }
    }

    pub fn freeze_columns(&mut self, columns: Vec<usize>) {
        self.frozen_fields = columns;
    }

    pub fn view_frozen(&self) -> Node<Msg> {
        li(
            [class("row")],
            self.fields
                .iter()
                .enumerate()
                .filter(|(index, _field)| self.frozen_fields.contains(index))
                .map(|(index, field)| {
                    field
                        .view()
                        .map(move |field_msg| Msg::FieldMsg(index, field_msg))
                })
                .collect::<Vec<Node<Msg>>>(),
        )
    }
}

impl Component<Msg> for RowView {
    fn update(&mut self, _msg: Msg) {}

    fn view(&self) -> Node<Msg> {
        li(
            [class("row")],
            self.fields
                .iter()
                .enumerate()
                .filter(|(index, _field)| !self.frozen_fields.contains(index))
                .map(|(index, field)| {
                    field
                        .view()
                        .map(move |field_msg| Msg::FieldMsg(index, field_msg))
                })
                .collect::<Vec<Node<Msg>>>(),
        )
    }
}
