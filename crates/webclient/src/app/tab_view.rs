use crate::app::{field_view, FieldView};
use diwata_intel::Tab;
use sauron::{
    html::{attributes::*, events::*, *},
    Component, Node,
};

#[derive(Debug, Clone)]
pub enum Msg {
    FieldMsg(usize, field_view::Msg),
}

pub struct TabView {
    fields: Vec<FieldView>,
}

impl TabView {
    pub fn new(tab: Tab) -> Self {
        TabView {
            fields: tab.fields.into_iter().map(FieldView::new).collect(),
        }
    }
}

impl Component<Msg> for TabView {
    fn update(&mut self, msg: Msg) {}
    fn view(&self) -> Node<Msg> {
        div([], [div([], [])])
    }
}
