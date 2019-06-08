use crate::app::{
    self,
    field_view::{self, FieldView},
};
use data_table::DataColumn;
use diwata_intel::{Dao, DataRow};
use sauron::{
    html::{attributes::*, events::*, units::*, *},
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
    index: usize,
    pub fields: Vec<Rc<RefCell<FieldView>>>,
    frozen_fields: Vec<usize>,
    is_frozen: bool,
}

impl RowView {
    pub fn new(index: usize, data_rows: &DataRow, data_columns: &[DataColumn]) -> Self {
        RowView {
            index,
            fields: data_rows
                .into_iter()
                .zip(data_columns.iter())
                .map(|(value, column)| Rc::new(RefCell::new(FieldView::new(value, column))))
                .collect(),
            frozen_fields: vec![],
            is_frozen: false,
        }
    }

    /// is value of any field modified
    pub fn is_changed(&self) -> bool {
        self.fields.iter().any(|field| field.borrow().is_changed())
    }

    /// return the primary columns value pair
    pub fn primary_dao(&self) -> Dao {
        self.fields
            .iter()
            .filter(|field| field.borrow().column.is_primary)
            .fold(Dao::new(), |mut dao, field_view| {
                let field = field_view.borrow();
                let column = &field.column.name;
                let value = &field.value;
                dao.insert(column, value.clone());
                dao
            })
    }

    pub fn freeze_columns(&mut self, columns: Vec<usize>) {
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
                // IMPORTANT: key is needed here to avoid sauron
                // reusing dom elements of similar rows, this is needed
                // so as to complete remove the dom and it's attached events
                // since events attached in a dom are not compared
                // and is not replaced.
                key(format!("row_{}", self.index)),
                classes_flag([
                    ("frozen_row", self.is_frozen),
                    ("modified", self.is_changed()),
                ]),
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
                // IMPORTANT: key is needed here to avoid sauron
                // reusing dom elements of similar rows, this is needed
                // so as to complete remove the dom and it's attached events
                // since events attached in a dom are not compared
                // and is not replaced.
                key(format!("row_{}", self.index)),
                class("frozen_column"), // The only difference in view_with_filter
                classes_flag([
                    ("frozen_row", self.is_frozen),
                    ("modified", self.is_changed()),
                ]),
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

    pub fn update(&mut self, msg: Msg) -> app::Cmd {
        match msg {
            Msg::FieldMsg(field_index, field_msg) => {
                self.fields[field_index].borrow_mut().update(field_msg);
                app::Cmd::none()
            }
            Msg::DoubleClick => app::Cmd::none(),
            Msg::Click => app::Cmd::none(),
        }
    }

    pub fn view(&self) -> Node<Msg> {
        self.view_with_filter(|(index, _field)| !self.frozen_fields.contains(index))
    }
}
