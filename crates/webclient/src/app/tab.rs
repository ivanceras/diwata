use crate::app::Field;
use sauron::html::attributes::*;
use sauron::html::*;
use sauron::Component;
use crate::Node;


use crate::app::Msg;

pub struct Tab {
    fields: Vec<Field>,
}

impl Tab {
    pub fn new() -> Self {
        Tab {
            fields: vec![Field {}, Field {}, Field {}],
        }
    }
}

impl Component<Msg> for Tab {
    fn update(&mut self, msg: Msg){
    }
    fn view(&self) -> Node {
        div(
            [],
            [
                text("this is tab"),
                div(
                    [],
                    self.fields
                        .iter()
                        .map(Field::view)
                        .collect::<Vec<Node>>(),
                ),
            ],
        )
    }
}
