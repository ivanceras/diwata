use sauron::html::attributes::*;
use sauron::html::*;
use sauron::Component;
use crate::app::Tab;
use crate::app::Msg;
use crate::Node;

pub struct DataWindow {
    tab: Tab,
}

impl DataWindow {
    pub fn new() -> Self {
        DataWindow { tab: Tab::new() }
    }
}

impl Component<Msg> for DataWindow {
    fn update(&mut self, msg: Msg){
    }
    fn view(&self) -> Node {
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
