use sauron::{
    html::{attributes::*, events::*, *},
    Component, Node,
};

use data_table::Value;

#[derive(Clone)]
pub enum Msg {
    ChangeValue(String),
}

#[derive(Clone)]
pub struct FieldView {
    value: Value,
}

impl FieldView {
    pub fn new(value: Value) -> Self {
        FieldView { value }
    }
}

impl Component<Msg> for FieldView {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::ChangeValue(value) => {
                self.value = Value::Text(value);
            }
        }
    }
    fn view(&self) -> Node<Msg> {
        div(
            [class("field_view")],
            [input(
                [
                    r#type("text"),
                    class("value"),
                    onchange(|input| Msg::ChangeValue(input.value)),
                    value(format!("{:?}", self.value)),
                ],
                [],
            )],
        )
    }
}
