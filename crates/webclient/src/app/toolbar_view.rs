use sauron::{
    html::{attributes::*, events::*, *},
    Component, Node,
};

#[derive(Clone)]
pub enum Msg {
    ToggleShowQuery,
}

pub struct ToolbarView {
    pub show_query: bool,
}

impl ToolbarView {
    pub fn new() -> Self {
        ToolbarView { show_query: true }
    }
}

impl Component<Msg> for ToolbarView {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::ToggleShowQuery => self.show_query = !self.show_query,
        }
    }

    fn view(&self) -> Node<Msg> {
        section(
            [class("toolbar")],
            [
                button([], [text("Create new record")]),
                button([], [text("Insert new record")]),
                button([], [text("Save")]),
                button([], [text("Cancel")]),
                button([], [text("Delete")]),
                button([], [text("Refresh")]),
                button([], [text("Clear filter")]),
                button([], [text("Filter more..")]),
                button([], [text("Sort..")]),
                button([], [text("Export")]),
                button(
                    [onclick(|_| Msg::ToggleShowQuery)],
                    [
                        input([r#type("checkbox")], []).attributes(attrs_flag([(
                            "checked",
                            "checked",
                            self.show_query,
                        )])),
                        text("Show query"),
                    ],
                ),
            ],
        )
    }
}
