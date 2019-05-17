use crate::app::field_view::{self, FieldView};
use data_table::{DataColumn, DataRow};
use sauron::{
    html::{attributes::*, events::*, *},
    Component, Node,
};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub enum Msg {
    FieldMsg(usize, field_view::Msg),
    DoubleClick,
    Click,
}

pub struct RowView {
    pub fields: Vec<Rc<RefCell<FieldView>>>,
    frozen_fields: Vec<usize>,
    is_frozen: bool,
}

impl RowView {
    pub fn new(data_rows: DataRow, data_columns: &[DataColumn]) -> Self {
        sauron::log!("data_rows: {}", data_rows.len());
        sauron::log!("data_columns: {}", data_columns.len());
        RowView {
            fields: data_rows
                .into_iter()
                .zip(data_columns.iter())
                .map(|(value, column)| Rc::new(RefCell::new(FieldView::new(value, column))))
                .collect(),
            frozen_fields: vec![],
            is_frozen: false,
        }
    }

    pub fn freeze_columns(&mut self, columns: Vec<usize>) {
        sauron::log!("row view freeze columns: {:?}", columns);
        self.frozen_fields = columns;
        self.update_frozen_column_fields();
    }

    fn view_with_filter<F>(&self, filter: F) -> Node<Msg>
    where
        F: Fn(&(usize, &Rc<RefCell<FieldView>>)) -> bool,
    {
        li(
            [
                class("row"),
                classes_flag([("frozen_row", self.is_frozen)]),
                styles([("height", px(Self::row_height()))]),
                onclick(|_| Msg::Click),
                ondblclick(|_| Msg::DoubleClick),
            ],
            self.fields
                .iter()
                .enumerate()
                .filter(filter)
                .map(|(index, field)| {
                    field
                        .borrow()
                        .view()
                        .map(move |field_msg| Msg::FieldMsg(index, field_msg))
                })
                .collect::<Vec<Node<Msg>>>(),
        )
    }

    /// frozen columns
    pub fn view_frozen_columns(&self) -> Node<Msg> {
        li(
            [
                class("row"),
                class("frozen_column"),
                classes_flag([("frozen_row", self.is_frozen)]),
                styles([("height", px(Self::row_height()))]),
                onclick(|_| Msg::Click),
                ondblclick(|_| Msg::DoubleClick),
            ],
            self.fields
                .iter()
                .enumerate()
                .filter(|(index, _field)| self.frozen_fields.contains(index))
                .map(|(index, field)| {
                    field
                        .borrow()
                        .view()
                        .map(move |field_msg| Msg::FieldMsg(index, field_msg))
                })
                .collect::<Vec<Node<Msg>>>(),
        )
    }

    pub fn row_height() -> i32 {
        30
    }

    pub fn set_is_frozen(&mut self, is_frozen: bool) {
        self.is_frozen = is_frozen;
        self.update_frozen_row_fields();
    }

    pub fn update_frozen_row_fields(&mut self) {
        self.fields
            .iter()
            .for_each(|field| field.borrow_mut().set_is_frozen_row(self.is_frozen))
    }

    pub fn update_frozen_column_fields(&mut self) {
        self.fields.iter().enumerate().for_each(|(index, field)| {
            if self.frozen_fields.contains(&index) {
                field.borrow_mut().set_is_frozen_column(true)
            } else {
                field.borrow_mut().set_is_frozen_column(false)
            }
        })
    }
}

impl Component<Msg> for RowView {
    fn update(&mut self, _msg: Msg) {}

    fn view(&self) -> Node<Msg> {
        self.view_with_filter(|(index, _field)| !self.frozen_fields.contains(index))
    }
}
