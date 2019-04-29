use sauron::{
    html::{attributes::*, events::*, *},
    Component, Node,
};

use crate::app::{tab_view, TabView};

#[derive(Debug, Clone)]
pub enum Msg {
    TabMsg(tab_view::Msg),
}

pub struct WindowView {
    tab_view: TabView,
}

impl WindowView {
    pub fn new() -> Self {
        WindowView {
            tab_view: TabView::new(),
        }
    }
}

impl Component<Msg> for WindowView {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::TabMsg(tab_msg) => self.tab_view.update(tab_msg),
        }
    }
    fn view(&self) -> Node<Msg> {
        div(
            [class("datawindow")],
            [div([], [self.tab_view.view().map(Msg::TabMsg)])],
        )
    }
}
