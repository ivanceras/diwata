use browser::html::attributes::*;
use browser::html::*;
use vdom::View;

pub struct Field {}

impl Field {
    pub fn new() -> Self {
        Field {}
    }
}

impl View for Field {
    fn view(&self) -> vdom::Node {
        div([], [text("this is field")])
    }
}
