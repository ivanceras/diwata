use crate::app::Field;
use browser::html::attributes::*;
use browser::html::*;
use vdom::View;

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

impl View for Tab {
    fn view(&self) -> vdom::Node {
        div(
            [],
            [
                text("this is tab"),
                div(
                    [],
                    self.fields
                        .iter()
                        .map(Field::view)
                        .collect::<Vec<vdom::Node>>(),
                ),
            ],
        )
    }
}
