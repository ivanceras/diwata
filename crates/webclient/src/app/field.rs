use sauron::html::attributes::*;
use sauron::html::*;
use sauron::Component;
use crate::Node;

use crate::app::Msg;

pub struct Field {}

impl Field {
    pub fn new() -> Self {
        Field {}
    }
}

impl Component<Msg> for Field {
    fn update(&mut self, msg: Msg){
    }
    fn view(&self) -> Node {
        div([], [text("this is field")])
    }
}
