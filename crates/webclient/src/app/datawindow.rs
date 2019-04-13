use browser::html::attributes::*;
use browser::html::*;
use vdom::View;

use crate::app::Tab;

pub struct DataWindow {
    tab: Tab,
}

impl DataWindow {
    pub fn new() -> Self {
        DataWindow { tab: Tab::new() }
    }
}

impl View for DataWindow {
    fn view(&self) -> vdom::Node {
        div(
            [],
            [
                div(
                    [],
                    [textarea([rows(5), cols(200), placeholder("SELECT * ")], [])],
                ),
                div([], [self.tab.view()]),
            ],
        )
    }
}
