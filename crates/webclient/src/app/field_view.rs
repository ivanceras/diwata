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
    /// is part of a frozen row, serves no
    /// other purposed other than coloring in css style
    is_frozen_row: bool,
    /// is part of a frozen column, serves no
    /// other puposed other than coloring in css style
    is_frozen_column: bool,
}

impl FieldView {
    pub fn new(value: Value) -> Self {
        FieldView {
            value,
            is_frozen_row: false,
            is_frozen_column: false,
        }
    }

    pub fn set_is_frozen_row(&mut self, frozen: bool) {
        self.is_frozen_row = frozen;
    }

    pub fn set_is_frozen_column(&mut self, frozen: bool) {
        self.is_frozen_column = frozen;
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
            [
                class("field_view"),
                classes_flag([
                    ("frozen_row", self.is_frozen_row),
                    ("frozen_column", self.is_frozen_column),
                ]),
            ],
            [input(
                [
                    r#type("text"),
                    class("value"),
                    classes_flag([
                        ("frozen_row", self.is_frozen_row),
                        ("frozen_column", self.is_frozen_column),
                    ]),
                    onchange(|input| Msg::ChangeValue(input.value)),
                    value(format!("{:?}", self.value)),
                ],
                [],
            )],
        )
    }
}
