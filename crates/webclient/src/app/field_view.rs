use sauron::{
    html::{attributes::*, events::*, *},
    Component, Node,
};

use diwata_intel::Field;

#[derive(Debug, Clone)]
pub enum Msg {
    FieldClick,
}

pub struct FieldView {
    click_count: u32,
}

impl FieldView {
    pub fn new(field: Field) -> Self {
        FieldView { click_count: 0 }
    }
}

impl Component<Msg> for FieldView {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::FieldClick => self.click_count += 1,
        }
    }
    fn view(&self) -> Node<Msg> {
        button(
            [class("btn"), onclick(|_| Msg::FieldClick)],
            [text(format!("this is field {}", self.click_count))],
        )
    }
}
