pub use datawindow::DataWindow;
pub use field::Field;
use sauron::{
    html::{attributes::*, events::*, *},
    Component, Node,
};
pub use tab::Tab;

mod datawindow;
mod field;
mod tab;

#[derive(Debug, Clone)]
pub enum Msg {
    Click,
    DataWindowMsg(datawindow::Msg),
}

pub struct App {
    datawindow: DataWindow,
    click_count: u32,
}

impl App {
    pub fn new() -> App {
        App {
            datawindow: DataWindow::new(),
            click_count: 0,
        }
    }
}

impl Component<Msg> for App {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Click => self.click_count += 1,
            Msg::DataWindowMsg(dw_msg) => self.datawindow.update(dw_msg),
        }
    }

    fn view(&self) -> Node<Msg> {
        div(
            [styles([("display", "flex"), ("flex-direction", "column")])],
            [
                h1([], [text("Diwata")]),
                textarea([rows(5), cols(200), placeholder("SELECT * ")], []),
                button(
                    [onclick(move |_| Msg::Click)],
                    [text(format!("Clicked {}", self.click_count))],
                ),
                div([], [self.datawindow.view().map(Msg::DataWindowMsg)]),
            ],
        )
    }
}
