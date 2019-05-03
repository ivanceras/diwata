use crate::app::field_view::{self, FieldView};

use data_table::DataRow;
use sauron::{
    html::{attributes::*, events::*, *},
    Component, Node,
};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
pub enum Msg {
    Close,
    FieldMsg(usize, field_view::Msg),
}

/// When a record from the main tab is clicked, it will show the detailed view of that
/// row, displaying only that 1 row, and it's related content
/// such as one_one tab, has_many and indirect tab
pub struct DetailView {
    //TODO: use a reference to the field view
    // instead of cloning
    fields: Vec<Rc<RefCell<FieldView>>>,
    pub is_visible: bool,
    pub row_index: Option<usize>,
}

impl DetailView {
    pub fn new() -> Self {
        DetailView {
            fields: vec![],
            is_visible: false,
            row_index: None,
        }
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
    }

    pub fn show(&mut self) {
        self.is_visible = true;
    }

    pub fn set_row(&mut self, row_index: usize) {
        self.row_index = Some(row_index);
    }

    pub fn set_fields(&mut self, fields: &Vec<Rc<RefCell<FieldView>>>) {
        self.fields = fields.clone();
    }
}

impl Component<Msg> for DetailView {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::FieldMsg(index, field_msg) => {
                self.fields[index].borrow_mut().update(field_msg);
            }
            Msg::Close => {
                sauron::log("Closing, intercepted in parent views");
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        main(
            [
                class("detail_view"),
                styles_flag([("display", "none", !self.is_visible)]),
            ],
            [
                text(format!("Detailed view {:?}", self.row_index)),
                section(
                    [],
                    self.fields
                        .iter()
                        .enumerate()
                        .map(|(index, field)| {
                            field
                                .borrow()
                                .view()
                                .map(move |field_msg| Msg::FieldMsg(index, field_msg))
                        })
                        .collect::<Vec<Node<Msg>>>(),
                ),
                button([onclick(|_| Msg::Close)], [text("Close")]),
            ],
        )
    }
}
