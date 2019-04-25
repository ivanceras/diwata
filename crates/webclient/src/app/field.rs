use crate::Node;
use sauron::{
    html::{attributes::*, events::*, *},
    Component,
};

#[derive(Debug, Clone)]
pub enum Msg {
    FieldClick,
}

pub struct Field {
    click_count: u32,
}

impl Field {
    pub fn new() -> Self {
        Field { click_count: 0 }
    }
}

impl Component<Msg> for Field {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::FieldClick => self.click_count += 1,
        }
    }
    fn view(&self) -> Node<Msg> {
        button(
            [onclick(|_| Msg::FieldClick)],
            [text(format!("this is field {}", self.click_count))],
        )
    }
}
