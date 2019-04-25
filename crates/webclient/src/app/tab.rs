use crate::app::{field, Field};
use sauron::{
    html::{attributes::*, events::*, *},
    Component, Node,
};

#[derive(Debug, Clone)]
pub enum Msg {
    TabClick,
    FieldMsg(usize, field::Msg),
}

pub struct Tab {
    fields: Vec<Field>,
    click_count: u32,
}

impl Tab {
    pub fn new() -> Self {
        Tab {
            fields: vec![Field::new(), Field::new(), Field::new()],
            click_count: 0,
        }
    }
}

impl Component<Msg> for Tab {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::TabClick => self.click_count += 1,
            Msg::FieldMsg(index, field_msg) => {
                self.fields[index].update(field_msg.clone());
            }
        }
    }
    fn view(&self) -> Node<Msg> {
        div(
            [],
            [
                button(
                    [class("btn"), onclick(|_| Msg::TabClick)],
                    [text(format!("this is tab {}", self.click_count))],
                ),
                div(
                    [],
                    self.fields
                        .iter()
                        .enumerate()
                        .map(|(index, field)| {
                            field.view().map(move |view| Msg::FieldMsg(index, view))
                        })
                        .collect::<Vec<Node<Msg>>>(),
                ),
            ],
        )
    }
}
