use sauron::{
    html::{attributes::*, *},
    Component, Node,
};

use data_table::Value;

#[derive(Clone)]
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
    fn update(&mut self, _msg: Msg) {}
    fn view(&self) -> Node<Msg> {
        input(
            [
                r#type("text"),
                class("value"),
                value(format!("{:?}", self.value)),
            ],
            [],
        )
    }
}
