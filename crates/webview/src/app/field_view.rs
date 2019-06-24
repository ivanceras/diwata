use data_table::{DataColumn, Type};
use diwata_intel::{Array, Value};
use sauron::{
    html::{attributes::*, events::*},
    Cmd, Component, Node,
    html_array::*,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Msg {
    TextChange(String),
    PrimaryClicked,
}

#[derive(Clone)]
pub struct FieldView {
    pub column: DataColumn,
    pub value: Value,
    pub new_value: Value,
    /// is part of a frozen row, serves no
    /// other purposed other than coloring in css style
    pub is_frozen_row: bool,
    /// is part of a frozen column, serves no
    /// other puposed other than coloring in css style
    pub is_frozen_column: bool,
}

impl FieldView {
    pub fn new(value: &Value, column: &DataColumn) -> Self {
        FieldView {
            new_value: value.clone(),
            value: value.clone(),
            column: column.clone(),
            is_frozen_row: false,
            is_frozen_column: false,
        }
    }

    pub fn is_immovable(&self) -> bool {
        self.is_frozen_row && self.is_frozen_column
    }

    pub fn is_normal_field(&self) -> bool {
        !self.is_frozen_row && !self.is_frozen_column
    }

    pub fn is_changed(&self) -> bool {
        self.value != self.new_value
    }

    pub fn set_is_frozen_row(&mut self, frozen: bool) {
        self.is_frozen_row = frozen;
    }

    pub fn set_is_frozen_column(&mut self, frozen: bool) {
        self.is_frozen_column = frozen;
    }

    fn view_value_as_primary(&self) -> Node<Msg> {
        let classes = classes_flag([
            ("value", true),
            ("frozen_row", self.is_frozen_row),
            ("frozen_column", self.is_frozen_column),
        ]);
        match &self.value {
            Value::Int(v) => a(
                [
                    classes,
                    onclick(|_| Msg::PrimaryClicked),
                    href(format!("#{}", v)),
                ],
                [text(v.to_string())],
            ),
            _ => {
                sauron::log!("todo primary: {:?}", self.value);
                text("unknown")
            }
        }
    }

    fn view_value(&self) -> Node<Msg> {
        let classes = classes_flag([
            ("value", true),
            ("frozen_row", self.is_frozen_row),
            ("frozen_column", self.is_frozen_column),
            ("modified", self.is_changed()),
        ]);
        match &self.value {
            Value::Nil => match self.column.data_type {
                Type::Bool => input([r#type("checkbox"), classes], []),
                _ => input([r#type("text"), classes, value("")], []),
            },
            Value::Text(v) => input(
                [
                    r#type("text"),
                    classes,
                    value(v),
                    onchange(|input| Msg::TextChange(input.value)),
                ],
                [],
            ),
            Value::Char(v) => input(
                [
                    r#type("text"),
                    classes,
                    value(v.to_string()),
                    onchange(|input| Msg::TextChange(input.value)),
                ],
                [],
            ),
            Value::Json(v) => input(
                [
                    r#type("text"),
                    classes,
                    value(v),
                    onchange(|input| Msg::TextChange(input.value)),
                ],
                [],
            ),
            Value::Uuid(v) => input(
                [
                    r#type("text"),
                    classes,
                    value(v.to_string()),
                    onchange(|input| Msg::TextChange(input.value)),
                ],
                [],
            ),
            Value::Bool(_v) => input([r#type("checkbox"), classes], []),
            Value::Tinyint(v) => input(
                [
                    r#type("number"),
                    classes,
                    value(v.to_string()),
                    onchange(|input| Msg::TextChange(input.value)),
                ],
                [],
            ),
            Value::Smallint(v) => input(
                [
                    r#type("number"),
                    classes,
                    value(v.to_string()),
                    onchange(|input| Msg::TextChange(input.value)),
                ],
                [],
            ),
            Value::Int(v) => input(
                [
                    r#type("number"),
                    classes,
                    value(v.to_string()),
                    onchange(|input| Msg::TextChange(input.value)),
                ],
                [],
            ),
            Value::Bigint(v) => input(
                [
                    r#type("number"),
                    classes,
                    value(v.to_string()),
                    onchange(|input| Msg::TextChange(input.value)),
                ],
                [],
            ),
            Value::Float(v) => input(
                [
                    r#type("number"),
                    classes,
                    value(v.to_string()),
                    onchange(|input| Msg::TextChange(input.value)),
                ],
                [],
            ),
            Value::Double(v) => input(
                [
                    r#type("number"),
                    classes,
                    value(v.to_string()),
                    onchange(|input| Msg::TextChange(input.value)),
                ],
                [],
            ),
            Value::BigDecimal(v) => input(
                [
                    r#type("number"),
                    classes,
                    value(v.to_string()),
                    onchange(|input| Msg::TextChange(input.value)),
                ],
                [],
            ),
            Value::ImageUri(v) => img([src(v), classes], []),
            Value::Array(Array::Text(v)) => input(
                [
                    r#type("text"),
                    classes,
                    value(v.join(",")),
                    onchange(|input| Msg::TextChange(input.value)),
                ],
                [],
            ),
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
                    onchange(|input| Msg::TextChange(input.value)),
                ],
                [],
            ),
            Value::Timestamp(v) => input(
                [
                    r#type("date"),
                    classes,
                    value(v.format("%Y-%m-%d").to_string()),
                    onchange(|input| Msg::TextChange(input.value)),
                ],
                [],
            ),
            Value::Date(v) => input(
                [
                    r#type("date"),
                    classes,
                    value(v.format("%Y-%m-%d").to_string()),
                    onchange(|input| Msg::TextChange(input.value)),
                ],
                [],
            ),
            Value::DateTime(v) => input(
                [
                    r#type("date"),
                    classes,
                    value(v.format("%Y-%m-%d").to_string()),
                    onchange(|input| Msg::TextChange(input.value)),
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
                if self.column.is_primary {
                    self.view_value_as_primary()
                } else {
                    self.view_value()
                },
            ],
        )
    }
}

impl Component<Msg> for FieldView {
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        sauron::log!("field updated: {:?}", msg);
        match msg {
            Msg::TextChange(value) => {
                self.new_value = Value::Text(value);
                Cmd::none()
            }
            Msg::PrimaryClicked => {
                sauron::log!("Primary clicked");
                Cmd::none()
            }
        }
    }
    /// when viewed as row
    fn view(&self) -> Node<Msg> {
        div(
            [
                class("field_view"),
                //styles([("width", px(200))]),
                classes_flag([
                    ("frozen_row", self.is_frozen_row),
                    ("frozen_column", self.is_frozen_column),
                ]),
            ],
            [self.view_value()],
        )
    }
}
