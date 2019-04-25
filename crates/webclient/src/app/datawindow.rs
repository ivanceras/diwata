use sauron::{
    html::{attributes::*, events::*, *},
    Component, Node,
};

use crate::app::{tab, Tab};

#[derive(Debug, Clone)]
pub enum Msg {
    TabMsg(tab::Msg),
    DataWindowClicked,
}

pub struct DataWindow {
    tab: Tab,
    click_count: u32,
}

impl DataWindow {
    pub fn new() -> Self {
        DataWindow {
            tab: Tab::new(),
            click_count: 0,
        }
    }
}

impl Component<Msg> for DataWindow {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::DataWindowClicked => self.click_count += 1,
            Msg::TabMsg(tab_msg) => self.tab.update(tab_msg),
        }
    }
    fn view(&self) -> Node<Msg> {
        div(
            [],
            [
                div(
                    [],
                    [
                        textarea([rows(5), cols(200), placeholder("SELECT * ")], []),
                        button(
                            [onclick(|_| Msg::DataWindowClicked)],
                            [text(format!("Data window here {}", self.click_count))],
                        ),
                    ],
                ),
                div([], [self.tab.view().map(Msg::TabMsg)]),
            ],
        )
    }
}
