use diwata_intel::Window;
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
    window_view: Vec<WindowView>,
}

impl App {
    pub fn new(windows: Vec<Window>) -> App {
        App {
            window_view: windows.into_iter().map(WindowView::new).collect(),
        }
    }
}

impl Component<Msg> for App {
    fn update(&mut self, msg: Msg) {}

    fn view(&self) -> Node<Msg> {
        div(
            [styles([("display", "flex"), ("flex-direction", "column")])],
            [
                h1([], [text("Diwata")]),
                textarea([rows(5), cols(200), placeholder("SELECT * ")], []),
            ],
        )
    }
}
