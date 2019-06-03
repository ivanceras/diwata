use data_table::{DataColumn, Type};
use diwata_intel::{Array, Value};
use sauron::{
    html::{attributes::*, events::*, *},
    Cmd, Component, Node,
};

#[derive(Debug, Clone)]
pub enum Msg {
    ChangeValue(String),
}

#[derive(Clone)]
pub struct FieldView {
    pub column: DataColumn,
    pub value: Value,
    /// is part of a frozen row, serves no
    /// other purposed other than coloring in css style
    is_frozen_row: bool,
    /// is part of a frozen column, serves no
    /// other puposed other than coloring in css style
    is_frozen_column: bool,
}

impl FieldView {
    pub fn new(value: Value, column: &DataColumn) -> Self {
        FieldView {
            value,
            column: column.clone(),
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

    fn view_value(&self) -> Node<Msg> {
        let classes = classes_flag([
            ("value", true),
            ("frozen_row", self.is_frozen_row),
            ("frozen_column", self.is_frozen_column),
        ]);
        match &self.value {
            Value::Nil => match self.column.data_type {
                Type::Bool => input([r#type("checkbox"), classes], []),
                _ => input([r#type("text"), classes, value("")], []),
            },
            Value::Text(v) => input([r#type("text"), classes, value(v)], []),
            Value::Json(v) => input([r#type("text"), classes, value(v)], []),
            Value::Uuid(v) => input([r#type("text"), classes, value(v.to_string())], []),
            Value::Bool(v) => input([r#type("checkbox"), classes], []),
            Value::Tinyint(v) => input([r#type("number"), classes, value(v.to_string())], []),
            Value::Smallint(v) => input([r#type("number"), classes, value(v.to_string())], []),
            Value::Int(v) => input([r#type("number"), classes, value(v.to_string())], []),
            Value::Bigint(v) => input([r#type("number"), classes, value(v.to_string())], []),
            Value::Float(v) => input([r#type("number"), classes, value(v.to_string())], []),
            Value::Double(v) => input([r#type("number"), classes, value(v.to_string())], []),
            Value::BigDecimal(v) => input([r#type("number"), classes, value(v.to_string())], []),
            Value::ImageUri(v) => img([src(v), classes], []),
            Value::Array(Array::Text(v)) => {
                input([r#type("text"), classes, value(v.join(","))], [])
            }
            Value::Array(Array::Float(v)) => input(
                [
                    r#type("text"),
                    classes,
                    value(
                        v.iter()
                            .map(ToString::to_string)
                            .collect::<Vec<String>>()
                            .join(","),
                    ),
                ],
                [],
            ),
            Value::Timestamp(v) => input(
                [
                    r#type("date"),
                    classes,
                    value(v.format("%Y-%m-%d").to_string()),
                ],
                [],
            ),
            Value::Date(v) => input(
                [
                    r#type("date"),
                    classes,
                    value(v.format("%Y-%m-%d").to_string()),
                ],
                [],
            ),
            _ => {
                sauron::log!("todo for: {:?}", self.value);
                text("unknown")
            }
        }
    }

    pub fn view_in_detail(&self) -> Node<Msg> {
        div(
            [
                class("field_view in_detail"),
                classes_flag([
                    ("frozen_row", self.is_frozen_row),
                    ("frozen_column", self.is_frozen_column),
                ]),
            ],
            [
                label([class("in_detail_column")], [text(&self.column.name)]),
                self.view_value(),
            ],
        )
    }
}

impl Component<Msg> for FieldView {
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::ChangeValue(value) => {
                self.value = Value::Text(value);
                Cmd::none()
            }
        }
    }
    /// when viewed as row
    fn view(&self) -> Node<Msg> {
        div(
            [
                class("field_view"),
                classes_flag([
                    ("frozen_row", self.is_frozen_row),
                    ("frozen_column", self.is_frozen_column),
                ]),
            ],
            [self.view_value()],
        )
    }
}
