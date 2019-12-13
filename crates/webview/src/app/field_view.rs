use data_table::{DataColumn, Type};
use diwata_intel::{Array, Value};
use sauron::{
    html::{attributes::*, events::*, *},
    Cmd, Component, Node,
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
        let classes = classes_flag(vec![
            ("value", true),
            ("frozen_row", self.is_frozen_row),
            ("frozen_column", self.is_frozen_column),
        ]);
        match &self.value {
            Value::Int(v) => a(
                vec![
                    classes,
                    onclick(|_| Msg::PrimaryClicked),
                    href(format!("#{}", v)),
                ],
                vec![text(v.to_string())],
            ),
            _ => {
                trace!("todo primary: {:?}", self.value);
                text("unknown")
            }
        }
    }

    fn view_value(&self) -> Node<Msg> {
        let classes = classes_flag(vec![
            ("value", true),
            ("frozen_row", self.is_frozen_row),
            ("frozen_column", self.is_frozen_column),
            ("modified", self.is_changed()),
        ]);
        match &self.value {
            Value::Nil => match self.column.data_type {
                Type::Bool => input(vec![r#type("checkbox"), classes], vec![]),
                _ => input(vec![r#type("text"), classes, value("")], vec![]),
            },
            Value::Text(v) => input(
                vec![
                    r#type("text"),
                    classes,
                    value(v),
                    onchange(|input| Msg::TextChange(input.value)),
                ],
                vec![],
            ),
            Value::Char(v) => input(
                vec![
                    r#type("text"),
                    classes,
                    value(v.to_string()),
                    onchange(|input| Msg::TextChange(input.value)),
                ],
                vec![],
            ),
            Value::Json(v) => input(
                vec![
                    r#type("text"),
                    classes,
                    value(v),
                    onchange(|input| Msg::TextChange(input.value)),
                ],
                vec![],
            ),
            Value::Uuid(v) => input(
                vec![
                    r#type("text"),
                    classes,
                    value(v.to_string()),
                    onchange(|input| Msg::TextChange(input.value)),
                ],
                vec![],
            ),
            Value::Bool(_v) => input(vec![r#type("checkbox"), classes], vec![]),
            Value::Tinyint(v) => input(
                vec![
                    r#type("number"),
                    classes,
                    value(v.to_string()),
                    onchange(|input| Msg::TextChange(input.value)),
                ],
                vec![],
            ),
            Value::Smallint(v) => input(
                vec![
                    r#type("number"),
                    classes,
                    value(v.to_string()),
                    onchange(|input| Msg::TextChange(input.value)),
                ],
                vec![],
            ),
            Value::Int(v) => input(
                vec![
                    r#type("number"),
                    classes,
                    value(v.to_string()),
                    onchange(|input| Msg::TextChange(input.value)),
                ],
                vec![],
            ),
            Value::Bigint(v) => input(
                vec![
                    r#type("number"),
                    classes,
                    value(v.to_string()),
                    onchange(|input| Msg::TextChange(input.value)),
                ],
                vec![],
            ),
            Value::Float(v) => input(
                vec![
                    r#type("number"),
                    classes,
                    value(v.to_string()),
                    onchange(|input| Msg::TextChange(input.value)),
                ],
                vec![],
            ),
            Value::Double(v) => input(
                vec![
                    r#type("number"),
                    classes,
                    value(v.to_string()),
                    onchange(|input| Msg::TextChange(input.value)),
                ],
                vec![],
            ),
            Value::BigDecimal(v) => input(
                vec![
                    r#type("number"),
                    classes,
                    value(v.to_string()),
                    onchange(|input| Msg::TextChange(input.value)),
                ],
                vec![],
            ),
            Value::ImageUri(v) => img(vec![src(v), classes], vec![]),
            Value::Array(Array::Text(v)) => input(
                vec![
                    r#type("text"),
                    classes,
                    value(v.join(",")),
                    onchange(|input| Msg::TextChange(input.value)),
                ],
                vec![],
            ),
            Value::Array(Array::Float(v)) => input(
                vec![
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
                vec![],
            ),
            Value::Timestamp(v) => input(
                vec![
                    r#type("date"),
                    classes,
                    value(v.format("%Y-%m-%d").to_string()),
                    onchange(|input| Msg::TextChange(input.value)),
                ],
                vec![],
            ),
            Value::Date(v) => input(
                vec![
                    r#type("date"),
                    classes,
                    value(v.format("%Y-%m-%d").to_string()),
                    onchange(|input| Msg::TextChange(input.value)),
                ],
                vec![],
            ),
            Value::DateTime(v) => input(
                vec![
                    r#type("date"),
                    classes,
                    value(v.format("%Y-%m-%d").to_string()),
                    onchange(|input| Msg::TextChange(input.value)),
                ],
                vec![],
            ),
            _ => {
                trace!("todo for: {:?}", self.value);
                text("unknown")
            }
        }
    }

    pub fn view_in_detail(&self) -> Node<Msg> {
        div(
            vec![
                class("field_view in_detail"),
                classes_flag(vec![
                    ("frozen_row", self.is_frozen_row),
                    ("frozen_column", self.is_frozen_column),
                ]),
            ],
            vec![
                label(
                    vec![class("in_detail_column")],
                    vec![text(&self.column.name)],
                ),
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
        trace!("field updated: {:?}", msg);
        match msg {
            Msg::TextChange(value) => {
                self.new_value = Value::Text(value);
                Cmd::none()
            }
            Msg::PrimaryClicked => {
                trace!("Primary clicked");
                Cmd::none()
            }
        }
    }
    /// when viewed as row
    fn view(&self) -> Node<Msg> {
        div(
            vec![
                class("field_view"),
                //styles(vec![("width", px(200))]),
                classes_flag(vec![
                    ("frozen_row", self.is_frozen_row),
                    ("frozen_column", self.is_frozen_column),
                ]),
            ],
            vec![self.view_value()],
        )
    }
}
