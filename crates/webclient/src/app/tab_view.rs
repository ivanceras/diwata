use crate::app::{field_view, FieldView};
use sauron::{
    html::{attributes::*, events::*, *},
    Component, Node,
};

#[derive(Debug, Clone)]
pub enum Msg {
    FieldMsg(usize, field_view::Msg),
}

pub struct TabView {
    field_views: Vec<FieldView>,
}

impl TabView {
    pub fn new() -> Self {
        TabView {
            field_views: vec![FieldView::new(), FieldView::new(), FieldView::new()],
        }
    }
}

impl Component<Msg> for TabView {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::FieldMsg(index, field_msg) => {
                self.field_views[index].update(field_msg.clone());
            }
        }
    }
    fn view(&self) -> Node<Msg> {
        div(
            [],
            [div(
                [],
                self.field_views
                    .iter()
                    .enumerate()
                    .map(|(index, field_view)| {
                        field_view
                            .view()
                            .map(move |view| Msg::FieldMsg(index, view))
                    })
                    .collect::<Vec<Node<Msg>>>(),
            )],
        )
    }
}
