use sauron::{
    html::{attributes::*, events::*, *},
    Component, Node,
};

use data_table::Value;
use diwata_intel::Field;

#[derive(Debug, Clone)]
pub enum Msg {
    FieldClick,
}

pub struct FieldView {
    value: Value,
}

impl FieldView {
    pub fn new(value: Value) -> Self {
        FieldView { value }
    }
}

impl Component<Msg> for FieldView {
    fn update(&mut self, msg: Msg) {}
    fn view(&self) -> Node<Msg> {
        button([class("btn")], [text("this is field {}")])
    }
}
