pub use field_view::FieldView;
use sauron::{
    html::{attributes::*, events::*, *},
    Component, Node,
};
pub use tab_view::TabView;
pub use window_view::WindowView;

mod field_view;
mod tab_view;
mod window_view;

#[derive(Debug, Clone)]
pub enum Msg {
    DataWindowMsg(window_view::Msg),
}

pub struct App {
    window_view: WindowView,
}

impl App {
    pub fn new() -> App {
        App {
            window_view: WindowView::new(),
        }
    }
}

impl Component<Msg> for App {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::DataWindowMsg(dw_msg) => self.window_view.update(dw_msg),
        }
    }

    fn view(&self) -> Node<Msg> {
        div(
            [styles([("display", "flex"), ("flex-direction", "column")])],
            [
                h1([], [text("Diwata")]),
                textarea([rows(5), cols(200), placeholder("SELECT * ")], []),
                div([], [self.window_view.view().map(Msg::DataWindowMsg)]),
            ],
        )
    }
}
